use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use serde_json::{json, Value};

use crate::model::{DaemonEvent, DownloadRow, Item, LyricLine, SourceInfo};

pub trait Backend {
    fn command(&mut self, name: &str, payload: Value);
    fn poll(&mut self) -> Vec<DaemonEvent>;
    fn close(&mut self) {}
}

pub struct MemoryBackend {
    pub commands: Vec<(String, Value)>,
    events: Vec<DaemonEvent>,
}

impl MemoryBackend {
    pub fn new() -> Self {
        Self { commands: Vec::new(), events: Vec::new() }
    }

    pub fn push(&mut self, event: DaemonEvent) {
        self.events.push(event);
    }
}

impl Default for MemoryBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for MemoryBackend {
    fn command(&mut self, name: &str, payload: Value) {
        self.commands.push((name.to_owned(), payload));
    }

    fn poll(&mut self) -> Vec<DaemonEvent> {
        std::mem::take(&mut self.events)
    }
}

pub struct BridgeBackend {
    child: Option<Child>,
    lines: Arc<Mutex<Vec<String>>>,
}

impl BridgeBackend {
    pub fn launch() -> Self {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut candidates = Vec::new();
        if let Some(custom) = std::env::var_os("MINERAL_GUI_BRIDGE") {
            candidates.push(PathBuf::from(custom));
        }
        if let Ok(exe) = std::env::current_exe()
            && let Some(dir) = exe.parent()
        {
            candidates.push(dir.join("mineral-gui-bridge"));
            candidates.push(dir.join("mineral-gui-bridge.exe"));
        }
        candidates.push(manifest_dir.join("../../target/release/mineral-gui-bridge"));
        candidates.push(manifest_dir.join("../../target/release/mineral-gui-bridge.exe"));
        candidates.push(manifest_dir.join("../../target/debug/mineral-gui-bridge"));
        candidates.push(manifest_dir.join("../../target/debug/mineral-gui-bridge.exe"));
        candidates.push(manifest_dir.join("../../bridge/target/release/mineral-gui-bridge"));
        candidates.push(manifest_dir.join("../../bridge/target/debug/mineral-gui-bridge"));

        let best = candidates
            .iter()
            .filter(|path| path.is_file())
            .max_by_key(|path| {
                path.metadata()
                    .and_then(|meta| meta.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            })
            .cloned()
            .unwrap_or_else(|| manifest_dir.join("../../target/release/mineral-gui-bridge"));

        Self::launch_at(best)
    }

    pub fn launch_at(binary: PathBuf) -> Self {
        let lines = Arc::new(Mutex::new(Vec::new()));
        if !binary.is_file() {
            lines.lock().expect("lines").push(
                json!({
                    "event": "connection",
                    "connected": false,
                    "message": format!("未找到桥接程序 {}。请先 cargo build -p mineral-gui-bridge --release", binary.display()),
                })
                .to_string(),
            );
            return Self { child: None, lines };
        }
        let mut command = Command::new(&binary);
        command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null());
        match command.spawn() {
            Ok(mut child) => {
                if let Some(stdout) = child.stdout.take() {
                    let lines = Arc::clone(&lines);
                    thread::spawn(move || {
                        let reader = std::io::BufReader::new(stdout);
                        for line in reader.lines() {
                            let Ok(line) = line else { break };
                            lines.lock().expect("lines").push(line);
                        }
                        lines.lock().expect("lines").push(
                            json!({"event":"connection","connected":false,"message":"Daemon 桥接已退出"}).to_string(),
                        );
                    });
                }
                let backend = Self { child: Some(child), lines };
                let mut backend = backend;
                backend.command("connect", json!({}));
                backend
            }
            Err(error) => {
                lines.lock().expect("lines").push(
                    json!({"event":"connection","connected":false,"message":format!("无法启动桥接：{error}")}).to_string(),
                );
                Self { child: None, lines }
            }
        }
    }
}

impl Backend for BridgeBackend {
    fn command(&mut self, name: &str, payload: Value) {
        let Some(child) = self.child.as_mut() else {
            self.lines.lock().expect("lines").push(
                json!({"event":"notice","level":"error","text":"Daemon 断连，操作未执行"}).to_string(),
            );
            return;
        };
        let Some(stdin) = child.stdin.as_mut() else { return };
        let mut message = payload;
        if let Some(object) = message.as_object_mut() {
            object.insert("cmd".to_owned(), Value::String(name.to_owned()));
        } else {
            message = json!({"cmd": name});
        }
        let _ = writeln!(stdin, "{message}");
        let _ = stdin.flush();
    }

    fn poll(&mut self) -> Vec<DaemonEvent> {
        let lines = std::mem::take(&mut *self.lines.lock().expect("lines"));
        lines
            .iter()
            .filter_map(|line| serde_json::from_str::<Value>(line).ok())
            .filter_map(parse_event)
            .collect()
    }

    fn close(&mut self) {
        if let Some(mut child) = self.child.take() {
            if let Some(stdin) = child.stdin.as_mut() {
                let _ = writeln!(stdin, "{}", json!({"cmd":"quit"}));
                let _ = stdin.flush();
            }
            let _ = child.wait();
        }
    }
}

impl Drop for BridgeBackend {
    fn drop(&mut self) {
        self.close();
    }
}

pub fn parse_event(value: Value) -> Option<DaemonEvent> {
    let kind = value.get("event")?.as_str()?.to_owned();
    Some(match kind.as_str() {
        "connection" => DaemonEvent::Connection {
            connected: value.get("connected").and_then(Value::as_bool).unwrap_or(false),
            message: value.get("message").and_then(Value::as_str).unwrap_or("").to_owned(),
        },
        "sources" => DaemonEvent::Sources {
            sources: serde_json::from_value::<Vec<SourceInfo>>(value.get("sources")?.clone()).unwrap_or_default(),
            message: value.get("message").and_then(Value::as_str).unwrap_or("").to_owned(),
        },
        "player" => {
            let state = value.get("state")?.clone();
            let player = value.get("player").cloned().unwrap_or(json!({}));
            DaemonEvent::Player {
                playing: player.get("playing").and_then(Value::as_bool).unwrap_or(false),
                position_ms: player.get("position_ms").and_then(Value::as_u64).unwrap_or(0),
                duration_ms: player.get("duration_ms").and_then(Value::as_u64),
                volume: player.get("volume").and_then(Value::as_u64).map(|n| n.min(100) as u8),
                queue: serde_json::from_value(state.get("queue")?.clone()).unwrap_or_default(),
                cursor: state.get("cursor").and_then(Value::as_u64).map(|n| n as usize),
                mode: state.get("mode").and_then(Value::as_str).unwrap_or("sequential").to_owned(),
                current: state.get("current").and_then(|item| serde_json::from_value::<Item>(item.clone()).ok()),
                lyrics: state.get("lyrics").and_then(|item| serde_json::from_value::<Vec<LyricLine>>(item.clone()).ok()).unwrap_or_default(),
            }
        }
        "library" => DaemonEvent::Library {
            playlists: value.get("playlists").and_then(|item| serde_json::from_value(item.clone()).ok()).unwrap_or_default(),
            partial: value.get("partial").and_then(Value::as_bool).unwrap_or(false),
        },
        "search" => DaemonEvent::Search {
            source: value.get("source").and_then(Value::as_str).unwrap_or("").to_owned(),
            kind: value.get("kind").and_then(Value::as_str).unwrap_or("song").to_owned(),
            query: value.get("query").and_then(Value::as_str).unwrap_or("").to_owned(),
            items: value.get("items").and_then(|item| serde_json::from_value(item.clone()).ok()).unwrap_or_default(),
            error: value.get("error").and_then(Value::as_str).unwrap_or("").to_owned(),
        },
        "detail" => DaemonEvent::Detail {
            key: value.get("key").and_then(Value::as_str).unwrap_or("").to_owned(),
            kind: value.get("kind").and_then(Value::as_str).unwrap_or("").to_owned(),
            title: value.get("title").and_then(Value::as_str).unwrap_or("").to_owned(),
            rows: value.get("rows").and_then(|item| serde_json::from_value(item.clone()).ok()).unwrap_or_default(),
            error: value.get("error").and_then(Value::as_str).unwrap_or("").to_owned(),
        },
        "loved" => DaemonEvent::Loved {
            keys: value.get("keys").and_then(|item| serde_json::from_value(item.clone()).ok()).unwrap_or_default(),
        },
        "love" => DaemonEvent::Love {
            key: value.get("key").and_then(Value::as_str).unwrap_or("").to_owned(),
            loved: value.get("loved").and_then(Value::as_bool).unwrap_or(false),
        },
        "downloads" => DaemonEvent::Downloads {
            items: value.get("items").and_then(|item| serde_json::from_value::<Vec<DownloadRow>>(item.clone()).ok()).unwrap_or_default(),
        },
        "pcm" => DaemonEvent::Pcm {
            peaks: numbers(value.get("peaks")),
            scope: numbers(value.get("scope")),
            rms: value.get("rms").and_then(Value::as_f64).unwrap_or(0.0) as f32,
        },
        "lyrics" => DaemonEvent::Lyrics {
            key: value.get("key").and_then(Value::as_str).unwrap_or("").to_owned(),
            lines: value.get("lines").and_then(|item| serde_json::from_value(item.clone()).ok()).unwrap_or_default(),
        },
        "notice" => DaemonEvent::Notice {
            level: value.get("level").and_then(Value::as_str).unwrap_or("info").to_owned(),
            text: value.get("text").and_then(Value::as_str).unwrap_or("").to_owned(),
        },
        "play_error" => DaemonEvent::PlayError(value.get("text").and_then(Value::as_str).unwrap_or("播放失败").to_owned()),
        "loading" => DaemonEvent::Loading(value.get("loading").and_then(Value::as_bool).unwrap_or(false)),
        "ready" => DaemonEvent::Ready,
        _ => return None,
    })
}

fn numbers(value: Option<&Value>) -> Vec<f32> {
    value
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_f64).map(|n| n as f32).collect())
        .unwrap_or_default()
}
