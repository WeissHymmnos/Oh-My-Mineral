use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::time::Instant;

use serde_json::{json, Value};

use crate::backend::Backend;
use crate::daemon_config::{default_daemon_lua_path, DaemonConfigData};
use crate::fuzzy::fuzzy_match;
use crate::model::{
    preset, CommandRow, DaemonEvent, DownloadRow, Item, LyricLine, LyricView, ModeBlob, NavCrumb,
    Notice, SchemeRow, Snapshot, SourceInfo, ViewState, SCHEMES, WORKSPACES,
};
use crate::prefs::{self, Prefs};

const NAV: &[(&str, &str)] = &[
    ("library", "曲库"),
    ("playlists", "歌单"),
    ("loved", "收藏"),
    ("downloads", "下载"),
    ("settings", "设置"),
];

pub struct Session<B: Backend> {
    backend: B,
    path: PathBuf,
    prefs: Prefs,
    pub workspace: String,
    pub input_mode: String,
    pub window_form: String,
    pub immersive: bool,
    pub lyric_track: String,
    pub viz_style: String,
    pub reduce_motion: bool,
    pub auto_adapt: bool,
    pub locked: bool,
    pub close_policy: String,
    pub follow_queue: bool,
    pub cover_mode: String,
    pub source_filter: Option<String>,
    pub shortcuts: BTreeMap<String, String>,
    pub queue: Vec<Item>,
    cursor: Option<usize>,
    current: Option<Item>,
    pub playing: bool,
    pub position_ms: u64,
    pub duration_ms: Option<u64>,
    volume: u8,
    play_mode: String,
    loading: bool,
    play_error: String,
    lyrics: Vec<LyricLine>,
    lyrics_key: String,
    pub peaks: Vec<f32>,
    scope: Vec<f32>,
    rms: f32,
    waterfall: Vec<Vec<f32>>,
    downloads: Vec<DownloadRow>,
    loved: BTreeSet<String>,
    catalog: BTreeMap<String, Item>,
    playlists: Vec<Item>,
    details: BTreeMap<String, (String, Vec<Item>, String)>,
    search_query: String,
    searching: bool,
    search_tab: String,
    list_filter: String,
    search_groups: BTreeMap<String, Vec<Item>>,
    search_parts: BTreeMap<(String, String), Vec<Item>>,
    search_errors: BTreeMap<(String, String), String>,
    search_pending: BTreeSet<(String, String)>,
    search_stale: bool,
    rec_song_key: String,
    rec_album_query: String,
    rec_genre: String,
    rec_albums: Vec<Item>,
    selected_key: String,
    selected_keys: Vec<String>,
    pending_play_detail: Option<String>,
    nav_id: String,
    nav_stack: Vec<NavCrumb>,
    notices: Vec<Notice>,
    notice_seq: u64,
    queue_drawer: bool,
    single_drawer: String,
    palette_open: bool,
    palette_query: String,
    mode_center_open: bool,
    mode_query: String,
    help_open: bool,
    close_prompt: bool,
    pub quit: bool,
    typing: bool,
    mode_stack: Vec<ModeBlob>,
    auto_undo: Option<ModeBlob>,
    return_workspace: String,
    connected: bool,
    daemon_message: String,
    sources: Vec<SourceInfo>,
    theme_bg: String,
    theme_fg: String,
    theme_accent: String,
    pub generation: u64,
    last_tick: Instant,
    rebinding: String,
    pub daemon_config: DaemonConfigData,
    pub daemon_lua_path: PathBuf,
}

impl<B: Backend> Session<B> {
    pub fn open(backend: B, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let prefs = prefs::load(&path);
        let default_lua_path = default_daemon_lua_path();
        let daemon_lua_path = if path == prefs_path_default() {
            default_lua_path.clone()
        } else {
            path.with_file_name("daemon.lua")
        };
        let daemon_config = if daemon_lua_path.exists() {
            DaemonConfigData::load_from_path(&daemon_lua_path)
        } else if default_lua_path.exists() {
            DaemonConfigData::load_from_path(&default_lua_path)
        } else {
            DaemonConfigData::default()
        };
        let mut shortcuts = default_shortcuts();
        shortcuts.extend(prefs.shortcuts.clone());
        let workspace = if WORKSPACES.contains(&prefs.preferred_workspace.as_str()) {
            prefs.preferred_workspace.clone()
        } else {
            "classic".to_owned()
        };
        let initial_volume = daemon_config.audio_volume;
        let mut session = Self {
            backend,
            path,
            workspace: workspace.clone(),
            input_mode: prefs.input_mode.clone(),
            window_form: prefs.window_form.clone(),
            immersive: prefs.immersive,
            lyric_track: prefs.lyric_track.clone(),
            viz_style: prefs.viz_style.clone(),
            reduce_motion: prefs.reduce_motion,
            auto_adapt: prefs.auto_adapt,
            locked: prefs.locked,
            close_policy: prefs.close_policy.clone(),
            follow_queue: prefs.follow_queue,
            cover_mode: prefs.cover_mode.clone(),
            source_filter: prefs.source_filter.clone(),
            shortcuts,
            queue: Vec::new(),
            cursor: None,
            current: None,
            playing: false,
            position_ms: 0,
            duration_ms: None,
            volume: initial_volume,
            play_mode: "sequential".to_owned(),
            loading: false,
            play_error: String::new(),
            lyrics: Vec::new(),
            lyrics_key: String::new(),
            peaks: vec![0.0; 48],
            scope: Vec::new(),
            rms: 0.0,
            waterfall: Vec::new(),
            downloads: Vec::new(),
            loved: BTreeSet::new(),
            catalog: BTreeMap::new(),
            playlists: Vec::new(),
            details: BTreeMap::new(),
            search_query: String::new(),
            searching: false,
            search_tab: "all".to_owned(),
            list_filter: String::new(),
            search_groups: empty_groups(),
            search_parts: BTreeMap::new(),
            search_errors: BTreeMap::new(),
            search_pending: BTreeSet::new(),
            search_stale: false,
            rec_song_key: String::new(),
            rec_album_query: String::new(),
            rec_genre: String::new(),
            rec_albums: Vec::new(),
            selected_key: String::new(),
            selected_keys: Vec::new(),
            pending_play_detail: None,
            nav_id: "library".to_owned(),
            nav_stack: Vec::new(),
            notices: Vec::new(),
            notice_seq: 1,
            queue_drawer: false,
            single_drawer: String::new(),
            palette_open: false,
            palette_query: String::new(),
            mode_center_open: false,
            mode_query: String::new(),
            help_open: false,
            close_prompt: false,
            quit: false,
            typing: false,
            mode_stack: Vec::new(),
            auto_undo: None,
            return_workspace: workspace,
            connected: false,
            daemon_message: "尚未连接 Daemon".to_owned(),
            sources: Vec::new(),
            theme_bg: "#111318".to_owned(),
            theme_fg: "#E2E2E9".to_owned(),
            theme_accent: "#A8C7FA".to_owned(),
            generation: 1,
            last_tick: Instant::now(),
            rebinding: String::new(),
            daemon_config,
            daemon_lua_path,
            prefs,
        };
        if !session.prefs.load_error.is_empty() {
            let text = session.prefs.load_error.clone();
            session.notify("error", &text);
        }
        session
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    pub fn close(&mut self) {
        self.persist();
        self.backend.close();
    }

    pub fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.saturating_duration_since(self.last_tick).as_millis() as u64;
        self.last_tick = now;
        if self.playing {
            self.position_ms = self.position_ms.saturating_add(delta);
            if let Some(duration) = self.duration_ms {
                self.position_ms = self.position_ms.min(duration);
            }
        }
        let events = self.backend.poll();
        let mut structural = false;
        for event in events {
            let changed = match &event {
                DaemonEvent::Pcm { .. } | DaemonEvent::Ready => false,
                DaemonEvent::Player {
                    queue,
                    cursor,
                    mode,
                    current,
                    lyrics,
                    playing,
                    ..
                } => !self.player_unchanged(queue, *cursor, mode, current.as_ref(), lyrics, *playing),
                DaemonEvent::Downloads { items } => !self.downloads_unchanged(items),
                _ => true,
            };
            self.apply_event(event);
            structural |= changed;
        }
        if structural {
            self.generation = self.generation.saturating_add(1);
        }
    }

    fn player_unchanged(
        &self,
        queue: &[Item],
        cursor: Option<usize>,
        mode: &str,
        current: Option<&Item>,
        lyrics: &[LyricLine],
        playing: bool,
    ) -> bool {
        self.playing == playing
            && self.play_mode == mode
            && self.cursor == cursor
            && self.queue.len() == queue.len()
            && self.queue.iter().zip(queue).all(|(old, new)| old.key == new.key)
            && self.current.as_ref().map(|song| song.key.as_str()) == current.map(|song| song.key.as_str())
            && (lyrics.is_empty() || self.lyrics.len() == lyrics.len())
    }

    fn downloads_unchanged(&self, items: &[DownloadRow]) -> bool {
        self.downloads.len() == items.len()
            && self.downloads.iter().zip(items).all(|(old, new)| {
                old.song.as_ref().map(|s| s.key.as_str()) == new.song.as_ref().map(|s| s.key.as_str())
                    && old.status == new.status
                    && old.failure == new.failure
            })
    }

    pub fn dispatch_value(&mut self, value: &Value) {
        let action = value.get("action").and_then(Value::as_str).unwrap_or("");
        let text = |key: &str| value.get(key).and_then(Value::as_str).unwrap_or("").to_owned();
        let flag = |key: &str, default: bool| value.get(key).and_then(Value::as_bool).unwrap_or(default);
        let number = |key: &str| value.get(key).and_then(Value::as_i64).unwrap_or(0);
        match action {
            "set_typing" => {
                let on = flag("on", false);
                if self.typing == on {
                    return;
                }
                self.set_typing(on);
            }
            "search" => self.search(text("query")),
            "search_tab" => {
                let tab = text("tab");
                if matches!(tab.as_str(), "all" | "song" | "artist" | "album" | "playlist") {
                    self.search_tab = tab;
                }
            }
            "source_filter" => self.set_source_filter(text("source")),
            "list_filter" => self.list_filter = text("query"),
            "select" => {
                let key = text("key");
                if self.selected_key == key {
                    return;
                }
                self.select(key);
            }
            "select_delta" => self.select_delta(number("delta") as isize),
            "select_many" => {
                let keys: Vec<String> = value.get("keys").and_then(|v| serde_json::from_value(v.clone()).ok()).unwrap_or_default();
                let next_selected = keys.last().cloned().unwrap_or_else(|| self.selected_key.clone());
                if self.selected_keys == keys && self.selected_key == next_selected {
                    return;
                }
                self.selected_keys = keys;
                self.selected_key = next_selected;
            }
            "activate" => self.activate(text("key")),
            "play_container" => self.play_container(text("key")),
            "enqueue" => self.enqueue(text("key")),
            "play_next" => self.play_next(text("key")),
            "toggle_play" => self.transport("toggle"),
            "next" => self.transport("next"),
            "prev" => self.transport("prev"),
            "seek" => self.seek(number("ms").max(0) as u64),
            "volume" => self.volume(number("pct")),
            "cycle_mode" => self.transport("cycle_mode"),
            "scheme" => self.scheme(&text("scheme")),
            "toggle_immersive" => {
                if self.window_form == "mini" {
                    return;
                }
                if self.immersive {
                    self.immersive = false;
                } else {
                    self.scheme("immersive");
                }
            }
            "input_mode" => {
                let mode = text("mode");
                if matches!(mode.as_str(), "pointer" | "keyboard" | "touch") {
                    self.input_mode = if self.input_mode == mode && mode != "pointer" {
                        "pointer".to_owned()
                    } else {
                        mode
                    };
                }
            }
            "return_mode" => self.return_mode(),
            "mode_center" => {
                if value.get("open").is_none() {
                    self.mode_center_open = !self.mode_center_open;
                } else {
                    self.mode_center_open = flag("open", false);
                }
                if value.get("query").is_some() {
                    self.mode_query = text("query");
                }
            }
            "palette" => {
                self.palette_open = if value.get("open").is_none() { !self.palette_open } else { flag("open", false) };
                if value.get("query").is_some() || !self.palette_open {
                    self.palette_query = text("query");
                }
            }
            "run_command" => self.run_command(&text("command")),
            "help" => self.help_open = !self.help_open,
            "queue_drawer" => self.queue_drawer = !self.queue_drawer,
            "drawer" => {
                let name = text("name");
                self.single_drawer = if self.single_drawer == name { String::new() } else { name };
                self.queue_drawer = self.single_drawer == "queue";
            }
            "lyric_track" => self.cycle_lyric(),
            "lyric_seek" => self.seek(number("ms").max(0) as u64),
            "viz" => {
                let style = text("style");
                if matches!(style.as_str(), "bars" | "scope" | "waterfall" | "terrain") {
                    self.viz_style = style;
                }
            }
            "cycle_viz" => {
                self.viz_style = match self.viz_style.as_str() {
                    "bars" => "scope",
                    "scope" => "waterfall",
                    "waterfall" => "terrain",
                    _ => "bars",
                }
                .to_owned();
            }
            "reveal" => self.reveal(),
            "follow" => self.follow_queue = flag("on", true),
            "queue_move" => {
                let idx = value
                    .get("index")
                    .and_then(Value::as_i64)
                    .filter(|n| *n >= 0)
                    .map(|n| n as usize)
                    .unwrap_or_else(|| self.selected_queue_index());
                self.queue_move(idx, &text("to"));
            }
            "queue_remove" => {
                let idx = value
                    .get("index")
                    .and_then(Value::as_i64)
                    .filter(|n| *n >= 0)
                    .map(|n| n as usize)
                    .unwrap_or_else(|| self.selected_queue_index());
                self.queue_remove(idx);
            }
            "love" => self.love(&text("key")),
            "download" => self.download(&text("key")),
            "copy" => self.copy(&text("key")),
            "retry" => self.retry(),
            "open" => self.open_key(&text("key"), &text("kind")),
            "back" => self.back(),
            "nav" => {
                self.nav_id = text("nav");
                self.searching = false;
                self.nav_stack.clear();
                self.cover_mode = "list".to_owned();
            }
            "cover_mode" => self.cover_mode(&text("mode")),
            "report_view" => self.report_view(number("scroll") as i32, &text("focus"), &text("expanded")),
            "panels" => {
                if value.get("left").is_some() {
                    self.prefs.panel_left = number("left") as i32;
                }
                if value.get("right").is_some() {
                    self.prefs.panel_right = number("right") as i32;
                }
            }
            "window_width" => {
                let prev_workspace = self.workspace.clone();
                let prev_undo = self.auto_undo.is_some();
                self.window_width(number("width") as i32);
                if self.workspace == prev_workspace && self.auto_undo.is_some() == prev_undo {
                    return;
                }
            }
            "undo_auto" => self.undo_auto(),
            "lock" => self.locked = flag("on", true),
            "auto_adapt" => self.auto_adapt = flag("on", true),
            "reduce_motion" => self.reduce_motion = flag("on", true),
            "close_policy" => {
                let policy = text("policy");
                if matches!(policy.as_str(), "ask" | "ui" | "stop") {
                    self.close_policy = policy;
                }
            }
            "request_close" => self.request_close(),
            "confirm_close" => self.confirm_close(&text("choice")),
            "connect" => {
                self.backend.command("connect", json!({}));
                self.daemon_message = "正在重连".to_owned();
            }
            "launch_daemon" => {
                self.backend.command("launch_daemon", json!({}));
                self.daemon_message = "正在启动 daemon".to_owned();
            }
            "shutdown_daemon" => self.shutdown_daemon(),
            "restart_daemon" => {
                self.backend.command("restart_daemon", json!({}));
                self.daemon_message = "Restarting Mineral daemon...".to_owned();
                self.notify("info", "Restarting Mineral daemon...");
            }
            "cycle_audio_output" => {
                if self.connected {
                    self.backend.command("cycle_audio_output", json!({}));
                } else {
                    self.notify("error", "Daemon disconnected; cannot switch audio output device");
                }
            }
            "daemon_config_set" => {
                let key = text("key");
                let val = text("value");
                let clean_key = key.strip_prefix("settings:daemon:").unwrap_or(&key);
                match self.daemon_config.set_or_cycle(clean_key, &val) {
                    Ok(msg) => {
                        if clean_key == "audio.volume" {
                            let pct = self.daemon_config.audio_volume as i64;
                            self.volume(pct);
                        }
                        match self.daemon_config.save_to_path(&self.daemon_lua_path) {
                            Ok(()) => self.notify("info", &format!("{msg} (saved to daemon.lua)")),
                            Err(err) => self.notify("error", &err),
                        }
                    }
                    Err(err) => self.notify("error", &err),
                }
            }
            "daemon_config_reset" => {
                let custom_setup = self.daemon_config.custom_setup_lua.clone();
                self.daemon_config = DaemonConfigData {
                    custom_setup_lua: custom_setup,
                    ..DaemonConfigData::default()
                };
                match self.daemon_config.save_to_path(&self.daemon_lua_path) {
                    Ok(()) => self.notify("info", "Reset all daemon settings to daemon-default.lua"),
                    Err(err) => self.notify("error", &err),
                }
            }
            "daemon_save_raw_lua" => {
                let lua = text("lua")
                    .replace("推荐", "")
                    .replace("排行榜", "")
                    .replace("删除远端", "")
                    .replace("重命名远端", "");
                if let Some(parent) = self.daemon_lua_path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                match std::fs::write(&self.daemon_lua_path, &lua) {
                    Ok(()) => {
                        self.daemon_config = DaemonConfigData::parse_lua(&lua);
                        self.notify("info", "Saved daemon.lua and triggered daemon hot-reload");
                    }
                    Err(err) => self.notify("error", &format!("Failed to write daemon.lua: {err}")),
                }
            }
            "save_combo" => self.save_combo(&text("name")),
            "apply_combo" => self.apply_combo(&text("name")),
            "delete_combo" => {
                let name = text("name");
                self.prefs.custom_modes.retain(|item| item.name != name);
            }
            "modular_preset" => self.modular_preset(&text("preset")),
            "modular_move" => self.modular_move(&text("module"), &text("direction")),
            "toggle_module" => self.toggle_module(&text("module")),
            "reset_layout" => self.reset_layout(),
            "reset_modes" => self.reset_modes(),
            "save_playlist" => {
                if self.connected {
                    self.backend.command("save_playlist", json!({"name": text("name")}));
                } else {
                    self.notify("error", "Daemon 断连，未保存歌单");
                }
            }
            "dismiss_notice" => {
                let id = number("id").max(0) as u64;
                self.notices.retain(|notice| notice.id != id);
            }
            "theme" => self.theme(number("r") as u8, number("g") as u8, number("b") as u8),
            "rebind" => self.rebinding = text("action"),
            "set_shortcut" => {
                let action = text("action");
                let key = text("key");
                if self.shortcuts.contains_key(&action) && !key.is_empty() {
                    self.shortcuts.insert(action, key);
                }
                self.rebinding.clear();
            }
            "expanded" => {
                let key = text("key");
                let opening = self.view().expanded != key && !key.is_empty();
                {
                    let state = self.view_mut();
                    state.expanded = if state.expanded == key { String::new() } else { key.clone() };
                }
                if opening && !self.details.contains_key(&key) && self.connected {
                    let kind = self.catalog.get(&key).map(|item| item.kind.clone()).unwrap_or_else(|| "playlist".to_owned());
                    if matches!(kind.as_str(), "playlist" | "album" | "artist") {
                        self.backend.command("open", json!({"key": key, "kind": kind}));
                    }
                }
            }
            _ => self.notify("error", "没有这个操作"),
        }
        self.persist();
        self.generation = self.generation.saturating_add(1);
    }

    pub fn dispatch_action(&mut self, action: &str, extra: Value) {
        let mut value = extra;
        if let Some(object) = value.as_object_mut() {
            object.insert("action".to_owned(), Value::String(action.to_owned()));
            let owned = value.clone();
            self.dispatch_value(&owned);
        } else {
            self.dispatch_value(&json!({"action": action}));
        }
    }

    pub fn snapshot(&self) -> Snapshot {
        let (mut rows, title, crumbs, kind) = self.compose();
        rows.retain(|row| fuzzy_match(&self.list_filter, &format!("{} {} {}", row.title, row.subtitle, row.album)));
        let current_key = self.current.as_ref().map(|c| c.key.as_str()).unwrap_or("");
        for row in &mut rows {
            if self.loved.contains(&row.key) {
                row.loved = true;
            }
            if !current_key.is_empty() && row.key == current_key {
                row.current = true;
            }
        }
        let total_duration_ms: u64 = rows.iter().filter_map(|row| row.duration_ms).sum();
        let mut selected = self.selected_key.clone();
        if !rows.is_empty() && !rows.iter().any(|row| row.key == selected) {
            selected = rows[0].key.clone();
        }
        let selected_item = self
            .catalog
            .get(&selected)
            .cloned()
            .or_else(|| rows.iter().find(|row| row.key == selected).cloned());
        let next_title = self
            .cursor
            .and_then(|idx| self.queue.get(idx + 1))
            .map(|song| format!("{} · {}", song.title, song.subtitle))
            .unwrap_or_default();
        let library_items = if self.searching && !self.search_query.is_empty() {
            self.search_rows()
        } else if let Some(top) = self.nav_stack.last()
            && let Some((_, detail_rows, _)) = self.details.get(&top.key)
        {
            detail_rows.clone()
        } else {
            self.library_rows()
        };
        let downloads_rows = self
            .downloads
            .iter()
            .filter_map(|item| {
                let mut row = item.song.clone()?;
                let status = if let Some(failure) = &item.failure {
                    format!("失败：{failure}")
                } else {
                    item.status.clone()
                };
                if !status.is_empty() {
                    row.subtitle = format!("{} · {status}", row.subtitle);
                }
                Some(row)
            })
            .collect();
        let scheme = self.active_scheme();
        let current = self.current.clone().unwrap_or_else(|| Item {
            key: String::new(),
            kind: "song".to_owned(),
            title: "没有正在播放".to_owned(),
            subtitle: String::new(),
            album: String::new(),
            source: String::new(),
            duration_ms: None,
            unavailable: false,
            loved: false,
            cover_url: None,
            index: None,
            current: false,
        });
        Snapshot {
            scheme_id: scheme.clone(),
            scheme_label: scheme_label(&scheme),
            workspace: self.workspace.clone(),
            input_mode: self.input_mode.clone(),
            window_form: self.window_form.clone(),
            touch: self.input_mode == "touch",
            keyboard: self.input_mode == "keyboard",
            immersive: self.immersive,
            mini: self.window_form == "mini",
            reduce_motion: self.reduce_motion,
            daemon_connected: self.connected,
            daemon_message: self.daemon_message.clone(),
            sources: self.sources.clone(),
            source_filter: self.source_filter.clone(),
            nav: NAV.iter().map(|(id, title)| {
                let mut item = Item::nav(id, title, "");
                item.current = *id == self.nav_id && !self.searching;
                item.key = (*id).to_owned();
                item
            }).collect(),
            content_kind: kind,
            content_title: title,
            crumbs,
            rows,
            selected_key: selected,
            empty: self.empty_reason(),
            list_filter: self.list_filter.clone(),
            cover_mode: self.cover_mode.clone(),
            search_query: self.search_query.clone(),
            search_updating: !self.search_pending.is_empty(),
            search_summary: self.search_summary(),
            search_songs: self.search_groups.get("song").cloned().unwrap_or_default(),
            search_artists: self.search_groups.get("artist").cloned().unwrap_or_default(),
            search_albums: self.search_groups.get("album").cloned().unwrap_or_default(),
            search_playlists: self.search_groups.get("playlist").cloned().unwrap_or_default(),
            rec_albums: self.recommended_album_wall(),
            rec_genre: self.rec_genre.clone(),
            recent_searches: self.prefs.recent_searches.clone(),
            recent_plays: self.prefs.recent_plays.iter().filter_map(|key| self.catalog.get(key).cloned()).collect(),
            queue: self.queue_rows(),
            cursor: self.cursor,
            follow_queue: self.follow_queue,
            queue_open: self.queue_drawer || self.workspace == "queue",
            player_title: if current.key.is_empty() { "没有正在播放".to_owned() } else { current.title.clone() },
            player_subtitle: current.subtitle.clone(),
            player_source: current.source.clone(),
            player_key: current.key.clone(),
            cover_url: current.cover_url.clone(),
            playing: self.playing,
            loading: self.loading,
            play_error: self.play_error.clone(),
            position_ms: self.position_ms,
            duration_ms: self.duration_ms.or(current.duration_ms),
            volume: self.volume,
            play_mode: self.play_mode.clone(),
            loved: self.loved.contains(&current.key),
            unavailable: current.unavailable,
            lyric_track: self.lyric_track.clone(),
            lyrics: self.lyric_view(),
            viz_style: self.viz_style.clone(),
            peaks: self.peaks.clone(),
            scope: self.scope.clone(),
            waterfall: self.waterfall.iter().rev().take(40).cloned().collect::<Vec<_>>().into_iter().rev().collect(),
            rms: self.rms,
            notices: self.notices.clone(),
            single_drawer: self.single_drawer.clone(),
            panel_left: self.prefs.panel_left,
            panel_right: self.prefs.panel_right,
            modules: self.prefs.modules.clone(),
            modular_preset: self.prefs.modular_preset.clone(),
            schemes: self.scheme_rows(),
            custom_modes: self.prefs.custom_modes.iter().map(|item| item.name.clone()).collect(),
            mode_center_open: self.mode_center_open,
            palette_open: self.palette_open,
            commands: self.commands(),
            help_open: self.help_open,
            shortcuts_enabled: !self.typing,
            close_prompt: self.close_prompt,
            close_policy: self.close_policy.clone(),
            auto_adapt: self.auto_adapt,
            locked: self.locked,
            can_undo_auto: self.auto_undo.is_some(),
            quit: self.quit,
            theme_bg: self.theme_bg.clone(),
            theme_fg: self.theme_fg.clone(),
            theme_accent: self.theme_accent.clone(),
            settings: self.settings_rows(),
            scroll: self.view().scroll,
            expanded_key: self.view().expanded.clone(),
            focus_area: self.view().focus.clone(),
            search_tab: self.search_tab.clone(),
            mode_query: self.mode_query.clone(),
            palette_query: self.palette_query.clone(),
            library_items,
            downloads_rows,
            next_title,
            selected_title: selected_item.as_ref().map(|item| item.title.clone()).unwrap_or_default(),
            selected_subtitle: selected_item.as_ref().map(|item| item.subtitle.clone()).unwrap_or_default(),
            selected_album: selected_item.as_ref().map(|item| item.album.clone()).unwrap_or_default(),
            selected_source: selected_item.as_ref().map(|item| item.source.clone()).unwrap_or_default(),
            selected_kind: selected_item.as_ref().map(|item| item.kind.clone()).unwrap_or_default(),
            selected_duration_ms: selected_item.as_ref().and_then(|item| item.duration_ms),
            selected_loved: selected_item.as_ref().is_some_and(|item| item.loved || self.loved.contains(&item.key)),
            selected_cover: selected_item.and_then(|item| item.cover_url),
            total_duration_ms,
            typing: self.typing,
            generation: self.generation,
        }
    }

    fn set_typing(&mut self, on: bool) {
        self.typing = on;
    }

    fn search(&mut self, query: String) {
        self.search_query = query.trim().to_owned();
        remember(&mut self.prefs.recent_searches, &self.search_query);
        let sources = self.active_sources();
        if self.search_query.is_empty() {
            self.searching = false;
            self.search_pending.clear();
            self.search_groups = empty_groups();
            self.search_parts.clear();
            return;
        }
        self.searching = true;
        self.nav_stack.clear();
        if !self.connected {
            self.search_pending.clear();
            self.notify("error", "Daemon 断连，无法搜索");
            return;
        }
        if sources.is_empty() {
            self.search_pending.clear();
            self.notify("error", "音源未登录或没有可搜索的音源");
            return;
        }
        self.search_stale = true;
        self.search_pending = sources
            .iter()
            .flat_map(|source| ["song", "artist", "album", "playlist"].into_iter().map(|kind| (source.id.clone(), kind.to_owned())))
            .collect();
        let ids: Vec<String> = sources.iter().map(|source| source.id.clone()).collect();
        self.backend.command("search", json!({"query": self.search_query, "sources": ids}));
    }

    fn set_source_filter(&mut self, source: String) {
        self.source_filter = if source.is_empty() { None } else { Some(source.clone()) };
        if self.nav_stack.last().is_some_and(|c| c.key.starts_with("source:")) {
            self.nav_stack.pop();
            if !source.is_empty() {
                self.open_key(&format!("source:{source}"), "source");
            }
        }
        if !self.search_query.is_empty() {
            let query = self.search_query.clone();
            self.search(query);
        }
        self.rec_album_query.clear();
        self.refresh_rec_albums();
    }

    fn select(&mut self, key: String) {
        self.selected_key = key.clone();
        if !key.is_empty() && !self.selected_keys.contains(&key) {
            self.selected_keys = vec![key];
        }
    }

    fn select_delta(&mut self, delta: isize) {
        let (mut rows, _, _, _) = self.compose();
        rows.retain(|row| fuzzy_match(&self.list_filter, &format!("{} {}", row.title, row.subtitle)));
        if rows.is_empty() {
            return;
        }
        let current_idx = rows.iter().position(|row| row.key == self.selected_key).unwrap_or(0) as isize;
        let next_idx = (current_idx + delta).clamp(0, (rows.len() - 1) as isize) as usize;
        self.select(rows[next_idx].key.clone());
    }

    fn activate(&mut self, key: String) {
        let key = if key.is_empty() { self.selected_key.clone() } else { key };
        if let Some(q) = key.strip_prefix("recent_search:") {
            self.search(q.to_owned());
            return;
        }
        if key.starts_with("settings:") {
            self.selected_key = key.clone();
            match key.as_str() {
                "settings:close" => {
                    self.close_policy = match self.close_policy.as_str() {
                        "ask" => "ui",
                        "ui" => "stop",
                        _ => "ask",
                    }
                    .to_owned();
                }
                "settings:adapt" => self.auto_adapt = !self.auto_adapt,
                "settings:lock" => self.locked = !self.locked,
                "settings:motion" => self.reduce_motion = !self.reduce_motion,
                "settings:reset-layout" => self.reset_layout(),
                "settings:reset-modes" => self.reset_modes(),
                "settings:daemon" => {
                    if self.connected {
                        self.backend.command("connect", json!({}));
                    } else {
                        self.backend.command("launch_daemon", json!({}));
                    }
                }
                "settings:daemon:restart" => {
                    self.dispatch_value(&json!({"action": "restart_daemon"}));
                }
                "settings:daemon:audio_output" => {
                    self.dispatch_value(&json!({"action": "cycle_audio_output"}));
                }
                "settings:daemon:reset_defaults" => {
                    self.dispatch_value(&json!({"action": "daemon_config_reset"}));
                }
                _ => {
                    if let Some(field) = key.strip_prefix("settings:daemon:") {
                        self.dispatch_value(&json!({
                            "action": "daemon_config_set",
                            "key": field,
                            "value": "__cycle__",
                        }));
                    }
                }
            }
            return;
        }
        let Some(row) = self.catalog.get(&key).cloned().or_else(|| self.find_row(&key)) else { return };
        self.selected_key = key.clone();
        if matches!(row.kind.as_str(), "playlist" | "album" | "artist" | "nav" | "source") {
            self.open_key(&key, &row.kind);
            return;
        }
        let rows = self.playable_context();
        let mut keys: Vec<String> = rows.iter().map(|item| item.key.clone()).collect();
        let index = if let Some(index) = keys.iter().position(|item| item == &key) {
            index
        } else {
            keys = vec![key];
            0
        };
        let context = self.context_for(&row);
        self.play_keys(keys, index, context);
    }

    fn play_container(&mut self, key: String) {
        let target_key = if !key.is_empty() {
            key.clone()
        } else if let Some(top) = self.nav_stack.last()
            && self.details.contains_key(&top.key)
        {
            top.key.clone()
        } else {
            self.selected_key.clone()
        };
        if let Some((title, rows, _)) = self.details.get(&target_key).cloned() {
            let song_keys: Vec<String> = rows.iter().filter(|r| r.kind == "song").map(|r| r.key.clone()).collect();
            if !song_keys.is_empty() {
                let kind = self
                    .catalog
                    .get(&target_key)
                    .map(|r| r.kind.as_str())
                    .unwrap_or("playlist");
                let ctx_kind = if kind == "album" { "album" } else { "playlist" };
                self.play_keys(song_keys, 0, json!({"kind": ctx_kind, "key": target_key, "name": title}));
                return;
            }
        }
        if key.is_empty() {
            let playable = self.playable_context();
            let song_keys: Vec<String> = playable.iter().map(|r| r.key.clone()).collect();
            if !song_keys.is_empty() {
                self.play_keys(song_keys, 0, json!({"kind": "manual"}));
                return;
            }
        }
        let kind = self.catalog.get(&target_key).map(|r| r.kind.clone()).unwrap_or_else(|| "playlist".to_owned());
        self.pending_play_detail = Some(target_key.clone());
        self.open_key(&target_key, &kind);
    }

    fn selected_queue_index(&self) -> usize {
        self.queue
            .iter()
            .position(|song| song.key == self.selected_key)
            .or(self.cursor)
            .unwrap_or(0)
    }

    fn enqueue(&mut self, key: String) {
        let chosen = if !self.selected_keys.is_empty() {
            self.selected_keys.clone()
        } else if !key.is_empty() {
            vec![key]
        } else {
            Vec::new()
        };
        let songs: Vec<Item> = chosen.iter().filter_map(|key| self.catalog.get(key).cloned()).filter(|item| item.kind == "song").collect();
        if songs.is_empty() {
            return;
        }
        if !self.connected {
            self.notify("error", "Daemon 断连，没有加入队列");
            return;
        }
        let keys: Vec<String> = songs.iter().map(|song| song.key.clone()).collect();
        self.backend.command("enqueue", json!({"keys": keys, "context": {"kind": "manual"}}));
        self.notify("info", &format!("已加入队列 {} 首", songs.len()));
    }

    fn play_next(&mut self, key: String) {
        let key = if key.is_empty() { self.selected_key.clone() } else { key };
        if !self.catalog.contains_key(&key) || !self.connected {
            self.notify("error", if self.connected { "找不到歌曲" } else { "Daemon 断连，无法插播" });
            return;
        }
        self.backend.command("play_next", json!({"keys": [key], "context": {"kind": "manual"}}));
        self.notify("info", "将在当前歌曲后播放");
    }

    fn seek(&mut self, ms: u64) {
        if !self.connected {
            self.notify("error", "Daemon 断连，无法跳转");
            return;
        }
        self.position_ms = ms;
        self.backend.command("seek", json!({"ms": ms}));
    }

    fn volume(&mut self, pct: i64) {
        self.volume = pct.clamp(0, 100) as u8;
        if self.connected {
            self.backend.command("volume", json!({"pct": self.volume}));
        }
    }

    fn scheme(&mut self, scheme: &str) {
        self.push_mode();
        self.apply_scheme(scheme, true);
        self.mode_center_open = false;
        self.notify("info", &format!("当前模式：{}。Ctrl+Shift+M 打开模式中心", scheme_label(scheme)));
    }

    fn return_mode(&mut self) {
        let Some(previous) = self.mode_stack.pop() else {
            self.notify("info", "没有上一个模式");
            return;
        };
        self.restore_mode(previous);
        self.notify("info", "已返回上一个模式");
    }

    fn run_command(&mut self, command: &str) {
        self.palette_open = false;
        if let Some(scheme) = command.strip_prefix("scheme:") {
            self.scheme(scheme);
            return;
        }
        if let Some(preset_name) = command.strip_prefix("modular_preset:") {
            self.modular_preset(preset_name);
            return;
        }
        if let Some(query) = command.strip_prefix("search_now:") {
            self.search(query.to_owned());
            return;
        }
        if let Some(key) = command.strip_prefix("play_item:") {
            self.activate(key.to_owned());
            return;
        }
        match command {
            "toggle_play" => self.transport("toggle"),
            "next" => self.transport("next"),
            "prev" => self.transport("prev"),
            "cycle_mode" => self.transport("cycle_mode"),
            "reveal" => self.reveal(),
            "return_mode" => self.return_mode(),
            "connect" => self.backend.command("connect", json!({})),
            "launch_daemon" => self.backend.command("launch_daemon", json!({})),
            "help" => self.help_open = true,
            "enqueue" => self.enqueue(String::new()),
            "love" => self.love(""),
            "download" => self.download(""),
            "toggle_immersive" => {
                if self.window_form != "mini" {
                    self.immersive = !self.immersive;
                }
            }
            "cycle_viz" => {
                self.viz_style = match self.viz_style.as_str() {
                    "bars" => "scope",
                    "scope" => "waterfall",
                    "waterfall" => "terrain",
                    _ => "bars",
                }
                .to_owned();
            }
            "queue_clear" => self.transport("queue_clear"),
            "queue_dedupe" => self.transport("queue_dedupe"),
            "queue_clear_unavailable" => self.transport("queue_clear_unavailable"),
            _ => {}
        }
    }

    fn cycle_lyric(&mut self) {
        self.lyric_track = match self.lyric_track.as_str() {
            "original" => "translation",
            "translation" => "romanization",
            _ => "original",
        }.to_owned();
    }

    fn reveal(&mut self) {
        self.follow_queue = true;
        if let Some(index) = self.cursor {
            if let Some(song) = self.queue.get(index) {
                self.selected_key = song.key.clone();
            }
        }
    }

    fn queue_move(&mut self, index: usize, to: &str) {
        if !self.connected {
            self.notify("error", "Daemon 断连，队列未修改");
            return;
        }
        let Some(song) = self.queue.get(index) else { return };
        let key = song.key.clone();
        self.backend.command("queue_move", json!({"index": index, "key": key, "to": to}));
    }

    fn queue_remove(&mut self, index: usize) {
        if !self.connected || index >= self.queue.len() {
            self.notify("error", "Daemon 断连，队列未修改");
            return;
        }
        let key = self.queue[index].key.clone();
        self.backend.command("queue_remove", json!({"index": index, "key": key}));
    }

    fn love(&mut self, key: &str) {
        let key = if key.is_empty() { self.selected_key.clone() } else { key.to_owned() };
        if !self.connected {
            self.notify("error", "Daemon 断连，无法收藏");
            return;
        }
        self.backend.command("love", json!({"key": key}));
    }

    fn download(&mut self, key: &str) {
        let key = if key.is_empty() { self.selected_key.clone() } else { key.to_owned() };
        if !self.connected {
            self.notify("error", "Daemon 断连，无法下载");
            return;
        }
        self.backend.command("download", json!({"key": key}));
        self.notify("info", "已提交下载");
    }

    fn copy(&mut self, key: &str) {
        let key = if key.is_empty() { self.selected_key.clone() } else { key.to_owned() };
        if let Some(row) = self.catalog.get(&key) {
            self.notify("info", &format!("已复制 {} · {}", row.title, row.key));
        }
    }

    fn retry(&mut self) {
        if !self.queue.is_empty() && self.cursor.is_some() {
            let keys: Vec<String> = self.queue.iter().map(|song| song.key.clone()).collect();
            let index = self.cursor.unwrap_or(0);
            self.play_keys(keys, index, json!({"kind": "manual"}));
        }
    }

    fn open_key(&mut self, key: &str, kind: &str) {
        self.cover_mode = "list".to_owned();
        let row = self.catalog.get(key).cloned().or_else(|| self.find_row(key));
        let kind = if kind.is_empty() {
            row.as_ref()
                .map(|item| item.kind.clone())
                .unwrap_or_else(|| {
                    if key.starts_with("nav:") {
                        "nav".to_owned()
                    } else if key.starts_with("source:") {
                        "source".to_owned()
                    } else {
                        String::new()
                    }
                })
        } else {
            kind.to_owned()
        };
        if kind == "nav" {
            self.nav_id = key.trim_start_matches("nav:").to_owned();
            self.nav_stack.clear();
            self.searching = false;
            return;
        }
        if kind == "source" {
            let source_id = key.trim_start_matches("source:");
            let source_key = format!("source:{source_id}");
            let title = self
                .sources
                .iter()
                .find(|s| s.id == source_id)
                .map(|s| if s.label.is_empty() { s.id.clone() } else { s.label.clone() })
                .unwrap_or_else(|| source_id.to_owned());
            self.nav_stack.push(NavCrumb {
                nav: self.nav_id.clone(),
                key: source_key.clone(),
                title,
            });
            self.selected_key = source_key;
            return;
        }
        if matches!(kind.as_str(), "playlist" | "album" | "artist") {
            if self.workspace == "queue" {
                self.workspace = "classic".to_owned();
            }
            if self.nav_id == "settings" {
                self.nav_id = "library".to_owned();
            }
        }
        if let Some(norm_album) = key.strip_prefix("local_album:") {
            let mut tracks = Vec::new();
            let mut album_title = row.as_ref().map(|r| r.title.clone()).unwrap_or_else(|| norm_album.to_owned());
            let mut first_cover = row.as_ref().and_then(|r| r.cover_url.clone());
            for song in self.catalog.values().chain(self.queue.iter()) {
                if song.kind == "song" && normalize_album_key(&song.album) == norm_album {
                    if album_title == norm_album && !song.album.trim().is_empty() {
                        album_title = song.album.trim().to_owned();
                    }
                    if first_cover.is_none() && song.cover_url.is_some() {
                        first_cover = song.cover_url.clone();
                    }
                    if !tracks.iter().any(|t: &Item| t.key == song.key) {
                        tracks.push(song.clone());
                    }
                }
            }
            for (_, detail_rows, _) in self.details.values() {
                for song in detail_rows {
                    if song.kind == "song" && normalize_album_key(&song.album) == norm_album {
                        if first_cover.is_none() && song.cover_url.is_some() {
                            first_cover = song.cover_url.clone();
                        }
                        if !tracks.iter().any(|t: &Item| t.key == song.key) {
                            tracks.push(song.clone());
                        }
                    }
                }
            }
            let synth_item = row.clone().unwrap_or_else(|| Item {
                key: key.to_owned(),
                kind: "album".to_owned(),
                title: album_title.clone(),
                subtitle: tracks.first().map(|t| t.subtitle.clone()).unwrap_or_default(),
                album: album_title.clone(),
                source: tracks.first().map(|t| t.source.clone()).unwrap_or_default(),
                duration_ms: None,
                unavailable: false,
                loved: false,
                cover_url: first_cover,
                index: None,
                current: false,
            });
            self.catalog.insert(key.to_owned(), synth_item);
            self.details.insert(key.to_owned(), (album_title.clone(), tracks.clone(), String::new()));
            if self.nav_stack.last().is_none_or(|c| c.key != key) {
                self.nav_stack.push(NavCrumb {
                    nav: self.nav_id.clone(),
                    key: key.to_owned(),
                    title: album_title.clone(),
                });
            }
            self.selected_key = key.to_owned();
            if self.pending_play_detail.as_deref() == Some(key) && !tracks.is_empty() {
                let song_keys: Vec<String> = tracks.iter().map(|r| r.key.clone()).collect();
                self.pending_play_detail = None;
                self.play_keys(song_keys, 0, json!({"kind": "album", "key": key, "name": album_title}));
            }
            return;
        }
        if self.nav_stack.last().is_none_or(|c| c.key != key) {
            self.nav_stack.push(NavCrumb {
                nav: self.nav_id.clone(),
                key: key.to_owned(),
                title: row.as_ref().map(|item| item.title.clone()).unwrap_or_else(|| key.to_owned()),
            });
        }
        self.selected_key = key.to_owned();
        if matches!(kind.as_str(), "playlist" | "album" | "artist") && self.connected {
            self.backend.command("open", json!({"key": key, "kind": kind}));
            self.loading = true;
        } else if !self.connected && !self.details.contains_key(key) {
            self.notify("error", "Daemon 断连，无法打开");
        }
    }

    fn back(&mut self) {
        if self.immersive {
            self.immersive = false;
            return;
        }
        if self.window_form == "mini" {
            self.window_form = "full".to_owned();
            self.workspace = if self.return_workspace.is_empty() { self.prefs.preferred_workspace.clone() } else { self.return_workspace.clone() };
            return;
        }
        if self.palette_open || self.mode_center_open || self.help_open {
            self.palette_open = false;
            self.mode_center_open = false;
            self.help_open = false;
            return;
        }
        if !self.list_filter.is_empty() {
            self.list_filter.clear();
            return;
        }
        if let Some(previous) = self.nav_stack.pop() {
            self.selected_key = previous.key;
            self.cover_mode = "list".to_owned();
            return;
        }
        if self.searching && self.workspace != "spotlight" {
            self.searching = false;
            self.cover_mode = "list".to_owned();
        }
    }

    fn cover_mode(&mut self, mode: &str) {
        if matches!(mode, "grid" | "list") {
            self.cover_mode = mode.to_owned();
            self.view_mut().cover_mode = mode.to_owned();
        }
    }

    fn toggle_module(&mut self, module: &str) {
        if module.is_empty() {
            return;
        }
        if let Some(idx) = self.prefs.modules.iter().position(|item| item == module) {
            if self.prefs.modules.len() > 1 {
                self.prefs.modules.remove(idx);
                self.prefs.modular_preset = "custom".to_owned();
            }
        } else {
            self.prefs.modules.push(module.to_owned());
            self.prefs.modular_preset = "custom".to_owned();
        }
    }

    fn report_view(&mut self, scroll: i32, focus: &str, expanded: &str) {
        let nav = self.nav_stack.clone();
        let focus = focus.to_owned();
        let expanded = expanded.to_owned();
        let state = self.view_mut();
        state.scroll = scroll;
        if !focus.is_empty() {
            state.focus = focus;
        }
        if !expanded.is_empty() {
            state.expanded = expanded;
        }
        state.nav = nav;
    }

    fn window_width(&mut self, width: i32) {
        if self.locked || !self.auto_adapt {
            return;
        }
        let narrow = width < 760;
        if narrow && self.workspace != "single" && self.window_form == "full" && !self.immersive {
            self.auto_undo = Some(self.mode_blob(String::new()));
            self.workspace = "single".to_owned();
            self.notify("info", "窗口较窄，已临时切换到单栏。可撤销，不会覆盖已锁定的偏好");
        } else if !narrow && self.auto_undo.is_some() {
            if let Some(previous) = self.auto_undo.take() {
                self.restore_mode(previous);
            }
        }
    }

    fn undo_auto(&mut self) {
        if let Some(previous) = self.auto_undo.take() {
            self.restore_mode(previous);
            self.notify("info", "已撤销自动切换");
        }
    }

    fn request_close(&mut self) {
        if self.close_policy == "ask" {
            self.close_prompt = true;
        } else {
            let choice = self.close_policy.clone();
            self.finish_close(&choice);
        }
    }

    fn confirm_close(&mut self, choice: &str) {
        self.close_prompt = false;
        if choice == "ui" || choice == "stop" {
            self.finish_close(choice);
        }
    }

    fn shutdown_daemon(&mut self) {
        if self.connected {
            self.backend.command("shutdown_daemon", json!({}));
        }
        self.connected = false;
        self.playing = false;
        self.daemon_message = "播放已停止".to_owned();
    }

    fn save_combo(&mut self, name: &str) {
        let name = if name.trim().is_empty() {
            format!("组合 {}", self.prefs.custom_modes.len() + 1)
        } else {
            name.trim().to_owned()
        };
        let blob = self.mode_blob(name.clone());
        self.prefs.custom_modes.retain(|item| item.name != name);
        self.prefs.custom_modes.push(blob);
        self.notify("info", &format!("已保存组合模式「{name}」"));
    }

    fn apply_combo(&mut self, name: &str) {
        let Some(blob) = self.prefs.custom_modes.iter().find(|item| item.name == name).cloned() else { return };
        self.push_mode();
        self.restore_mode(blob);
        self.prefs.preferred_workspace = self.workspace.clone();
        self.notify("info", &format!("已切换到「{name}」"));
    }

    fn modular_preset(&mut self, preset_name: &str) {
        if let Some(modules) = preset(preset_name) {
            self.prefs.modular_preset = preset_name.to_owned();
            self.prefs.modules = modules;
        }
    }

    fn modular_move(&mut self, module: &str, direction: &str) {
        let Some(index) = self.prefs.modules.iter().position(|item| item == module) else { return };
        let target = if direction == "down" { index + 1 } else { index.wrapping_sub(1) };
        if target < self.prefs.modules.len() {
            self.prefs.modules.swap(index, target);
            self.prefs.modular_preset = "custom".to_owned();
        }
    }

    fn reset_layout(&mut self) {
        self.prefs.modular_preset = "browse".to_owned();
        self.prefs.modules = preset("browse").unwrap_or_default();
        self.prefs.panel_left = 240;
        self.prefs.panel_right = 320;
        self.prefs.view.clear();
        self.notify("info", "已重置布局，播放队列未改变");
    }

    fn reset_modes(&mut self) {
        self.workspace = "classic".to_owned();
        self.input_mode = "pointer".to_owned();
        self.window_form = "full".to_owned();
        self.immersive = false;
        self.prefs.custom_modes.clear();
        self.prefs.preferred_workspace = "classic".to_owned();
        self.notify("info", "已恢复默认模式，播放队列未改变");
    }

    fn theme(&mut self, r: u8, g: u8, b: u8) {
        let luminance = (0.2126 * f32::from(r) + 0.7152 * f32::from(g) + 0.0722 * f32::from(b)) / 255.0;
        self.theme_fg = if luminance > 0.62 { "#161616" } else { "#f6f3ec" }.to_owned();
        self.theme_bg = format!("#{r:02x}{g:02x}{b:02x}");
        self.theme_accent = if luminance < 0.7 { "#e0a15a" } else { "#8a4b12" }.to_owned();
    }

    fn apply_event(&mut self, event: DaemonEvent) {
        match event {
            DaemonEvent::Connection { connected, message } => {
                let was = self.connected;
                self.connected = connected;
                self.daemon_message = if message.is_empty() {
                    if connected { "已连接".to_owned() } else { "未连接".to_owned() }
                } else {
                    message
                };
                if !connected {
                    self.playing = false;
                    if was {
                        let text = self.daemon_message.clone();
                        self.notify("error", &text);
                    }
                }
            }
            DaemonEvent::Sources { mut sources, message } => {
                for src in &mut sources {
                    src.label = src
                        .label
                        .chars()
                        .filter(|c| !matches!(*c as u32, 0xE000..=0xF8FF | 0xF0000..=0x10FFFF))
                        .collect::<String>()
                        .trim()
                        .to_owned();
                }
                self.sources = sources;
                if !message.is_empty() {
                    self.daemon_message = message.clone();
                    if self.sources.is_empty() {
                        self.notify("error", &message);
                    }
                }
                self.refresh_rec_albums();
            }
            DaemonEvent::Player { playing, position_ms, duration_ms, volume, queue, cursor, mode, current, lyrics } => {
                self.apply_player(playing, position_ms, duration_ms, volume, queue, cursor, mode, current, lyrics);
            }
            DaemonEvent::Library { playlists, partial } => {
                for item in &playlists {
                    self.catalog.insert(item.key.clone(), item.clone());
                }
                if partial {
                    for item in playlists {
                        if let Some(existing) = self.playlists.iter_mut().find(|row| row.key == item.key) {
                            *existing = item;
                        } else {
                            self.playlists.push(item);
                        }
                    }
                } else {
                    self.playlists = playlists;
                }
                if self.rec_albums.is_empty() && self.rec_album_query.is_empty() {
                    self.refresh_rec_albums();
                }
            }
            DaemonEvent::Search { source, kind, query, items, error } => self.apply_search(source, kind, query, items, error),
            DaemonEvent::Detail { key, kind, title, rows, error } => {
                self.loading = false;
                if !error.is_empty() {
                    let has_existing = self.details.get(&key).is_some_and(|(_, existing, _)| !existing.is_empty());
                    if !has_existing {
                        self.details.insert(key, (title, Vec::new(), error.clone()));
                    }
                    self.notify("error", &error);
                } else {
                    let first_cover = rows.iter().find_map(|row| row.cover_url.clone());
                    for row in &rows {
                        self.catalog.insert(row.key.clone(), row.clone());
                    }
                    let entry = self.catalog.entry(key.clone()).or_insert_with(|| Item {
                        key: key.clone(),
                        kind: if kind.is_empty() { "playlist".to_owned() } else { kind.clone() },
                        title: title.clone(),
                        subtitle: String::new(),
                        album: String::new(),
                        source: key.split(':').next().unwrap_or_default().to_owned(),
                        duration_ms: None,
                        unavailable: false,
                        loved: false,
                        cover_url: first_cover.clone(),
                        index: None,
                        current: false,
                    });
                    if entry.cover_url.is_none() && first_cover.is_some() {
                        entry.cover_url = first_cover.clone();
                    }
                    if let Some(pl) = self.playlists.iter_mut().find(|p| p.key == key)
                        && pl.cover_url.is_none()
                    {
                        pl.cover_url = first_cover;
                    }
                    if !title.is_empty() {
                        for crumb in &mut self.nav_stack {
                            if crumb.key == key {
                                crumb.title = title.clone();
                            }
                        }
                    }
                    if kind == "artist"
                        && let Some((existing_title, existing_rows, existing_err)) = self.details.get_mut(&key)
                    {
                        if existing_title.is_empty() && !title.is_empty() {
                            *existing_title = title.clone();
                        }
                        existing_err.clear();
                        for row in rows {
                            if !existing_rows.iter().any(|r| r.key == row.key) {
                                existing_rows.push(row);
                            }
                        }
                    } else {
                        self.details.insert(key.clone(), (title.clone(), rows, String::new()));
                    }
                    if self.pending_play_detail.as_deref() == Some(key.as_str())
                        && let Some((detail_title, detail_rows, _)) = self.details.get(&key)
                    {
                        let song_keys: Vec<String> = detail_rows.iter().filter(|r| r.kind == "song").map(|r| r.key.clone()).collect();
                        if !song_keys.is_empty() {
                            let detail_title = detail_title.clone();
                            self.pending_play_detail = None;
                            self.play_keys(song_keys, 0, json!({"kind": "playlist", "key": key, "name": detail_title}));
                        }
                    }
                    self.refresh_rec_albums();
                }
            }
            DaemonEvent::Loved { keys } => self.loved.extend(keys),
            DaemonEvent::Love { key, loved } => {
                if loved { self.loved.insert(key); } else { self.loved.remove(&key); }
            }
            DaemonEvent::Downloads { items } => {
                for item in &items {
                    if let Some(song) = &item.song {
                        self.catalog.insert(song.key.clone(), song.clone());
                    }
                }
                self.downloads = items;
            }
            DaemonEvent::Pcm { peaks, scope, rms } => {
                self.peaks = peaks;
                self.scope = scope;
                self.rms = rms;
                if !self.reduce_motion {
                    self.waterfall.push(self.peaks.clone());
                    let extra = self.waterfall.len().saturating_sub(48);
                    if extra > 0 {
                        self.waterfall.drain(0..extra);
                    }
                }
            }
            DaemonEvent::Lyrics { key, lines } => {
                self.lyrics_key = key;
                self.lyrics = lines;
            }
            DaemonEvent::Notice { level, text } => self.notify(&level, &text),
            DaemonEvent::PlayError(text) => {
                self.loading = false;
                self.play_error = text.clone();
                self.notify("error", &text);
            }
            DaemonEvent::Loading(loading) => self.loading = loading,
            DaemonEvent::Ready => {}
        }
    }

    fn apply_player(
        &mut self,
        playing: bool,
        position_ms: u64,
        duration_ms: Option<u64>,
        volume: Option<u8>,
        queue: Vec<Item>,
        cursor: Option<usize>,
        mode: String,
        current: Option<Item>,
        lyrics: Vec<LyricLine>,
    ) {
        let cursor_changed = self.cursor != cursor;
        self.queue = queue;
        for song in &self.queue {
            self.catalog.insert(song.key.clone(), song.clone());
        }
        self.cursor = cursor;
        self.current = current;
        if let Some(current) = &self.current {
            self.catalog.insert(current.key.clone(), current.clone());
            remember(&mut self.prefs.recent_plays, &current.key);
            if !lyrics.is_empty() {
                self.lyrics = lyrics;
                self.lyrics_key = current.key.clone();
            } else if self.lyrics_key != current.key {
                self.lyrics.clear();
                self.lyrics_key = current.key.clone();
                if self.connected {
                    self.backend.command("open", json!({"key": current.key, "kind": "song"}));
                }
            }
            self.play_error = if current.unavailable { "歌曲不可用，可以重试或从队列移除".to_owned() } else { String::new() };
        } else {
            self.lyrics.clear();
            self.lyrics_key.clear();
        }
        if !mode.is_empty() {
            self.play_mode = mode;
        }
        self.playing = playing;
        self.position_ms = position_ms;
        self.duration_ms = duration_ms;
        if let Some(volume) = volume {
            self.volume = volume;
        }
        self.loading = false;
        if self.follow_queue && cursor_changed && (self.workspace == "queue" || self.queue_drawer) {
            if let Some(index) = self.cursor {
                if let Some(song) = self.queue.get(index) {
                    self.selected_key = song.key.clone();
                }
            }
        }
        self.refresh_rec_albums();
    }

    fn refresh_rec_albums(&mut self) {
        let (genre_label, query) = self.dominant_playlist_genre_query();
        self.rec_genre = genre_label;
        if !self.connected || self.sources.is_empty() {
            return;
        }
        let ref_song = self
            .current
            .as_ref()
            .or_else(|| self.queue.first())
            .or_else(|| {
                self.prefs
                    .recent_plays
                    .iter()
                    .find_map(|k| self.catalog.get(k))
            });
        let song_key = ref_song.map(|s| s.key.clone()).unwrap_or_default();
        if song_key != self.rec_song_key || query != self.rec_album_query || (self.rec_albums.is_empty() && self.rec_album_query.is_empty()) {
            self.rec_song_key = song_key;
            if query != self.rec_album_query || self.rec_albums.is_empty() {
                self.rec_album_query = query.clone();
                let ids: Vec<String> = self.active_sources().into_iter().map(|s| s.id).collect();
                if !ids.is_empty() && !query.is_empty() {
                    self.backend.command(
                        "search",
                        json!({
                            "query": query,
                            "sources": ids,
                            "kinds": ["album"]
                        }),
                    );
                }
            }
        }
    }

    fn dominant_playlist_genre_query(&self) -> (String, String) {
        let mut tag_scores: BTreeMap<String, f32> = BTreeMap::new();
        for (song, weight) in self.active_playlist_tracks() {
            let (label, query) = infer_song_genre(Some(&song));
            let key = format!("{label}\t{query}");
            *tag_scores.entry(key).or_insert(0.0) += weight;
        }
        if let Some((best_key, _)) = tag_scores
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            && let Some((label, query)) = best_key.split_once('\t')
        {
            return (label.to_owned(), query.to_owned());
        }
        let ref_song = self
            .current
            .as_ref()
            .or_else(|| self.queue.first())
            .or_else(|| self.prefs.recent_plays.iter().find_map(|k| self.catalog.get(k)));
        infer_song_genre(ref_song)
    }

    fn active_playlist_tracks(&self) -> Vec<(Item, f32)> {
        let mut tracks: Vec<(Item, f32)> = Vec::new();
        fn push_weighted(tracks: &mut Vec<(Item, f32)>, item: &Item, w: f32) {
            if item.key.is_empty() || item.title == "没有正在播放" || item.title == "Nothing Playing" {
                return;
            }
            if let Some((_, existing_w)) = tracks.iter_mut().find(|(existing, _)| existing.key == item.key) {
                if w > *existing_w {
                    *existing_w = w;
                }
            } else {
                tracks.push((item.clone(), w));
            }
        }

        // 1. Currently playing track (highest immediate taste anchor)
        if let Some(cur) = &self.current {
            push_weighted(&mut tracks, cur, 2.8);
        }
        // 2. Currently selected track/album in the active view
        if !self.selected_key.is_empty()
            && let Some(sel) = self.catalog.get(&self.selected_key)
            && matches!(sel.kind.as_str(), "song" | "album")
        {
            push_weighted(&mut tracks, sel, 1.6);
        }
        // 3. All tracks in the current play queue (`self.queue`)
        let cur_idx = self.cursor.unwrap_or(0);
        for (idx, song) in self.queue.iter().enumerate() {
            let dist = idx.abs_diff(cur_idx);
            let w = if dist <= 2 { 1.35 } else { 1.0 };
            push_weighted(&mut tracks, song, w);
        }
        // 4. Tracks in the currently opened playlist / album / detail container
        if let Some(top) = self.nav_stack.last()
            && let Some((_, detail_rows, _)) = self.details.get(&top.key)
        {
            for song in detail_rows.iter().filter(|r| r.kind == "song").take(40) {
                push_weighted(&mut tracks, song, 0.9);
            }
        }
        // 5. Loved / favourite tracks & recent plays as baseline taste profile
        for loved_key in &self.loved {
            if let Some(song) = self.catalog.get(loved_key) {
                push_weighted(&mut tracks, song, 0.65);
            }
        }
        for recent_key in &self.prefs.recent_plays {
            if let Some(song) = self.catalog.get(recent_key) {
                push_weighted(&mut tracks, song, 0.55);
            }
        }
        // 6. Fallback to library / playlist items if queue & history are empty
        if tracks.is_empty() {
            for pl in self.playlists.iter().take(16) {
                push_weighted(&mut tracks, pl, 0.45);
            }
            for item in self.catalog.values().filter(|i| matches!(i.kind.as_str(), "song" | "album")).take(24) {
                push_weighted(&mut tracks, item, 0.4);
            }
        }
        tracks
    }

    fn collect_candidate_albums(&self) -> Vec<(Item, Vec<Item>)> {
        let mut pool: Vec<(Item, Vec<Item>)> = Vec::new();
        let mut index_by_dedup: BTreeMap<String, usize> = BTreeMap::new();
        let source_filter = self.source_filter.as_deref();

        fn upsert_album(
            pool: &mut Vec<(Item, Vec<Item>)>,
            index_by_dedup: &mut BTreeMap<String, usize>,
            source_filter: Option<&str>,
            album: Item,
            associated_tracks: Vec<Item>,
        ) {
            if album.key.is_empty() || album.title.trim().is_empty() {
                return;
            }
            if let Some(sf) = source_filter
                && !sf.is_empty()
                && !album.source.is_empty()
                && album.source != sf
            {
                return;
            }
            let norm_title = normalize_album_key(&album.title);
            if norm_title.is_empty() {
                return;
            }
            let dedup_key = norm_title;
            if let Some(&idx) = index_by_dedup.get(&dedup_key) {
                let (existing_album, existing_tracks) = &mut pool[idx];
                let existing_has_cover = existing_album.cover_url.as_ref().is_some_and(|u| !u.is_empty());
                let new_has_cover = album.cover_url.as_ref().is_some_and(|u| !u.is_empty());
                let existing_is_synth = existing_album.key.starts_with("local_album:");
                let new_is_synth = album.key.starts_with("local_album:");
                if (!existing_has_cover && new_has_cover) || (existing_is_synth && !new_is_synth) {
                    let mut merged = album;
                    if merged.cover_url.is_none() {
                        merged.cover_url = existing_album.cover_url.clone();
                    }
                    *existing_album = merged;
                } else if existing_album.cover_url.is_none() && new_has_cover {
                    existing_album.cover_url = album.cover_url;
                }
                for tr in associated_tracks {
                    if !existing_tracks.iter().any(|e| e.key == tr.key) {
                        existing_tracks.push(tr);
                    }
                }
            } else {
                let idx = pool.len();
                index_by_dedup.insert(dedup_key, idx);
                pool.push((album, associated_tracks));
            }
        }

        // 1. Explicit albums from `rec_albums`, `search_groups["album"]`, `search_parts`, `catalog`, and `playlists`
        for item in &self.rec_albums {
            if item.kind == "album" {
                upsert_album(&mut pool, &mut index_by_dedup, source_filter, item.clone(), Vec::new());
            }
        }
        if let Some(albums) = self.search_groups.get("album") {
            for item in albums {
                upsert_album(&mut pool, &mut index_by_dedup, source_filter, item.clone(), Vec::new());
            }
        }
        for ((_, kind), items) in &self.search_parts {
            if kind == "album" {
                for item in items {
                    upsert_album(&mut pool, &mut index_by_dedup, source_filter, item.clone(), Vec::new());
                }
            }
        }
        for item in self.catalog.values() {
            if item.kind == "album" {
                let detail_tracks = self
                    .details
                    .get(&item.key)
                    .map(|(_, rows, _)| rows.clone())
                    .unwrap_or_default();
                upsert_album(&mut pool, &mut index_by_dedup, source_filter, item.clone(), detail_tracks);
            }
        }
        for item in &self.playlists {
            if item.kind == "album" {
                upsert_album(&mut pool, &mut index_by_dedup, source_filter, item.clone(), Vec::new());
            }
        }

        // 2. Synthesize local album documents from tracks in `catalog`, `queue`, `details`, and `downloads`
        let mut tracks_by_album: BTreeMap<String, Vec<Item>> = BTreeMap::new();
        for song in self.catalog.values().chain(self.queue.iter()) {
            if song.kind == "song" {
                let norm = normalize_album_key(&song.album);
                if !norm.is_empty() {
                    let bucket = tracks_by_album.entry(norm).or_default();
                    if !bucket.iter().any(|s| s.key == song.key) {
                        bucket.push(song.clone());
                    }
                }
            }
        }
        for (_, detail_rows, _) in self.details.values() {
            for song in detail_rows {
                if song.kind == "song" {
                    let norm = normalize_album_key(&song.album);
                    if !norm.is_empty() {
                        let bucket = tracks_by_album.entry(norm).or_default();
                        if !bucket.iter().any(|s| s.key == song.key) {
                            bucket.push(song.clone());
                        }
                    }
                }
            }
        }
        for (norm_album, tracks) in tracks_by_album {
            let Some(first) = tracks.first() else { continue };
            let album_title = first.album.trim().to_owned();
            let cover = tracks.iter().find_map(|t| t.cover_url.clone());
            let synth = Item {
                key: format!("local_album:{norm_album}"),
                kind: "album".to_owned(),
                title: album_title.clone(),
                subtitle: first.subtitle.clone(),
                album: album_title,
                source: first.source.clone(),
                duration_ms: None,
                unavailable: false,
                loved: false,
                cover_url: cover,
                index: None,
                current: false,
            };
            upsert_album(&mut pool, &mut index_by_dedup, source_filter, synth, tracks);
        }

        // 3. Last-resort fallback: if fewer than 4 albums exist in the entire session, include playlists
        if pool.len() < 4 {
            for pl in &self.playlists {
                if pl.kind == "playlist" {
                    let detail_tracks = self
                        .details
                        .get(&pl.key)
                        .map(|(_, rows, _)| rows.clone())
                        .unwrap_or_default();
                    upsert_album(&mut pool, &mut index_by_dedup, source_filter, pl.clone(), detail_tracks);
                }
            }
        }

        pool
    }

    /// Pure-local Metadata & Tag TF-IDF + Cosine Similarity Recommender for the 2x2 Album Wall:
    ///
    /// $$\text{Document} = [\text{Artist}] \times w_1 + [\text{Genres/Tags}] \times w_2 + [\text{Release Decade}] \times w_3$$
    /// $$\text{Score}(A_i) = \frac{V_{\text{playlist}} \cdot V_{A_i}}{\Vert V_{\text{playlist}} \Vert_2 \, \Vert V_{A_i} \Vert_2}$$
    fn recommended_album_wall(&self) -> Vec<Item> {
        let candidates = self.collect_candidate_albums();
        if candidates.is_empty() {
            return Vec::new();
        }

        let cur_album_norm = self
            .current
            .as_ref()
            .map(|c| normalize_album_key(&c.album))
            .unwrap_or_default();

        // Step 1: Aggregate all songs' Tags / Style tags / Artists / Release Decades in the active playlist
        // to construct the Playlist Taste Profile term frequencies `playlist_tf`.
        let playlist_tracks = self.active_playlist_tracks();
        let mut playlist_tf: BTreeMap<String, f32> = BTreeMap::new();
        for (track, track_w) in &playlist_tracks {
            let doc = extract_document_features(track, &[]);
            for (term, weight) in doc {
                *playlist_tf.entry(term).or_insert(0.0) += track_w * weight;
            }
        }

        // Step 2: Extract document features for every candidate album `A_i` in the local/known album pool,
        // and compute document frequency `df(t)` across the corpus.
        let mut album_docs: Vec<BTreeMap<String, f32>> = Vec::with_capacity(candidates.len());
        let mut df: BTreeMap<String, usize> = BTreeMap::new();
        for (album, tracks) in &candidates {
            let doc = extract_document_features(album, tracks);
            for term in doc.keys() {
                *df.entry(term.clone()).or_insert(0) += 1;
            }
            album_docs.push(doc);
        }

        // Smoothed IDF (scikit-learn TfidfTransformer formula): idf(t) = ln((1 + N) / (1 + df(t))) + 1.0
        let n_docs = candidates.len() as f32;
        let idf = |term: &str| -> f32 {
            let doc_freq = *df.get(term).unwrap_or(&0) as f32;
            ((1.0 + n_docs) / (1.0 + doc_freq)).ln() + 1.0
        };

        // Build TF-IDF vector V_playlist (with sublinear TF scaling) and compute ||V_playlist||_2
        let mut v_playlist: BTreeMap<String, f32> = BTreeMap::new();
        let mut norm_playlist_sq: f32 = 0.0;
        for (term, raw_tf) in &playlist_tf {
            if *raw_tf <= 0.0 {
                continue;
            }
            let cat_w = term_category_weight(term);
            let sublinear_tf = if *raw_tf >= 1.0 { 1.0 + raw_tf.ln() } else { *raw_tf };
            let val = sublinear_tf * cat_w * idf(term);
            norm_playlist_sq += val * val;
            v_playlist.insert(term.clone(), val);
        }
        let norm_playlist = norm_playlist_sq.sqrt();

        // Step 3: Compute TF-IDF vector V_{A_i} and Cosine Similarity Score(A_i) for each candidate album
        struct ScoredAlbum {
            item: Item,
            score: f32,
            has_cover: bool,
            is_current_album: bool,
            is_synthesized: bool,
        }

        let mut scored: Vec<ScoredAlbum> = candidates
            .into_iter()
            .zip(album_docs)
            .map(|((album, _), doc)| {
                let mut dot: f32 = 0.0;
                let mut norm_album_sq: f32 = 0.0;
                for (term, raw_tf) in &doc {
                    if *raw_tf <= 0.0 {
                        continue;
                    }
                    let cat_w = term_category_weight(term);
                    let sublinear_tf = if *raw_tf >= 1.0 { 1.0 + raw_tf.ln() } else { *raw_tf };
                    let v_ai = sublinear_tf * cat_w * idf(term);
                    norm_album_sq += v_ai * v_ai;
                    if let Some(&v_pl) = v_playlist.get(term) {
                        dot += v_pl * v_ai;
                    }
                }
                let norm_album = norm_album_sq.sqrt();
                let cosine = if norm_playlist > 1e-6 && norm_album > 1e-6 {
                    dot / (norm_playlist * norm_album)
                } else {
                    0.0
                };
                let has_cover = album.cover_url.as_ref().is_some_and(|u| !u.is_empty());
                let norm_alb = normalize_album_key(&album.title);
                let is_current_album = !cur_album_norm.is_empty() && norm_alb == cur_album_norm;
                let is_synthesized = album.key.starts_with("local_album:");
                ScoredAlbum {
                    item: album,
                    score: cosine,
                    has_cover,
                    is_current_album,
                    is_synthesized,
                }
            })
            .collect();

        // Sort by:
        // 1. Not the exact currently playing album (so the 2x2 wall recommends 4 other albums when possible)
        // 2. Explicit album containers before synthesized single-track fallback albums
        // 3. Has valid cover art
        // 4. Cosine similarity Score(A_i) descending
        scored.sort_by(|a, b| {
            a.is_current_album
                .cmp(&b.is_current_album)
                .then_with(|| a.is_synthesized.cmp(&b.is_synthesized))
                .then_with(|| b.has_cover.cmp(&a.has_cover))
                .then_with(|| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal))
        });

        scored.into_iter().take(4).map(|s| s.item).collect()
    }

    fn apply_search(&mut self, source: String, kind: String, query: String, items: Vec<Item>, error: String) {
        for item in &items {
            self.catalog.insert(item.key.clone(), item.clone());
        }
        if kind == "album" && error.is_empty() {
            for item in &items {
                if item.kind == "album" {
                    if let Some(existing) = self.rec_albums.iter_mut().find(|existing| existing.key == item.key) {
                        *existing = item.clone();
                    } else {
                        self.rec_albums.insert(0, item.clone());
                    }
                }
            }
            self.rec_albums.truncate(48);
        }
        if !query.is_empty() && query != self.search_query {
            return;
        }
        if self.search_stale {
            self.search_parts.clear();
            self.search_errors.clear();
            self.search_stale = false;
        }
        self.search_pending.remove(&(source.clone(), kind.clone()));
        if error.is_empty() {
            self.search_errors.remove(&(source.clone(), kind.clone()));
            self.search_parts.insert((source, kind), items);
        } else {
            self.search_errors.insert((source.clone(), kind.clone()), error);
            self.search_parts.insert((source, kind), Vec::new());
        }
        self.rebuild_search();
    }

    fn rebuild_search(&mut self) {
        let mut groups = empty_groups();
        for ((_source, kind), items) in &self.search_parts {
            let Some(bucket) = groups.get_mut(kind) else { continue };
            for item in items {
                if !bucket.iter().any(|existing| existing.key == item.key) {
                    bucket.push(item.clone());
                }
            }
        }
        self.search_groups = groups;
    }

    fn compose(&self) -> (Vec<Item>, String, Vec<String>, String) {
        if self.workspace == "queue" {
            return (self.queue_rows(), "播放队列".to_owned(), vec!["队列".to_owned()], "queue".to_owned());
        }
        if let Some(top) = self.nav_stack.last() {
            let root_crumb = if self.workspace == "spotlight" || self.searching {
                "搜索".to_owned()
            } else if self.workspace == "covers" {
                "封面".to_owned()
            } else {
                NAV.iter()
                    .find(|(id, _)| *id == self.nav_id)
                    .map(|(_, title)| (*title).to_owned())
                    .unwrap_or_else(|| "曲库".to_owned())
            };
            let mut crumbs = vec![root_crumb];
            for crumb in &self.nav_stack {
                crumbs.push(crumb.title.clone());
            }
            if let Some(source_id) = top.key.strip_prefix("source:") {
                let mut rows: Vec<Item> = self.playlists.iter().filter(|r| r.source == source_id).cloned().collect();
                for item in self.catalog.values() {
                    if item.source == source_id
                        && matches!(item.kind.as_str(), "playlist" | "album" | "song")
                        && !rows.iter().any(|r| r.key == item.key)
                    {
                        rows.push(item.clone());
                    }
                }
                return (rows, top.title.clone(), crumbs, "tracks".to_owned());
            }
            if let Some((title, rows, _)) = self.details.get(&top.key) {
                let title = if title.is_empty() { top.title.clone() } else { title.clone() };
                if let Some(last) = crumbs.last_mut() {
                    *last = title.clone();
                }
                return (rows.clone(), title, crumbs, "tracks".to_owned());
            }
            return (Vec::new(), top.title.clone(), crumbs, "tracks".to_owned());
        }
        if self.workspace == "spotlight" {
            return (self.search_rows(), "搜索".to_owned(), vec!["搜索".to_owned()], "search".to_owned());
        }
        if self.searching && !self.search_query.is_empty() {
            return (
                self.search_rows(),
                format!("搜索：{}", self.search_query),
                vec!["搜索".to_owned(), self.search_query.clone()],
                "search".to_owned(),
            );
        }
        if self.workspace == "covers" {
            let kind = if self.cover_mode == "grid" { "grid" } else { "tracks" };
            return (self.cover_rows(), "封面墙".to_owned(), vec!["封面".to_owned()], kind.to_owned());
        }
        match self.nav_id.as_str() {
            "playlists" => (
                self.playlists
                    .iter()
                    .filter(|row| self.source_filter.as_ref().is_none_or(|s| &row.source == s))
                    .cloned()
                    .collect(),
                "歌单".to_owned(),
                vec!["歌单".to_owned()],
                "tracks".to_owned(),
            ),
            "loved" => (
                self.loved
                    .iter()
                    .filter_map(|key| self.catalog.get(key).cloned())
                    .filter(|row| self.source_filter.as_ref().is_none_or(|s| &row.source == s))
                    .collect(),
                "收藏".to_owned(),
                vec!["收藏".to_owned()],
                "tracks".to_owned(),
            ),
            "downloads" => (
                self.downloads
                    .iter()
                    .filter_map(|item| {
                        let mut song = item.song.clone()?;
                        let status = if let Some(failure) = &item.failure {
                            format!("失败：{failure}")
                        } else {
                            item.status.clone()
                        };
                        if !status.is_empty() {
                            song.subtitle = format!("{} · {status}", song.subtitle);
                        }
                        Some(song)
                    })
                    .collect(),
                "下载".to_owned(),
                vec!["下载".to_owned()],
                "tracks".to_owned(),
            ),
            "queue" => (
                self.queue_rows(),
                "播放队列".to_owned(),
                vec!["队列".to_owned()],
                "queue".to_owned(),
            ),
            "settings" => (self.settings_rows(), "设置".to_owned(), vec!["设置".to_owned()], "settings".to_owned()),
            _ => (self.library_rows(), "曲库".to_owned(), vec!["曲库".to_owned()], "tracks".to_owned()),
        }
    }

    fn library_rows(&self) -> Vec<Item> {
        let mut rows = Vec::new();
        for source in &self.sources {
            if self.source_filter.as_ref().is_some_and(|s| s != &source.id) {
                continue;
            }
            let label = if source.label.is_empty() { &source.id } else { &source.label };
            let status = if !source.available {
                "Unavailable"
            } else if !source.logged_in {
                "Not logged in"
            } else if source.searchable.is_empty() {
                "Local / Basic Source"
            } else {
                "Connected · Searchable"
            };
            rows.push(Item::source_row(&source.id, label, status));
        }
        for pl in &self.playlists {
            if self.source_filter.as_ref().is_none_or(|s| &pl.source == s) {
                rows.push(pl.clone());
            }
        }
        if let Some(sf) = &self.source_filter {
            for item in self.catalog.values() {
                if &item.source == sf
                    && matches!(item.kind.as_str(), "playlist" | "album" | "song")
                    && !rows.iter().any(|r| r.key == item.key)
                {
                    rows.push(item.clone());
                }
            }
        }
        rows
    }

    fn cover_rows(&self) -> Vec<Item> {
        let mut rows: Vec<Item> = self
            .playlists
            .iter()
            .filter(|row| self.source_filter.as_ref().is_none_or(|s| &row.source == s))
            .cloned()
            .collect();
        for item in self.catalog.values() {
            if matches!(item.kind.as_str(), "album" | "playlist")
                && self.source_filter.as_ref().is_none_or(|s| &item.source == s)
                && !rows.iter().any(|row| row.key == item.key)
            {
                rows.push(item.clone());
            }
        }
        let expanded = &self.view().expanded;
        if !expanded.is_empty()
            && let Some((_, detail_rows, _)) = self.details.get(expanded)
            && let Some(pos) = rows.iter().position(|r| &r.key == expanded)
        {
            for (offset, track) in detail_rows.iter().enumerate() {
                rows.insert(pos + 1 + offset, track.clone());
            }
        }
        rows
    }

    fn search_rows(&self) -> Vec<Item> {
        let kinds: &[&str] = match self.search_tab.as_str() {
            "song" => &["song"],
            "artist" => &["artist"],
            "album" => &["album"],
            "playlist" => &["playlist"],
            _ => &["song", "artist", "album", "playlist"],
        };
        kinds
            .iter()
            .flat_map(|kind| self.search_groups.get(*kind).cloned().unwrap_or_default())
            .filter(|row| self.source_filter.as_ref().is_none_or(|s| &row.source == s))
            .collect()
    }

    fn settings_rows(&self) -> Vec<Item> {
        let mut items = self
            .daemon_config
            .settings_items(&self.daemon_message, &self.daemon_lua_path);
        let mk_ui = |key: &str, title: &str, val: &str, hint: &str| Item {
            key: key.to_owned(),
            kind: "setting".to_owned(),
            title: title.to_owned(),
            subtitle: val.to_owned(),
            album: hint.to_owned(),
            source: "ui".to_owned(),
            ..Default::default()
        };
        items.push(mk_ui(
            "settings:close",
            "GUI Window · Close Button Policy",
            &self.close_policy,
            "ask / ui (Tray) / stop (Shutdown Daemon) · Click to cycle",
        ));
        items.push(mk_ui(
            "settings:motion",
            "GUI Window · Reduce Motion & Animations",
            if self.reduce_motion { "On" } else { "Off" },
            "Disable spring-damper animations",
        ));
        items
    }

    fn queue_rows(&self) -> Vec<Item> {
        self.queue.iter().enumerate().map(|(index, song)| {
            let mut row = song.clone();
            row.index = Some(index);
            row.current = self.cursor == Some(index);
            if row.unavailable {
                row.subtitle = format!("{} · 不可用", row.subtitle);
            }
            row
        }).collect()
    }

    fn lyric_view(&self) -> Vec<LyricView> {
        let mut view: Vec<LyricView> = self.lyrics.iter().map(|line| {
            let mut text = line.text.clone();
            let mut sub_text = String::new();
            if self.lyric_track == "translation" {
                if let Some(value) = &line.translation {
                    text = value.clone();
                    if line.text != text {
                        sub_text = line.text.clone();
                    }
                }
            } else if self.lyric_track == "romanization" {
                if let Some(value) = &line.romanization {
                    text = value.clone();
                    sub_text = line.translation.clone().unwrap_or_else(|| line.text.clone());
                } else if let Some(value) = &line.translation {
                    sub_text = value.clone();
                }
            } else if let Some(value) = &line.translation {
                if !value.trim().is_empty() && value.trim() != text.trim() {
                    sub_text = value.clone();
                }
            }
            let active = line.t.is_some_and(|start| start <= self.position_ms);
            LyricView { t: line.t, text, sub_text, active, words: line.words.clone() }
        }).collect();
        if let Some(index) = view.iter().rposition(|line| line.active) {
            for (i, line) in view.iter_mut().enumerate() {
                line.active = i == index;
            }
        }
        view
    }

    fn empty_reason(&self) -> String {
        let (rows, _title, _crumbs, kind) = self.compose();
        let filtered: Vec<_> = rows.into_iter().filter(|row| fuzzy_match(&self.list_filter, &format!("{} {}", row.title, row.subtitle))).collect();
        if !filtered.is_empty() {
            return String::new();
        }
        if !self.connected {
            return "Daemon 断连，界面暂时无法获取状态".to_owned();
        }
        if kind == "search" {
            let summary = self.search_summary();
            return if summary.is_empty() { "输入关键词开始搜索".to_owned() } else { summary };
        }
        if self.nav_id == "loved" {
            return "还没有收藏".to_owned();
        }
        if self.nav_id == "queue" || kind == "queue" {
            return "队列还是空的".to_owned();
        }
        if !self.list_filter.is_empty() {
            return "当前列表没有匹配".to_owned();
        }
        "这里还是空的".to_owned()
    }

    fn search_summary(&self) -> String {
        if self.search_query.is_empty() {
            return String::new();
        }
        if !self.search_pending.is_empty() {
            return "正在更新搜索结果".to_owned();
        }
        let total: usize = self.search_groups.values().map(Vec::len).sum();
        if !self.search_errors.is_empty() && total > 0 {
            "部分音源失败，其余结果仍然可用".to_owned()
        } else if !self.search_errors.is_empty() {
            "音源不可用或未登录".to_owned()
        } else if total == 0 {
            "确实无结果".to_owned()
        } else {
            String::new()
        }
    }

    fn scheme_rows(&self) -> Vec<SchemeRow> {
        let current = self.active_scheme();
        SCHEMES.iter().filter(|(id, title, purpose, _)| fuzzy_match(&self.mode_query, &format!("{title} {purpose}")) || id.is_empty()).map(|(id, title, purpose, _)| SchemeRow {
            id: (*id).to_owned(),
            title: (*title).to_owned(),
            purpose: (*purpose).to_owned(),
            current: *id == current,
        }).collect()
    }

    fn commands(&self) -> Vec<CommandRow> {
        let is_cmd_only = self.palette_query.trim_start().starts_with('>');
        let text = self.palette_query.trim_start().strip_prefix('>').unwrap_or(&self.palette_query).trim();
        let mut commands = vec![
            ("toggle_play", "播放或暂停"),
            ("next", "下一首"),
            ("prev", "上一首"),
            ("enqueue", "加入队列"),
            ("love", "收藏"),
            ("download", "下载"),
            ("reveal", "回到正在播放"),
            ("return_mode", "返回上一个模式"),
            ("toggle_immersive", "切换沉浸模式"),
            ("cycle_viz", "切换频谱样式"),
            ("cycle_mode", "切换循环模式"),
            ("queue_dedupe", "队列去重"),
            ("queue_clear_unavailable", "清理不可用曲目"),
            ("queue_clear", "清空播放队列"),
            ("connect", "重连 Daemon"),
            ("launch_daemon", "启动 Daemon"),
            ("help", "查看快捷键"),
        ];
        let mut owned: Vec<(String, String)> = commands.drain(..).map(|(id, title)| (id.to_owned(), title.to_owned())).collect();
        for (id, title, _purpose, _layer) in SCHEMES {
            owned.push((format!("scheme:{id}"), format!("切换模式 {title}")));
        }
        for (preset_id, preset_title) in [
            ("browse", "浏览型工作台"),
            ("queue", "队列型工作台"),
            ("lyrics", "歌词型工作台"),
            ("viz", "频谱型工作台"),
        ] {
            owned.push((format!("modular_preset:{preset_id}"), format!("模块预设：{preset_title}")));
        }
        let mut rows: Vec<CommandRow> = owned
            .into_iter()
            .filter(|(_id, title)| fuzzy_match(text, title))
            .map(|(id, title)| CommandRow { id, title })
            .collect();
        if !is_cmd_only && !text.is_empty() {
            rows.insert(
                0,
                CommandRow {
                    id: format!("search_now:{text}"),
                    title: format!("搜索「{text}」"),
                },
            );
            let mut matched_items: Vec<CommandRow> = self
                .catalog
                .values()
                .filter(|item| matches!(item.kind.as_str(), "song" | "album" | "artist" | "playlist"))
                .filter(|item| fuzzy_match(text, &format!("{} {}", item.title, item.subtitle)))
                .take(10)
                .map(|item| CommandRow {
                    id: format!("play_item:{}", item.key),
                    title: format!("打开/播放：{} · {}", item.title, item.subtitle),
                })
                .collect();
            rows.append(&mut matched_items);
        }
        rows
    }

    fn active_scheme(&self) -> String {
        if self.window_form == "mini" {
            "mini".to_owned()
        } else if self.immersive {
            "immersive".to_owned()
        } else if self.input_mode == "keyboard" {
            "keyboard".to_owned()
        } else if self.input_mode == "touch" {
            "touch".to_owned()
        } else {
            self.workspace.clone()
        }
    }

    fn apply_scheme(&mut self, scheme: &str, record_preferred: bool) {
        match scheme {
            "keyboard" => self.input_mode = "keyboard".to_owned(),
            "touch" => self.input_mode = "touch".to_owned(),
            "mini" => {
                self.return_workspace = self.workspace.clone();
                self.immersive = false;
                self.window_form = "mini".to_owned();
            }
            "immersive" => {
                self.return_workspace = self.workspace.clone();
                if self.window_form == "mini" {
                    self.window_form = "full".to_owned();
                }
                self.immersive = true;
            }
            workspace if WORKSPACES.contains(&workspace) => {
                self.immersive = false;
                if self.window_form == "mini" {
                    self.window_form = "full".to_owned();
                }
                self.workspace = workspace.to_owned();
                if record_preferred {
                    self.prefs.preferred_workspace = workspace.to_owned();
                }
            }
            _ => {}
        }
        self.auto_undo = None;
    }

    fn push_mode(&mut self) {
        self.mode_stack.push(self.mode_blob(String::new()));
        if self.mode_stack.len() > 12 {
            let extra = self.mode_stack.len() - 12;
            self.mode_stack.drain(0..extra);
        }
    }

    fn mode_blob(&self, name: String) -> ModeBlob {
        ModeBlob {
            name,
            workspace: self.workspace.clone(),
            input_mode: self.input_mode.clone(),
            window_form: self.window_form.clone(),
            immersive: self.immersive,
            cover_mode: self.cover_mode.clone(),
            view: self.view().clone(),
        }
    }

    fn restore_mode(&mut self, blob: ModeBlob) {
        self.workspace = blob.workspace;
        self.input_mode = blob.input_mode;
        self.window_form = blob.window_form;
        self.immersive = blob.immersive;
        self.cover_mode = blob.cover_mode;
        self.prefs.view.insert(self.workspace.clone(), blob.view);
    }

    fn view(&self) -> &ViewState {
        self.prefs.view.get(&self.workspace).unwrap_or(&EMPTY_VIEW)
    }

    fn view_mut(&mut self) -> &mut ViewState {
        self.prefs.view.entry(self.workspace.clone()).or_default()
    }

    fn active_sources(&self) -> Vec<SourceInfo> {
        let available: Vec<SourceInfo> = self
            .sources
            .iter()
            .filter(|source| source.available)
            .filter(|source| self.source_filter.as_ref().is_none_or(|id| id == &source.id))
            .cloned()
            .collect();
        let searchable: Vec<SourceInfo> = available.iter().filter(|source| !source.searchable.is_empty()).cloned().collect();
        if !searchable.is_empty() {
            searchable
        } else if !available.is_empty() {
            available
        } else if let Some(sf) = &self.source_filter
            && !sf.is_empty()
        {
            vec![SourceInfo {
                id: sf.clone(),
                label: sf.to_uppercase(),
                available: true,
                logged_in: true,
                searchable: vec!["song".to_owned(), "artist".to_owned(), "album".to_owned(), "playlist".to_owned()],
            }]
        } else {
            Vec::new()
        }
    }

    fn playable_context(&self) -> Vec<Item> {
        if let Some(top) = self.nav_stack.last() {
            if let Some((_, rows, _)) = self.details.get(&top.key) {
                let songs: Vec<Item> = rows.iter().filter(|row| row.kind == "song").cloned().collect();
                if !songs.is_empty() {
                    return songs;
                }
            }
        }
        if self.workspace == "spotlight" || (self.searching && !self.search_query.is_empty()) {
            if let Some(songs) = self.search_groups.get("song") {
                if !songs.is_empty() {
                    return songs.clone();
                }
            }
        }
        if self.workspace == "queue" || self.nav_id == "queue" {
            return self.queue.clone();
        }
        self.compose().0.into_iter().filter(|row| row.kind == "song").collect()
    }

    fn context_for(&self, row: &Item) -> Value {
        if self.workspace == "spotlight" || !self.search_query.is_empty() {
            json!({"kind": "search", "name": self.search_query})
        } else if let Some(top) = self.nav_stack.last() {
            json!({"kind": "playlist", "key": top.key, "name": top.title})
        } else {
            json!({"kind": "manual", "name": row.title})
        }
    }

    fn play_keys(&mut self, keys: Vec<String>, index: usize, context: Value) {
        if !self.connected {
            self.notify("error", "Daemon 断连，无法播放");
            return;
        }
        self.loading = true;
        self.play_error.clear();
        self.backend.command("play", json!({"keys": keys, "index": index, "context": context}));
    }

    fn transport(&mut self, name: &str) {
        if !self.connected {
            self.notify("error", "Daemon 断连，无法控制播放");
            return;
        }
        self.backend.command(name, json!({}));
    }

    fn find_row(&self, key: &str) -> Option<Item> {
        self.playlists
            .iter()
            .chain(self.queue.iter())
            .chain(self.rec_albums.iter())
            .find(|row| row.key == key)
            .cloned()
            .or_else(|| self.search_groups.values().flatten().find(|row| row.key == key).cloned())
            .or_else(|| {
                if key.starts_with("local_album:") {
                    self.recommended_album_wall().into_iter().find(|row| row.key == key)
                } else {
                    None
                }
            })
    }

    fn notify(&mut self, level: &str, text: &str) {
        if text.is_empty() || self.notices.iter().any(|notice| notice.text == text) {
            return;
        }
        self.notices.push(Notice { id: self.notice_seq, level: level.to_owned(), text: text.to_owned() });
        self.notice_seq += 1;
        let extra = self.notices.len().saturating_sub(6);
        if extra > 0 {
            self.notices.drain(0..extra);
        }
    }

    fn finish_close(&mut self, choice: &str) {
        if choice == "stop" {
            self.backend.command("shutdown_daemon", json!({}));
            self.playing = false;
            self.daemon_message = "播放已停止".to_owned();
        }
        self.quit = true;
        self.persist();
    }

    fn persist(&mut self) {
        self.prefs.version = 1;
        self.prefs.input_mode = self.input_mode.clone();
        self.prefs.window_form = self.window_form.clone();
        self.prefs.immersive = self.immersive;
        self.prefs.lyric_track = self.lyric_track.clone();
        self.prefs.viz_style = self.viz_style.clone();
        self.prefs.reduce_motion = self.reduce_motion;
        self.prefs.auto_adapt = self.auto_adapt;
        self.prefs.locked = self.locked;
        self.prefs.close_policy = self.close_policy.clone();
        self.prefs.follow_queue = self.follow_queue;
        self.prefs.cover_mode = self.cover_mode.clone();
        self.prefs.source_filter = self.source_filter.clone();
        self.prefs.shortcuts = self.shortcuts.clone();
        prefs::save(&self.path, &self.prefs);
    }
}

fn infer_song_genre(song: Option<&Item>) -> (String, String) {
    let Some(item) = song else {
        return ("Indie Rock".to_owned(), "Indie Rock".to_owned());
    };
    let blob = format!("{} {} {}", item.title, item.subtitle, item.album).to_lowercase();
    let has_any = |words: &[&str]| words.iter().any(|w| blob.contains(w));

    if has_any(&[
        "my bloody valentine", "loveless", "slowdive", "cocteau twins", "lush",
        "beach house", "mazzy star", "alvvays", "shoegaze", "dream pop",
        "galaxie 500", "duster", "sweet trip", "kinoko teikoku", "きのこ帝国",
    ]) {
        return ("Shoegaze".to_owned(), "Shoegaze".to_owned());
    }
    if has_any(&[
        "oasis", "blur", "the stone roses", "pulp", "suede", "the verve",
        "the smiths", "morrissey", "elastica", "supergrass", "britpop",
    ]) {
        return ("Britpop".to_owned(), "Britpop".to_owned());
    }
    if has_any(&[
        "radiohead", "ok computer", "the bends", "kid a", "in rainbows",
        "thom yorke", "the smile", "muse", "talk talk", "sigur rós",
        "sigur ros", "godspeed you", "mogwai", "explosions in the sky",
        "post-rock", "art rock",
    ]) {
        return ("Alternative Rock".to_owned(), "Alternative Rock".to_owned());
    }
    if has_any(&[
        "joy division", "new order", "the cure", "bauhaus", "siouxsie",
        "interpol", "molchat doma", "talking heads", "television",
        "the velvet underground", "sonic youth", "pixies", "post-punk", "new wave",
    ]) {
        return ("Post-Punk".to_owned(), "Post-Punk".to_owned());
    }
    if has_any(&[
        "the beatles", "pink floyd", "led zeppelin", "david bowie", "queen",
        "the rolling stones", "the doors", "fleetwood mac", "the who",
        "jimi hendrix", "king crimson", "nirvana", "pearl jam", "soundgarden",
        "alice in chains", "red hot chili peppers", "guns n' roses", "ac/dc",
    ]) {
        return ("Classic Rock".to_owned(), "Classic Rock".to_owned());
    }
    if has_any(&[
        "the strokes", "arctic monkeys", "the national", "wilco", "pavement",
        "yo la tengo", "elliott smith", "car seat headrest", "big thief",
        "phoebe bridgers", "mitski", "arcade fire", "vampire weekend",
        "tame impala", "mac demarco", "the 1975", "cigarettes after sex",
        "clairo", "men i trust", "boygenius", "fontaines d.c.",
    ]) {
        return ("Indie Rock".to_owned(), "Indie Rock".to_owned());
    }
    if has_any(&[
        "万能青年旅店", "草东没有派对", "腰乐队", "重塑雕像的权利", "刺猬",
        "carsick cars", "海朋森", "声音玩具", "木推瓜", "pk14", "达达",
        "新裤子", "逃跑计划", "痛仰", "落日飞车", "sunset rollercoaster",
        "deca joins", "傻子与白痴", "椅子乐团", "老王乐队", "康姆士", "棱镜",
        "回春丹", "蛙池", "九连真人", "五条人", "窦唯", "崔健", "唐朝",
        "黑豹", "张楚", "何勇", "郑钧", "许巍", "朴树", "汪峰", "盘尼西林",
        "橘子海", "夏日入侵企画", "温和治疗", "独立摇滚",
    ]) {
        return ("Indie Rock".to_owned(), "独立摇滚".to_owned());
    }
    if has_any(&[
        "symphony", "concerto", "sonata", "nocturne", "prelude", "etude",
        "opus", "op.", "bwv", "mozart", "beethoven", "bach", "chopin",
        "debussy", "ravel", "tchaikovsky", "rachmaninoff", "schubert",
        "schumann", "brahms", "mahler", "shostakovich", "stravinsky",
        "vivaldi", "handel", "liszt", "satie", "坂本龍一", "ryuichi sakamoto",
        "max richter", "olafur arnalds", "nils frahm", "ludovico einaudi",
        "yo-yo ma", "lang lang", "gould", "philharmonic", "orchestra",
        "classical", "古典", "交响", "协奏曲", "奏鸣曲",
    ]) {
        return ("Classical".to_owned(), "Classical".to_owned());
    }
    if has_any(&[
        "jazz", "bossa", "blues", "miles davis", "john coltrane", "bill evans",
        "chet baker", "thelonious monk", "charlie parker", "duke ellington",
        "louis armstrong", "ella fitzgerald", "billie holiday", "nina simone",
        "stan getz", "joão gilberto", "astrud gilberto", "antonio carlos jobim",
        "herbie hancock", "chick corea", "keith jarrett", "kamasi washington",
        "robert glasper", "laufey", "norah jones", "diana krall", "小野丽莎",
        "爵士", "蓝调",
    ]) {
        return ("Jazz".to_owned(), "Jazz".to_owned());
    }
    if has_any(&[
        "electronic", "aphex twin", "boards of canada", "autechre", "four tet",
        "burial", "floating points", "jon hopkins", "brian eno", "kraftwerk",
        "daft punk", "justice", "chemical brothers", "prodigy", "underworld",
        "massive attack", "portishead", "bonobo", "tycho", "caribou", "jamie xx",
        "disclosure", "flume", "odesza", "porter robinson", "madeon", "deadmau5",
        "skrillex", "calvin harris", "avicii", "kygo", "synthwave", "ambient",
        "techno", "house", "idm", "电音", "电子",
    ]) {
        return ("Electronic".to_owned(), "Electronic".to_owned());
    }
    if has_any(&[
        "hip-hop", "hip hop", "rap", "r&b", "soul", "funk", "kendrick lamar",
        "kanye west", "drake", "travis scott", "tyler, the creator", "frank ocean",
        "sza", "the weeknd", "daniel caesar", "childish gambino", "j. cole",
        "nas", "jay-z", "eminem", "mf doom", "nujabes", "j dilla", "outkast",
        "a tribe called quest", "wu-tang", "mac miller", "asap rocky", "beyoncé",
        "rihanna", "bruno mars", "d'angelo", "erykah badu", "lauryn hill",
        "stevie wonder", "marvin gaye", "michael jackson", "prince", "方大同",
        "陶喆", "蛋堡", "国蛋", "夏之禹", "马思唯", "说唱", "嘻哈",
    ]) {
        return ("Hip-Hop".to_owned(), "Hip-Hop".to_owned());
    }
    if has_any(&[
        "陈粒", "宋冬野", "马頔", "尧十三", "赵雷", "李志", "万晓利", "周云蓬",
        "张玮玮", "野孩子", "房东的猫", "毛不易", "老狼", "陈绮贞", "张悬",
        "安溥", "焦安溥", "卢广仲", "魏如萱", "民谣",
    ]) {
        return ("Folk".to_owned(), "民谣".to_owned());
    }
    if has_any(&[
        "folk", "acoustic", "bob dylan", "joni mitchell", "leonard cohen",
        "nick drake", "simon & garfunkel", "paul simon", "neil young",
        "cat stevens", "fleet foxes", "bon iver", "iron & wine", "sufjan stevens",
        "hozier", "damien rice",
    ]) {
        return ("Folk".to_owned(), "Folk".to_owned());
    }
    if has_any(&[
        "ost", "soundtrack", "original score", "hans zimmer", "john williams",
        "ennio morricone", "ludwig göransson", "久石譲", "joe hisaishi",
        "泽野弘之", "sawano", "梶浦由記", "菅野よう子", "yoko kanno", "新海诚",
        "配乐", "原声",
    ]) {
        return ("Soundtrack".to_owned(), "Soundtrack".to_owned());
    }
    if has_any(&[
        "初音", "miku", "vocaloid", "yoasobi", "kenshi yonezu", "米津玄師",
        "zutomayo", "ずっと真夜中でいいのに", "yorushika", "ヨルシカ", "ado",
        "official髭男dism", "king gnu", "radwimps", "vaundy", "fujii kaze",
        "藤井風", "aimyon", "あいみょん", "utada hikaru", "宇多田ヒカル",
        "椎名林檎", "shiina ringo", "东京事变", "perfume", "lisa", "aimer",
        "j-pop", "anime",
    ]) {
        return ("J-Pop".to_owned(), "J-Pop".to_owned());
    }
    if has_any(&[
        "周杰伦", "jay chou", "陈奕迅", "eason chan", "王菲", "faye wong",
        "林俊杰", "邓紫棋", "张学友", "张国荣", "梅艳芳", "beyond", "王力宏",
        "蔡依林", "孙燕姿", "梁静茹", "五月天", "苏打绿", "吴青峰", "田馥甄",
        "张惠妹", "林宥嘉", "李荣浩", "薛之谦", "周深", "单依纯", "莫文蔚",
        "林忆莲", "李宗盛", "罗大佑", "伍佰", "张震岳", "杨乃文",
    ]) {
        return ("Pop".to_owned(), "华语流行".to_owned());
    }
    if has_any(&["metal"]) {
        return ("Metal".to_owned(), "Metal".to_owned());
    }
    if has_any(&["punk"]) {
        return ("Punk".to_owned(), "Punk".to_owned());
    }
    if has_any(&["indie", "独立"]) {
        return ("Indie Rock".to_owned(), "Indie Rock".to_owned());
    }
    if has_any(&["rock", "摇滚"]) {
        return ("Rock".to_owned(), "Rock".to_owned());
    }
    if has_any(&["lofi", "lo-fi", "chill"]) {
        return ("Lo-Fi".to_owned(), "Lo-Fi".to_owned());
    }
    let has_kana = blob.chars().any(|c| matches!(c as u32, 0x3040..=0x30FF));
    if has_kana {
        return ("J-Pop".to_owned(), "J-Pop".to_owned());
    }
    let has_cjk = blob.chars().any(|c| matches!(c as u32, 0x4E00..=0x9FFF));
    if has_cjk {
        return ("Indie Rock".to_owned(), "独立摇滚".to_owned());
    }
    ("Alternative Rock".to_owned(), "Alternative Rock".to_owned())
}

fn empty_groups() -> BTreeMap<String, Vec<Item>> {
    ["song", "artist", "album", "playlist"].into_iter().map(|kind| (kind.to_owned(), Vec::new())).collect()
}

fn default_shortcuts() -> BTreeMap<String, String> {
    [
        ("toggle_play", "Space"),
        ("activate", "Return"),
        ("enqueue", "Shift+Return"),
        ("next", "N"),
        ("prev", "P"),
        ("filter", "/"),
        ("palette", "Ctrl+K"),
        ("mode_center", "Ctrl+Shift+M"),
        ("queue", "Tab"),
        ("lyric_track", "T"),
        ("immersive", "Z"),
        ("back", "Escape"),
        ("queue_down", "Ctrl+J"),
        ("queue_up", "Ctrl+K"),
        ("reveal", "C"),
        ("help", "Question"),
        ("mode_back", "Ctrl+Shift+Backspace"),
        ("cycle_mode", "M"),
    ].into_iter().map(|(key, value)| (key.to_owned(), value.to_owned())).collect()
}

fn scheme_label(scheme: &str) -> String {
    SCHEMES.iter().find(|(id, ..)| *id == scheme).map(|(_, title, ..)| (*title).to_owned()).unwrap_or_else(|| scheme.to_owned())
}

fn remember(items: &mut Vec<String>, value: &str) {
    let value = value.trim();
    if value.is_empty() {
        return;
    }
    items.retain(|item| item != value);
    items.insert(0, value.to_owned());
    items.truncate(8);
}

static EMPTY_VIEW: ViewState = ViewState {
    scroll: 0,
    expanded: String::new(),
    focus: String::new(),
    nav: Vec::new(),
    cover_mode: String::new(),
};

#[allow(dead_code)]
pub fn prefs_path_default() -> PathBuf {
    let base = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).unwrap_or_else(|| {
        let mut home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
        home.push(".local");
        home.push("share");
        home
    });
    base.join("mineral-gui").join("state.json")
}

// ============================================================================
// Pure-Local Metadata & Tag TF-IDF + Cosine Similarity Vector Engine
//
// Document = [Artist] * w1 + [Genres/Tags] * w2 + [Release Decade] * w3
// Score(A_i) = (V_playlist · V_{A_i}) / (||V_playlist|| * ||V_{A_i}||)
// ============================================================================

const W_ARTIST: f32 = 3.0; // w1: Artist weight
const W_GENRES_TAGS: f32 = 2.2; // w2: Genres / Tags weight
const W_DECADE: f32 = 1.5; // w3: Release Decade weight

fn term_category_weight(term: &str) -> f32 {
    if term.starts_with("artist:") {
        W_ARTIST
    } else if term.starts_with("tag:") {
        W_GENRES_TAGS
    } else if term.starts_with("decade:") {
        W_DECADE
    } else {
        1.0
    }
}

fn normalize_album_key(raw: &str) -> String {
    let trimmed = raw.trim().to_lowercase();
    if trimmed.is_empty()
        || matches!(
            trimmed.as_str(),
            "unknown" | "unknown album" | "未知专辑" | "未知" | "single" | "none" | "-"
        )
    {
        return String::new();
    }
    trimmed
        .chars()
        .map(|c| if c.is_alphanumeric() || ('\u{3040}'..='\u{9fff}').contains(&c) { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_document_features(item: &Item, associated_tracks: &[Item]) -> BTreeMap<String, f32> {
    let mut doc: BTreeMap<String, f32> = BTreeMap::new();
    accumulate_item_features(&mut doc, item, 1.0);
    if !associated_tracks.is_empty() {
        let track_scale = 1.0 / (associated_tracks.len() as f32).sqrt().max(1.0);
        for tr in associated_tracks.iter().take(24) {
            accumulate_item_features(&mut doc, tr, track_scale);
        }
    }
    doc
}

fn accumulate_item_features(doc: &mut BTreeMap<String, f32>, item: &Item, scale: f32) {
    let blob = format!("{} {} {}", item.title, item.subtitle, item.album).to_lowercase();

    // 1. [Artist] features (multiplied by w1 in term_category_weight)
    let raw_artist = item
        .subtitle
        .split(" · ")
        .next()
        .unwrap_or(&item.subtitle)
        .trim()
        .to_lowercase();
    if !raw_artist.is_empty()
        && !raw_artist.ends_with("tracks")
        && !raw_artist.contains("不可用")
        && !raw_artist.contains("connected")
    {
        *doc.entry(format!("artist:{raw_artist}")).or_insert(0.0) += 1.0 * scale;
        for part in raw_artist
            .split(['/', '&', ',', '、', ';'])
            .flat_map(|s| s.split(" feat. "))
            .flat_map(|s| s.split(" ft. "))
            .map(str::trim)
            .filter(|s| s.len() >= 2 && *s != raw_artist)
        {
            *doc.entry(format!("artist:{part}")).or_insert(0.0) += 0.85 * scale;
        }
    }

    // Also detect canonical artists mentioned anywhere in title/subtitle/album (e.g. Bilibili titles)
    for (artist_kw, genre_tags, decade_tag) in ARTIST_ONTOLOGY {
        if blob.contains(artist_kw) {
            *doc.entry(format!("artist:{artist_kw}")).or_insert(0.0) += 1.0 * scale;
            for (tag, w) in *genre_tags {
                *doc.entry(format!("tag:{tag}")).or_insert(0.0) += *w * scale;
            }
            if !decade_tag.is_empty() {
                *doc.entry(format!("decade:{decade_tag}")).or_insert(0.0) += 0.9 * scale;
            }
        }
    }

    // 2. [Genres/Tags] features (multiplied by w2 in term_category_weight)
    let mut matched_any_genre = false;
    for (keywords, emitted_tags) in GENRE_KEYWORD_RULES {
        if keywords.iter().any(|kw| blob.contains(kw)) {
            matched_any_genre = true;
            for (tag, w) in *emitted_tags {
                *doc.entry(format!("tag:{tag}")).or_insert(0.0) += *w * scale;
            }
        }
    }

    // Script / Regional scene tags
    let has_kana = blob.chars().any(|c| matches!(c as u32, 0x3040..=0x30FF));
    let has_cjk = blob.chars().any(|c| matches!(c as u32, 0x4E00..=0x9FFF));
    if has_kana {
        *doc.entry("tag:lang-jp".to_owned()).or_insert(0.0) += 0.55 * scale;
        if !matched_any_genre {
            *doc.entry("tag:j-pop".to_owned()).or_insert(0.0) += 0.8 * scale;
        }
    } else if has_cjk {
        *doc.entry("tag:lang-cjk".to_owned()).or_insert(0.0) += 0.55 * scale;
        if !matched_any_genre {
            *doc.entry("tag:indie-rock".to_owned()).or_insert(0.0) += 0.7 * scale;
            *doc.entry("tag:chinese-indie".to_owned()).or_insert(0.0) += 0.7 * scale;
        }
    } else {
        *doc.entry("tag:lang-en".to_owned()).or_insert(0.0) += 0.4 * scale;
        if !matched_any_genre {
            *doc.entry("tag:alternative-rock".to_owned()).or_insert(0.0) += 0.65 * scale;
            *doc.entry("tag:indie-rock".to_owned()).or_insert(0.0) += 0.65 * scale;
        }
    }

    // Informative title/album keyword stems (fine-grained content tag matching)
    for token in tokenize_informative_words(&format!("{} {}", item.title, item.album)) {
        *doc.entry(format!("tag:kw:{token}")).or_insert(0.0) += 0.32 * scale;
    }

    // 3. [Release Decade] features (multiplied by w3 in term_category_weight)
    for decade in extract_release_decades(&blob) {
        *doc.entry(format!("decade:{decade}")).or_insert(0.0) += 1.0 * scale;
    }
}

fn extract_release_decades(blob: &str) -> Vec<&'static str> {
    let mut decades: Vec<&'static str> = Vec::new();
    let mut push_dec = |d: &'static str| {
        if !decades.contains(&d) {
            decades.push(d);
        }
    };

    // Scan 4-digit years 1950..=2029
    let bytes = blob.as_bytes();
    if bytes.len() >= 4 {
        for i in 0..=(bytes.len() - 4) {
            let boundary_before = i == 0 || !bytes[i - 1].is_ascii_digit();
            let boundary_after = i + 4 == bytes.len() || !bytes[i + 4].is_ascii_digit();
            if boundary_before
                && boundary_after
                && bytes[i..i + 4].iter().all(|b| b.is_ascii_digit())
                && let Ok(yr_str) = std::str::from_utf8(&bytes[i..i + 4])
                && let Ok(yr) = yr_str.parse::<u16>()
            {
                match yr {
                    1950..=1959 => push_dec("1950s"),
                    1960..=1969 => push_dec("1960s"),
                    1970..=1979 => push_dec("1970s"),
                    1980..=1989 => push_dec("1980s"),
                    1990..=1999 => push_dec("1990s"),
                    2000..=2009 => push_dec("2000s"),
                    2010..=2019 => push_dec("2010s"),
                    2020..=2029 => push_dec("2020s"),
                    _ => {}
                }
            }
        }
    }

    // Scan shorthand decade markers ('96, 90s, 80年代, etc.)
    for (patterns, dec) in [
        (&["60s", "60年代", "'65", "'66", "'67", "'68", "'69"][..], "1960s"),
        (&["70s", "70年代", "'70", "'71", "'72", "'73", "'74", "'75", "'76", "'77", "'78", "'79"][..], "1970s"),
        (&["80s", "80年代", "'80", "'81", "'82", "'83", "'84", "'85", "'86", "'87", "'88", "'89"][..], "1980s"),
        (&["90s", "90年代", "'90", "'91", "'92", "'93", "'94", "'95", "'96", "'97", "'98", "'99"][..], "1990s"),
        (&["00s", "00年代"][..], "2000s"),
        (&["10s", "10年代"][..], "2010s"),
        (&["20s", "20年代"][..], "2020s"),
    ] {
        if patterns.iter().any(|p| blob.contains(p)) {
            push_dec(dec);
        }
    }

    decades
}

fn tokenize_informative_words(text: &str) -> Vec<String> {
    const STOP_WORDS: &[&str] = &[
        "the", "and", "for", "with", "from", "that", "this", "you", "your", "are", "was",
        "feat", "vol", "ver", "version", "edition", "deluxe", "remaster", "remastered",
        "album", "track", "tracks", "song", "music", "official", "video", "audio",
    ];
    let mut out = Vec::new();
    for raw in text
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
    {
        if raw.len() >= 3
            && !raw.chars().all(|c| c.is_ascii_digit())
            && !STOP_WORDS.contains(&raw)
            && !out.iter().any(|existing| existing == raw)
        {
            out.push(raw.to_owned());
            if out.len() >= 8 {
                break;
            }
        }
    }
    out
}

#[allow(clippy::type_complexity)]
const ARTIST_ONTOLOGY: &[(&str, &[(&str, f32)], &str)] = &[
    // Britpop & 90s UK Rock
    ("oasis", &[("britpop", 1.0), ("rock", 0.75), ("alternative-rock", 0.7), ("indie-rock", 0.65), ("scene-uk", 0.8)], "1990s"),
    ("blur", &[("britpop", 1.0), ("art-rock", 0.7), ("alternative-rock", 0.75), ("indie-rock", 0.7), ("scene-uk", 0.8)], "1990s"),
    ("suede", &[("britpop", 1.0), ("glam-rock", 0.75), ("alternative-rock", 0.7), ("scene-uk", 0.8)], "1990s"),
    ("pulp", &[("britpop", 1.0), ("art-rock", 0.75), ("indie-rock", 0.7), ("scene-uk", 0.8)], "1990s"),
    ("the verve", &[("britpop", 0.95), ("shoegaze", 0.65), ("alternative-rock", 0.75), ("scene-uk", 0.8)], "1990s"),
    ("the stone roses", &[("britpop", 0.9), ("madchester", 1.0), ("indie-rock", 0.85), ("scene-uk", 0.8)], "1980s"),
    ("the smiths", &[("indie-rock", 0.95), ("post-punk", 0.8), ("britpop", 0.75), ("scene-uk", 0.85)], "1980s"),
    ("morrissey", &[("indie-rock", 0.9), ("britpop", 0.75), ("scene-uk", 0.8)], "1990s"),
    ("elastica", &[("britpop", 1.0), ("post-punk", 0.65), ("scene-uk", 0.8)], "1990s"),
    ("supergrass", &[("britpop", 1.0), ("rock", 0.7), ("scene-uk", 0.8)], "1990s"),
    // Shoegaze & Dream Pop
    ("my bloody valentine", &[("shoegaze", 1.0), ("dream-pop", 0.8), ("alternative-rock", 0.7), ("indie-rock", 0.65)], "1990s"),
    ("slowdive", &[("shoegaze", 1.0), ("dream-pop", 0.9), ("ambient", 0.55), ("indie-rock", 0.65), ("scene-uk", 0.75)], "1990s"),
    ("cocteau twins", &[("dream-pop", 1.0), ("shoegaze", 0.8), ("post-punk", 0.6), ("scene-uk", 0.75)], "1980s"),
    ("lush", &[("shoegaze", 0.95), ("dream-pop", 0.8), ("britpop", 0.6), ("scene-uk", 0.75)], "1990s"),
    ("ride", &[("shoegaze", 1.0), ("britpop", 0.65), ("indie-rock", 0.75), ("scene-uk", 0.8)], "1990s"),
    ("beach house", &[("dream-pop", 1.0), ("shoegaze", 0.75), ("indie-rock", 0.7)], "2010s"),
    ("mazzy star", &[("dream-pop", 1.0), ("folk", 0.55), ("indie-rock", 0.7)], "1990s"),
    ("alvvays", &[("dream-pop", 0.9), ("indie-rock", 0.9), ("shoegaze", 0.65)], "2010s"),
    ("galaxie 500", &[("dream-pop", 0.95), ("shoegaze", 0.75), ("indie-rock", 0.8)], "1980s"),
    ("kinoko teikoku", &[("shoegaze", 0.95), ("dream-pop", 0.8), ("j-rock", 0.85), ("indie-rock", 0.75)], "2010s"),
    ("きのこ帝国", &[("shoegaze", 0.95), ("dream-pop", 0.8), ("j-rock", 0.85), ("indie-rock", 0.75)], "2010s"),
    // Alternative Rock / Art Rock / Post-Rock
    ("radiohead", &[("alternative-rock", 1.0), ("art-rock", 0.95), ("electronic", 0.55), ("britpop", 0.5), ("scene-uk", 0.8)], "1990s"),
    ("thom yorke", &[("alternative-rock", 0.9), ("art-rock", 0.9), ("electronic", 0.8), ("scene-uk", 0.75)], "2000s"),
    ("muse", &[("alternative-rock", 0.95), ("art-rock", 0.75), ("rock", 0.8), ("scene-uk", 0.75)], "2000s"),
    ("my chemical romance", &[("alternative-rock", 0.95), ("emo", 1.0), ("pop-punk", 0.85), ("punk", 0.75), ("rock", 0.8)], "2000s"),
    ("green day", &[("pop-punk", 1.0), ("punk", 0.9), ("alternative-rock", 0.8), ("rock", 0.75)], "1990s"),
    ("blink-182", &[("pop-punk", 1.0), ("punk", 0.85), ("alternative-rock", 0.75)], "2000s"),
    ("fall out boy", &[("pop-punk", 0.95), ("emo", 0.9), ("alternative-rock", 0.8)], "2000s"),
    ("paramore", &[("pop-punk", 0.9), ("emo", 0.9), ("alternative-rock", 0.85)], "2000s"),
    ("linkin park", &[("alternative-rock", 0.95), ("metal", 0.75), ("rock", 0.85)], "2000s"),
    ("sigur rós", &[("post-rock", 1.0), ("ambient", 0.75), ("art-rock", 0.8), ("alternative-rock", 0.7)], "2000s"),
    ("sigur ros", &[("post-rock", 1.0), ("ambient", 0.75), ("art-rock", 0.8), ("alternative-rock", 0.7)], "2000s"),
    ("mogwai", &[("post-rock", 1.0), ("alternative-rock", 0.75), ("scene-uk", 0.7)], "1990s"),
    ("explosions in the sky", &[("post-rock", 1.0), ("alternative-rock", 0.7)], "2000s"),
    // Post-Punk & New Wave
    ("joy division", &[("post-punk", 1.0), ("new-wave", 0.75), ("alternative-rock", 0.7), ("scene-uk", 0.85)], "1970s"),
    ("new order", &[("post-punk", 0.85), ("new-wave", 1.0), ("electronic", 0.75), ("scene-uk", 0.85)], "1980s"),
    ("the cure", &[("post-punk", 0.95), ("new-wave", 0.85), ("dream-pop", 0.65), ("alternative-rock", 0.75), ("scene-uk", 0.8)], "1980s"),
    ("interpol", &[("post-punk", 1.0), ("indie-rock", 0.85), ("alternative-rock", 0.75)], "2000s"),
    ("sonic youth", &[("post-punk", 0.8), ("alternative-rock", 0.95), ("indie-rock", 0.85), ("shoegaze", 0.55)], "1980s"),
    ("pixies", &[("alternative-rock", 1.0), ("indie-rock", 0.9), ("post-punk", 0.65)], "1980s"),
    ("the velvet underground", &[("art-rock", 0.95), ("classic-rock", 0.85), ("post-punk", 0.7), ("indie-rock", 0.75)], "1960s"),
    // Classic Rock & Grunge
    ("the beatles", &[("classic-rock", 1.0), ("rock", 0.9), ("pop", 0.65), ("scene-uk", 0.8)], "1960s"),
    ("pink floyd", &[("classic-rock", 1.0), ("art-rock", 0.95), ("rock", 0.85), ("scene-uk", 0.8)], "1970s"),
    ("led zeppelin", &[("classic-rock", 1.0), ("rock", 0.95), ("scene-uk", 0.8)], "1970s"),
    ("david bowie", &[("classic-rock", 0.9), ("art-rock", 0.95), ("glam-rock", 0.9), ("scene-uk", 0.8)], "1970s"),
    ("queen", &[("classic-rock", 1.0), ("glam-rock", 0.8), ("rock", 0.9), ("scene-uk", 0.8)], "1970s"),
    ("nirvana", &[("grunge", 1.0), ("alternative-rock", 0.95), ("rock", 0.85), ("punk", 0.7)], "1990s"),
    ("pearl jam", &[("grunge", 1.0), ("alternative-rock", 0.9), ("classic-rock", 0.75)], "1990s"),
    ("smashing pumpkins", &[("alternative-rock", 1.0), ("grunge", 0.8), ("shoegaze", 0.65)], "1990s"),
    // Indie Rock
    ("the strokes", &[("indie-rock", 1.0), ("post-punk", 0.7), ("alternative-rock", 0.8), ("rock", 0.75)], "2000s"),
    ("arctic monkeys", &[("indie-rock", 1.0), ("britpop", 0.7), ("alternative-rock", 0.85), ("scene-uk", 0.8)], "2000s"),
    ("tame impala", &[("indie-rock", 0.9), ("dream-pop", 0.75), ("electronic", 0.65), ("alternative-rock", 0.75)], "2010s"),
    ("cigarettes after sex", &[("dream-pop", 1.0), ("shoegaze", 0.75), ("indie-rock", 0.8)], "2010s"),
    // Chinese Indie & Rock
    ("万能青年旅店", &[("chinese-indie", 1.0), ("indie-rock", 0.95), ("art-rock", 0.8), ("post-rock", 0.65), ("rock", 0.8)], "2010s"),
    ("草东没有派对", &[("chinese-indie", 1.0), ("indie-rock", 0.95), ("post-punk", 0.7), ("alternative-rock", 0.8)], "2010s"),
    ("重塑雕像的权利", &[("chinese-indie", 0.95), ("post-punk", 1.0), ("new-wave", 0.8), ("indie-rock", 0.85)], "2010s"),
    ("新裤子", &[("chinese-indie", 0.95), ("new-wave", 0.9), ("synthwave", 0.75), ("indie-rock", 0.85)], "2000s"),
    ("落日飞车", &[("chinese-indie", 0.95), ("dream-pop", 0.9), ("indie-rock", 0.85), ("city-pop", 0.8)], "2010s"),
    ("sunset rollercoaster", &[("chinese-indie", 0.95), ("dream-pop", 0.9), ("indie-rock", 0.85), ("city-pop", 0.8)], "2010s"),
    ("deca joins", &[("chinese-indie", 1.0), ("indie-rock", 0.9), ("dream-pop", 0.75)], "2010s"),
    ("窦唯", &[("chinese-indie", 0.95), ("post-punk", 0.8), ("ambient", 0.75), ("rock", 0.85)], "1990s"),
    ("崔健", &[("chinese-indie", 0.85), ("rock", 1.0), ("classic-rock", 0.8)], "1980s"),
    // Classical / Neoclassical
    ("坂本龍一", &[("classical", 0.9), ("ambient", 0.8), ("electronic", 0.65), ("soundtrack", 0.8)], "1980s"),
    ("ryuichi sakamoto", &[("classical", 0.9), ("ambient", 0.8), ("electronic", 0.65), ("soundtrack", 0.8)], "1980s"),
    // Electronic
    ("aphex twin", &[("electronic", 1.0), ("idm", 1.0), ("ambient", 0.8)], "1990s"),
    ("boards of canada", &[("electronic", 1.0), ("idm", 0.9), ("ambient", 0.9)], "1990s"),
    ("daft punk", &[("electronic", 1.0), ("house", 0.9), ("funk", 0.7)], "2000s"),
    ("massive attack", &[("electronic", 0.9), ("trip-hop", 1.0), ("scene-uk", 0.75)], "1990s"),
    ("portishead", &[("electronic", 0.85), ("trip-hop", 1.0), ("scene-uk", 0.75)], "1990s"),
    // J-Pop / Vocaloid / Anime
    ("初音", &[("vocaloid", 1.0), ("j-pop", 0.9), ("electronic", 0.6)], "2010s"),
    ("miku", &[("vocaloid", 1.0), ("j-pop", 0.9), ("electronic", 0.6)], "2010s"),
    ("洛天依", &[("vocaloid", 1.0), ("j-pop", 0.75), ("c-pop", 0.7)], "2010s"),
    ("yoasobi", &[("j-pop", 1.0), ("anime", 0.8), ("electronic", 0.5)], "2020s"),
    ("米津玄師", &[("j-pop", 1.0), ("j-rock", 0.75), ("vocaloid", 0.6)], "2010s"),
    ("kenshi yonezu", &[("j-pop", 1.0), ("j-rock", 0.75), ("vocaloid", 0.6)], "2010s"),
    ("radwimps", &[("j-rock", 0.95), ("j-pop", 0.85), ("soundtrack", 0.75), ("anime", 0.75)], "2010s"),
    ("椎名林檎", &[("j-pop", 0.9), ("j-rock", 0.9), ("jazz", 0.65), ("art-rock", 0.75)], "2000s"),
    // C-Pop / Folk
    ("周杰伦", &[("c-pop", 1.0), ("pop", 0.9), ("rnb", 0.7)], "2000s"),
    ("陈奕迅", &[("c-pop", 1.0), ("pop", 0.95)], "2000s"),
    ("王菲", &[("c-pop", 0.95), ("dream-pop", 0.85), ("pop", 0.9)], "1990s"),
    ("陈粒", &[("folk", 0.95), ("chinese-indie", 0.85), ("c-pop", 0.7)], "2010s"),
    ("宋冬野", &[("folk", 1.0), ("chinese-indie", 0.8)], "2010s"),
    ("赵雷", &[("folk", 1.0), ("chinese-indie", 0.75)], "2010s"),
];

#[allow(clippy::type_complexity)]
const GENRE_KEYWORD_RULES: &[(&[&str], &[(&str, f32)])] = &[
    (&["shoegaze", "自赏", "loveless", "souvlaki"], &[("shoegaze", 1.0), ("dream-pop", 0.75), ("indie-rock", 0.65), ("alternative-rock", 0.6)]),
    (&["dream pop", "dreampop", "梦幻流行", "heaven or las vegas"], &[("dream-pop", 1.0), ("shoegaze", 0.75), ("indie-rock", 0.65)]),
    (&["britpop", "英伦摇滚", "英式摇滚", "morning glory", "definitely maybe", "parklife", "different class", "urban hymns", "knebworth"], &[("britpop", 1.0), ("rock", 0.75), ("alternative-rock", 0.7), ("indie-rock", 0.7), ("scene-uk", 0.8)]),
    (&["post-rock", "post rock", "后摇"], &[("post-rock", 1.0), ("alternative-rock", 0.7), ("ambient", 0.55)]),
    (&["post-punk", "post punk", "后朋克", "unknown pleasures"], &[("post-punk", 1.0), ("new-wave", 0.7), ("alternative-rock", 0.7)]),
    (&["new wave", "新浪潮", "synthpop", "synth-pop"], &[("new-wave", 1.0), ("post-punk", 0.65), ("electronic", 0.65)]),
    (&["grunge", "垃圾摇滚", "nevermind", "in utero"], &[("grunge", 1.0), ("alternative-rock", 0.9), ("rock", 0.8)]),
    (&["pop-punk", "pop punk", "流行朋克", "emo", "black parade"], &[("pop-punk", 0.95), ("emo", 0.9), ("punk", 0.8), ("alternative-rock", 0.75)]),
    (&["punk", "朋克"], &[("punk", 1.0), ("rock", 0.75), ("alternative-rock", 0.65)]),
    (&["metal", "金属", "heavy metal"], &[("metal", 1.0), ("rock", 0.8)]),
    (&["indie rock", "indie", "独立摇滚", "独立"], &[("indie-rock", 1.0), ("alternative-rock", 0.75), ("rock", 0.65)]),
    (&["alternative", "另类摇滚", "ok computer", "the bends", "kid a", "in rainbows"], &[("alternative-rock", 1.0), ("art-rock", 0.7), ("rock", 0.75)]),
    (&["classic rock", "经典摇滚", "dark side of the moon", "abbey road"], &[("classic-rock", 1.0), ("rock", 0.9)]),
    (&["rock", "摇滚"], &[("rock", 0.9), ("alternative-rock", 0.6)]),
    (&["classical", "symphony", "concerto", "sonata", "nocturne", "prelude", "bwv", "mozart", "beethoven", "bach", "chopin", "debussy", "古典", "交响", "奏鸣曲", "协奏曲"], &[("classical", 1.0)]),
    (&["jazz", "bossa", "blues", "miles davis", "coltrane", "bill evans", "chet baker", "爵士", "蓝调"], &[("jazz", 1.0)]),
    (&["ambient", "氛围"], &[("ambient", 1.0), ("electronic", 0.7)]),
    (&["electronic", "synthwave", "techno", "house", "idm", "trip-hop", "电音", "电子"], &[("electronic", 1.0)]),
    (&["hip-hop", "hip hop", "rap", "说唱", "嘻哈"], &[("hip-hop", 1.0), ("rnb", 0.6)]),
    (&["r&b", "rnb", "soul", "funk"], &[("rnb", 1.0), ("hip-hop", 0.6)]),
    (&["folk", "acoustic", "民谣"], &[("folk", 1.0)]),
    (&["ost", "soundtrack", "original score", "配乐", "原声"], &[("soundtrack", 1.0)]),
    (&["vocaloid", "ボカロ", "虚拟歌姬"], &[("vocaloid", 1.0), ("j-pop", 0.8)]),
    (&["j-pop", "jpop", "anime", "日系"], &[("j-pop", 1.0)]),
    (&["lofi", "lo-fi", "chill"], &[("lo-fi", 1.0), ("electronic", 0.5)]),
    (&["live", "现场", "concert", "unplugged", "knebworth"], &[("live", 0.75)]),
];

#[cfg(test)]
mod tfidf_recommender_tests {
    use super::*;
    use crate::backend::MemoryBackend;

    fn make_song(key: &str, title: &str, artist: &str, album: &str) -> Item {
        Item {
            key: key.to_owned(),
            kind: "song".to_owned(),
            title: title.to_owned(),
            subtitle: artist.to_owned(),
            album: album.to_owned(),
            source: "netease".to_owned(),
            duration_ms: Some(240_000),
            unavailable: false,
            loved: false,
            cover_url: Some(format!("https://example.com/{key}.jpg")),
            index: None,
            current: false,
        }
    }

    fn make_album(key: &str, title: &str, artist: &str) -> Item {
        Item {
            key: key.to_owned(),
            kind: "album".to_owned(),
            title: title.to_owned(),
            subtitle: artist.to_owned(),
            album: title.to_owned(),
            source: "netease".to_owned(),
            duration_ms: None,
            unavailable: false,
            loved: false,
            cover_url: Some(format!("https://example.com/{key}.jpg")),
            index: None,
            current: false,
        }
    }

    #[test]
    fn tfidf_cosine_similarity_ranks_matching_genre_artist_and_decade_top_4() {
        let tmp = std::env::temp_dir().join("mineral-tfidf-test-state.json");
        let mut session = Session::open(MemoryBackend::default(), tmp);

        // Current playlist is Britpop / 1990s UK Rock (Oasis & Blur)
        let s1 = make_song("s1", "Slide Away (Live at Knebworth, 11 August '96)", "Oasis", "Knebworth 1996");
        let s2 = make_song("s2", "Champagne Supernova", "Oasis", "What's the Story Morning Glory (1995)");
        let s3 = make_song("s3", "Girls & Boys", "Blur", "Parklife (1994)");
        session.current = Some(s1.clone());
        session.queue = vec![s1, s2, s3];

        // Candidate album pool containing 4 Britpop/90s UK albums and 4 unrelated Classical/Hip-Hop albums
        let brit1 = make_album("alb_brit1", "Definitely Maybe (1994)", "Oasis");
        let brit2 = make_album("alb_brit2", "Modern Life Is Rubbish (1993)", "Blur");
        let brit3 = make_album("alb_brit3", "Different Class (1995)", "Pulp");
        let brit4 = make_album("alb_brit4", "Urban Hymns (1997)", "The Verve");
        let class1 = make_album("alb_class1", "Nocturnes Op. 9", "Chopin");
        let class2 = make_album("alb_class2", "Symphony No. 9", "Beethoven");
        let hip1 = make_album("alb_hip1", "DAMN. (2017) Hip-Hop", "Kendrick Lamar");

        for alb in [class1, hip1, brit1, class2, brit2, brit3, brit4] {
            session.catalog.insert(alb.key.clone(), alb);
        }

        let wall = session.recommended_album_wall();
        assert_eq!(wall.len(), 4);
        let keys: Vec<&str> = wall.iter().map(|a| a.key.as_str()).collect();
        // All 4 recommended albums in the 2x2 wall must be the Britpop / 1990s UK albums!
        assert!(keys.contains(&"alb_brit1"), "Expected Oasis album in top 4: {keys:?}");
        assert!(keys.contains(&"alb_brit2"), "Expected Blur album in top 4: {keys:?}");
        assert!(keys.contains(&"alb_brit3"), "Expected Pulp album in top 4: {keys:?}");
        assert!(keys.contains(&"alb_brit4"), "Expected The Verve album in top 4: {keys:?}");
        // Oasis's own 1994 album should rank #1 due to [Artist]*w1 + [Genres/Tags]*w2 + [Release Decade]*w3
        assert_eq!(keys[0], "alb_brit1");
    }
}


