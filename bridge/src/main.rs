//! Stdio JSON-lines adapter in front of `mineral_client`.
//!
//! The Qt and GTK shells share one playback session by talking to this process.
//! Playback itself stays in the Mineral daemon.

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::process::Stdio;
use std::time::Duration;

use mineral_audio::OutputTarget;
use mineral_channel_core::{Page, PlaylistLoad};
use mineral_client::connection::ClientConfig;
use mineral_client::Client;
use mineral_model::{
    AlbumId, ArtistId, Lyrics, PlaylistId, SearchKind, Song, SongId, SourceKind,
};
use mineral_paths::socket_path;
use mineral_protocol::{
    Event, FailureKind, PlayMode, QueueAnchor, QueueContextWire, QueueOp, QueuePos, Request,
    Subscription, SubscriptionTopic,
};
use mineral_task::{ChannelFetchKind, Priority, TaskEvent, TaskKind};
use serde_json::{json, Value};
use tokio::process::Command;
use tokio::sync::mpsc;

fn emit(value: Value) {
    let mut out = std::io::stdout().lock();
    if let Ok(line) = serde_json::to_string(&value) {
        let _ = writeln!(out, "{line}");
        let _ = out.flush();
    }
}

fn fail_text(kind: &FailureKind) -> &'static str {
    match kind {
        FailureKind::Invalid => "请求无效，播放状态未改变",
        FailureKind::NotFound => "目标不存在",
        FailureKind::Unavailable => "音源未登录或当前不可用",
        FailureKind::Conflict => "状态已变化，请重试",
        FailureKind::Internal => "Daemon 内部错误",
    }
}

fn source_from(name: &str) -> SourceKind {
    SourceKind::from_name(name)
}

fn split_key(key: &str) -> Option<(&str, &str)> {
    key.split_once(':')
}

fn song_id(key: &str) -> Option<SongId> {
    let (ns, raw) = split_key(key)?;
    Some(SongId::new(source_from(ns), raw))
}

fn playlist_id(key: &str) -> Option<PlaylistId> {
    let (ns, raw) = split_key(key)?;
    Some(PlaylistId::new(source_from(ns), raw))
}

fn album_id(key: &str) -> Option<AlbumId> {
    let (ns, raw) = split_key(key)?;
    Some(AlbumId::new(source_from(ns), raw))
}

fn artist_id(key: &str) -> Option<ArtistId> {
    let (ns, raw) = split_key(key)?;
    Some(ArtistId::new(source_from(ns), raw))
}

fn clean_label(raw: &str) -> String {
    let cleaned: String = raw
        .chars()
        .filter(|ch| !matches!(*ch as u32, 0xE000..=0xF8FF | 0xF0000..=0x10FFFF))
        .collect();
    let trimmed = cleaned.trim();
    if trimmed.is_empty() {
        raw.to_owned()
    } else {
        trimmed.to_owned()
    }
}

fn cover_of(source: &str, url: &Option<mineral_model::MediaUrl>) -> Option<String> {
    let raw = url.as_ref()?.to_string();
    if raw.is_empty() {
        return None;
    }
    if !raw.starts_with("http://") && !raw.starts_with("https://") {
        return Some(raw);
    }
    let base_dir = mineral_paths::cover_cache_dir().unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_owned());
        std::path::PathBuf::from(home).join(".cache/mineral/cover")
    });
    let sub_dir = base_dir.join(if source.is_empty() { "misc" } else { source });
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    raw.hash(&mut hasher);
    let hex = format!("{:016x}", hasher.finish());
    for ext in ["jpg", "png", "webp", "img"] {
        let candidate = sub_dir.join(format!("{hex}.{ext}"));
        if candidate.exists() {
            return Some(candidate.to_string_lossy().into_owned());
        }
    }
    let target = sub_dir.join(format!("{hex}.jpg"));
    static IN_FLIGHT: std::sync::OnceLock<std::sync::Mutex<std::collections::HashSet<String>>> =
        std::sync::OnceLock::new();
    let set = IN_FLIGHT.get_or_init(|| std::sync::Mutex::new(std::collections::HashSet::new()));
    if let Ok(mut guard) = set.lock()
        && guard.insert(raw.clone())
    {
        let target_clone = target.clone();
        let sub_dir_clone = sub_dir.clone();
        tokio::spawn(async move {
            let _ = tokio::fs::create_dir_all(&sub_dir_clone).await;
            let tmp = sub_dir_clone.join(format!("{hex}.part"));
            let mut cmd = Command::new("curl");
            cmd.args(["-sL", "--max-time", "8", "-o"])
                .arg(&tmp)
                .arg(&raw)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            #[cfg(windows)]
            cmd.creation_flags(0x0800_0000);
            let status = cmd.status().await;
            if status.is_ok_and(|s| s.success())
                && tokio::fs::metadata(&tmp).await.is_ok_and(|m| m.len() > 128)
            {
                let _ = tokio::fs::rename(&tmp, &target_clone).await;
            } else {
                let _ = tokio::fs::remove_file(&tmp).await;
            }
        });
    }
    Some(target.to_string_lossy().into_owned())
}

fn song_json(song: &Song, loved: &HashMap<String, bool>) -> Value {
    let key = song.id.qualified();
    let source_name = song.source().name();
    let artists: Vec<String> = song.artists.iter().map(|artist| artist.name.clone()).collect();
    json!({
        "key": key,
        "kind": "song",
        "title": song.name,
        "subtitle": artists.join(" / "),
        "artists": artists,
        "album": song.album.as_ref().map(|album| album.name.clone()).unwrap_or_default(),
        "duration_ms": song.duration_ms,
        "cover_url": cover_of(source_name, &song.cover_url),
        "source": source_name,
        "source_label": clean_label(song.source().label()),
        "unavailable": song.unavailable,
        "loved": loved.get(&key).copied().unwrap_or(false),
    })
}

fn cache_song(cache: &mut HashMap<String, Song>, loved: &HashMap<String, bool>, song: &Song) -> Value {
    let dto = song_json(song, loved);
    cache.insert(song.id.qualified(), song.clone());
    dto
}

fn lyric_json(lyrics: &Lyrics) -> Vec<Value> {
    lyrics
        .lines
        .iter()
        .map(|line| {
            let words: Vec<Value> = line
                .kind
                .words()
                .iter()
                .map(|word| {
                    json!({
                        "t": word.start_ms,
                        "dur": word.dur_ms,
                        "text": word.text,
                    })
                })
                .collect();
            json!({
                "t": line.time_ms,
                "text": line.kind.text(),
                "translation": line.translation,
                "romanization": line.romanization,
                "words": words,
            })
        })
        .collect()
}

fn playlist_json(playlist: &mineral_model::Playlist) -> Value {
    let source_name = playlist.source().name();
    json!({
        "key": playlist.id.qualified(),
        "kind": "playlist",
        "title": playlist.name,
        "subtitle": format!("{} tracks · {}", playlist.track_count, source_name),
        "album": format!("{} tracks", playlist.track_count),
        "source": source_name,
        "source_label": clean_label(playlist.source().label()),
        "cover_url": cover_of(source_name, &playlist.cover_url),
        "track_count": playlist.track_count,
    })
}

fn album_json(album: &mineral_model::Album) -> Value {
    let source_name = album.source().name();
    let artists: Vec<String> = album.artists.iter().map(|artist| artist.name.clone()).collect();
    json!({
        "key": album.id.qualified(),
        "kind": "album",
        "title": album.name,
        "subtitle": artists.join(" / "),
        "album": "Album",
        "source": source_name,
        "source_label": clean_label(album.source().label()),
        "cover_url": cover_of(source_name, &album.cover_url),
    })
}

fn artist_json(artist: &mineral_model::Artist) -> Value {
    let source_name = artist.source().name();
    json!({
        "key": artist.id.qualified(),
        "kind": "artist",
        "title": artist.name,
        "subtitle": artist.description.chars().take(80).collect::<String>(),
        "album": "Artist",
        "source": source_name,
        "source_label": clean_label(artist.source().label()),
        "cover_url": cover_of(source_name, &artist.avatar_url),
    })
}

struct App {
    client: Option<Client>,
    songs: HashMap<String, Song>,
    loved: HashMap<String, bool>,
    searchable: HashMap<String, Vec<SearchKind>>,
    launched: Option<tokio::process::Child>,
    last_player_sig: Value,
    last_downloads_sig: Value,
}

impl App {
    fn new() -> Self {
        Self {
            client: None,
            songs: HashMap::new(),
            loved: HashMap::new(),
            searchable: HashMap::new(),
            launched: None,
            last_player_sig: Value::Null,
            last_downloads_sig: Value::Null,
        }
    }

    async fn try_connect_once(&mut self) -> Result<(), String> {
        let path = socket_path().map_err(|e| format!("无法解析 socket：{e}"))?;
        let wire = mineral_protocol::SocketWire::connect(&path)
            .await
            .map_err(|e| format!("{e}"))?;
        let client = Client::from_wire(Box::new(wire), "mineral-gui", ClientConfig::default())
            .await
            .map_err(|e| format!("握手失败：{e}"))?;
        for topic in [
            SubscriptionTopic::Player,
            SubscriptionTopic::Playback,
            SubscriptionTopic::Tasks,
            SubscriptionTopic::DownloadsSummary,
            SubscriptionTopic::DownloadsDetail,
            SubscriptionTopic::Pcm,
            SubscriptionTopic::Events(Subscription::Toast),
            SubscriptionTopic::Events(Subscription::Lifecycle),
            SubscriptionTopic::Events(Subscription::ServiceInfo),
            SubscriptionTopic::Events(Subscription::Task),
        ] {
            let _ = client.subscribe(topic);
        }
        client
            .wait_subscriptions_ready(Duration::from_secs(2))
            .await;
        self.client = Some(client);
        self.last_player_sig = Value::Null;
        self.last_downloads_sig = Value::Null;
        emit(json!({
            "event": "connection",
            "connected": true,
            "message": "已连接 Mineral daemon",
            "socket": path.display().to_string(),
        }));
        self.refresh_caps().await;
        self.publish();
        Ok(())
    }

    async fn connect(&mut self) {
        match self.try_connect_once().await {
            Ok(()) => {}
            Err(_error) if self.launched.is_none() => {
                self.launch_daemon().await;
            }
            Err(error) => {
                emit(json!({
                    "event": "connection",
                    "connected": false,
                    "message": format!("Daemon 未连接（{error}）。可启动 mineral serve 后重试。"),
                }));
            }
        }
    }

    async fn launch_daemon(&mut self) {
        if let Some(child) = &mut self.launched {
            if matches!(child.try_wait(), Ok(None)) {
                if let Err(error) = self.try_connect_once().await {
                    emit(json!({
                        "event": "connection",
                        "connected": false,
                        "message": format!("Daemon 未连接（{error}）。"),
                    }));
                }
                return;
            }
            self.launched = None;
        }
        let mut candidates = Vec::new();
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                candidates.push(dir.join("mineral"));
                candidates.push(dir.join("mineral.exe"));
            }
            candidates.push(exe);
        }
        candidates.push(std::path::PathBuf::from("/usr/bin/mineral"));
        candidates.push(std::path::PathBuf::from("/usr/local/bin/mineral"));
        candidates.push(std::path::PathBuf::from("/opt/homebrew/bin/mineral"));
        candidates.push(std::path::PathBuf::from("mineral"));
        candidates.push(std::path::PathBuf::from("mineral.exe"));
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            let home = std::path::PathBuf::from(home);
            candidates.push(home.join(".cargo/bin/mineral"));
            candidates.push(home.join(".cargo/bin/mineral.exe"));
            candidates.push(home.join(".local/bin/mineral"));
        }
        let mut last_error = None;
        for bin in candidates {
            let mut cmd = Command::new(&bin);
            cmd.arg("serve")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null());
            #[cfg(unix)]
            cmd.process_group(0);
            #[cfg(windows)]
            cmd.creation_flags(0x0800_0000);
            match cmd.spawn() {
                Ok(child) => {
                    self.launched = Some(child);
                    emit(json!({"event":"notice","level":"info","text":"正在启动 mineral serve"}));
                    for _ in 0..8 {
                        tokio::time::sleep(Duration::from_millis(250)).await;
                        if self.try_connect_once().await.is_ok() {
                            return;
                        }
                    }
                    if let Err(error) = self.try_connect_once().await {
                        emit(json!({
                            "event": "connection",
                            "connected": false,
                            "message": format!("Daemon 未连接（{error}）。"),
                        }));
                    }
                    return;
                }
                Err(error) => last_error = Some(error),
            }
        }
        if let Some(error) = last_error {
            emit(json!({
                "event": "notice",
                "level": "error",
                "text": format!("无法启动 mineral：{error}。请先安装 Mineral daemon。"),
            }));
        }
    }

    fn client(&self) -> Option<&Client> {
        self.client.as_ref().filter(|client| client.connected())
    }

    async fn refresh_caps(&mut self) {
        let Some(client) = self.client() else {
            return;
        };
        match client.channel_caps().await.into_success() {
            Some(caps) => {
                self.searchable.clear();
                let mut sources: Vec<Value> = Vec::new();
                for (source, cap) in &caps {
                    self.searchable
                        .insert(source.name().to_owned(), cap.searchable().to_vec());
                    let kinds: Vec<&str> = cap
                        .searchable()
                        .iter()
                        .map(|kind| match kind {
                            SearchKind::Song => "song",
                            SearchKind::Album => "album",
                            SearchKind::Artist => "artist",
                            SearchKind::Playlist => "playlist",
                            SearchKind::User => "user",
                        })
                        .collect();
                    sources.push(json!({
                        "id": source.name(),
                        "label": clean_label(source.label()),
                        "available": true,
                        "logged_in": true,
                        "searchable": kinds,
                    }));
                }
                if sources.is_empty() {
                    emit(json!({
                        "event": "sources",
                        "sources": [],
                        "message": "音源未登录或没有可用音源",
                    }));
                } else {
                    emit(json!({"event":"sources","sources":sources}));
                    if let Some(client) = self.client() {
                        for (source, _) in caps {
                            let _ = client.try_fire(Request::SubmitTask(
                                TaskKind::ChannelFetch(ChannelFetchKind::MyPlaylists { source }),
                                Priority::User,
                            ));
                            let _ = client.try_fire(Request::SubmitTask(
                                TaskKind::ChannelFetch(ChannelFetchKind::MyAlbums { source }),
                                Priority::Background,
                            ));
                        }
                    }
                }
            }
            None => {
                emit(json!({
                    "event": "sources",
                    "sources": [],
                    "message": "音源未登录或能力表不可用",
                }));
            }
        }
    }

    fn publish(&mut self) {
        let extracted = {
            let Some(client) = self.client() else {
                return;
            };
            let player_bits = client.mirror().read_player(|player| {
                (
                    player.queue().to_vec(),
                    player.current().and_then(|current| current.current_song.clone()),
                    player.current().and_then(|current| current.current_lyrics.clone()),
                    player.cursor().queue_index(),
                    player.play_mode(),
                    player.queue_ready(),
                )
            });
            let fallback_dur = player_bits.1.as_ref().and_then(|song| song.duration_ms);
            let playback = client.mirror().read_playback(|playback| {
                let anchor = playback.anchor();
                json!({
                    "playing": anchor.playing,
                    "position_ms": playback.position_ms(std::time::Instant::now()),
                    "duration_ms": anchor.duration_ms.or(fallback_dur),
                    "volume": anchor.volume_pct,
                    "backend": format!("{:?}", anchor.backend),
                })
            });
            let downloads = client.mirror().downloads_detail_snapshot();
            let pcm = client.mirror().drain_pcm();
            let events = client.mirror().drain_events();
            (player_bits, playback, downloads, pcm, events)
        };
        let ((queue_songs, current, lyrics, cursor, mode, ready), playback, downloads, pcm, events) =
            extracted;
        let mode_name = match mode {
            PlayMode::Sequential => "sequential",
            PlayMode::Shuffle => "shuffle",
            PlayMode::RepeatAll => "repeat_all",
            PlayMode::RepeatOne => "repeat_one",
        };
        let queue: Vec<Value> = queue_songs
            .iter()
            .map(|song| cache_song(&mut self.songs, &self.loved, song))
            .collect();
        let current_json = current
            .as_ref()
            .map(|song| cache_song(&mut self.songs, &self.loved, song));
        let lyrics_val = lyrics.as_ref().map(lyric_json).unwrap_or_default();
        let is_playing = playback.get("playing").and_then(Value::as_bool).unwrap_or(false);
        let pos_bucket = if is_playing {
            playback.get("position_ms").and_then(Value::as_u64).unwrap_or(0) / 500
        } else {
            playback.get("position_ms").and_then(Value::as_u64).unwrap_or(0) / 2000
        };
        let player_sig = json!({
            "playing": is_playing,
            "pos_bucket": pos_bucket,
            "duration_ms": playback.get("duration_ms"),
            "volume": playback.get("volume"),
            "queue": queue,
            "cursor": cursor,
            "mode": mode_name,
            "current": current_json,
            "lyrics_len": lyrics_val.len(),
            "queue_ready": ready,
        });
        if self.last_player_sig != player_sig {
            self.last_player_sig = player_sig;
            emit(json!({
                "event": "player",
                "player": playback,
                "state": {
                    "queue": queue,
                    "cursor": cursor,
                    "mode": mode_name,
                    "current": current_json,
                    "lyrics": lyrics_val,
                    "queue_ready": ready,
                }
            }));
        }
        if let Some(detail) = downloads {
            let items: Vec<Value> = detail
                .rows()
                .iter()
                .map(|row| {
                    json!({
                        "song": cache_song(&mut self.songs, &self.loved, &row.song),
                        "status": format!("{:?}", row.status),
                        "bytes_done": row.bytes_done,
                        "bytes_total": row.bytes_total,
                        "failure": row.failure.as_ref().map(|failure| format!("{failure}")),
                    })
                })
                .collect();
            let dl_val = json!(items);
            if self.last_downloads_sig != dl_val {
                self.last_downloads_sig = dl_val.clone();
                emit(json!({"event":"downloads","items":dl_val}));
            }
        }
        if !pcm.is_empty() {
            emit(pcm_json(&pcm));
        }
        for event in events {
            match event {
                Event::Task(task) => self.on_task(*task),
                Event::Toast { content, .. } => {
                    let text: String = content.iter().map(|span| span.text.as_str()).collect();
                    if !text.is_empty() {
                        emit(json!({"event":"notice","level":"info","text":text}));
                    }
                }
                Event::Failure(notice) => {
                    emit(json!({"event":"notice","level":"error","text":notice.to_string()}));
                }
                _ => {}
            }
        }
    }

    fn on_task(&mut self, task: TaskEvent) {
        match task {
            TaskEvent::LibrarySnapshot { playlists } => {
                let items: Vec<Value> = playlists.iter().map(playlist_json).collect();
                emit(json!({"event":"library","playlists":items}));
            }
            TaskEvent::PlaylistsFetched { source, playlists } => {
                let items: Vec<Value> = playlists.iter().map(playlist_json).collect();
                emit(json!({"event":"library","source":source.name(),"playlists":items,"partial":true}));
            }
            TaskEvent::SavedAlbumsSnapshot { albums } => {
                let items: Vec<Value> = albums.iter().map(album_json).collect();
                emit(json!({"event":"library","playlists":items,"partial":true}));
            }
            TaskEvent::AlbumsFetched { source, albums } => {
                let items: Vec<Value> = albums.iter().map(album_json).collect();
                emit(json!({"event":"library","source":source.name(),"playlists":items,"partial":true}));
            }
            TaskEvent::LikedSongIdsFetched { source, ids } => {
                let keys: Vec<String> = ids.iter().map(|id| id.qualified()).collect();
                for key in &keys {
                    self.loved.insert(key.clone(), true);
                }
                emit(json!({"event":"loved","source":source.name(),"keys":keys}));
            }
            TaskEvent::PlaylistDetailFetched { id, detail, .. } => {
                let rows: Vec<Value> = detail
                    .playlist
                    .entries
                    .iter()
                    .map(|entry| cache_song(&mut self.songs, &self.loved, &entry.song))
                    .collect();
                emit(json!({
                    "event": "detail",
                    "key": id.qualified(),
                    "kind": "playlist",
                    "title": detail.playlist.name,
                    "rows": rows,
                    "complete": detail.complete,
                }));
            }
            TaskEvent::PlaylistDetailFailed { id, .. } => {
                emit(json!({
                    "event": "detail",
                    "key": id.qualified(),
                    "kind": "playlist",
                    "error": "歌单不可用或音源未登录",
                }));
            }
            TaskEvent::SearchResults {
                source,
                kind,
                query,
                payload,
                ..
            } => {
                let kind_name = search_kind_name(kind);
                let items = match payload {
                    mineral_task::SearchPayload::Songs(songs) => songs
                        .iter()
                        .map(|song| cache_song(&mut self.songs, &self.loved, song))
                        .collect::<Vec<_>>(),
                    mineral_task::SearchPayload::Albums(albums) => {
                        albums.iter().map(album_json).collect()
                    }
                    mineral_task::SearchPayload::Playlists(playlists) => {
                        playlists.iter().map(playlist_json).collect()
                    }
                    mineral_task::SearchPayload::Artists(artists) => {
                        artists.iter().map(artist_json).collect()
                    }
                };
                emit(json!({
                    "event": "search",
                    "source": source.name(),
                    "kind": kind_name,
                    "query": query,
                    "items": items,
                }));
            }
            TaskEvent::SearchPageFailed {
                source, kind, query, ..
            } => {
                emit(json!({
                    "event": "search",
                    "source": source.name(),
                    "kind": search_kind_name(kind),
                    "query": query,
                    "items": [],
                    "error": "音源不可用或未登录",
                }));
            }
            TaskEvent::AlbumDetailFetched { id, album } => {
                let rows: Vec<Value> = album
                    .tracks
                    .iter()
                    .map(|track| cache_song(&mut self.songs, &self.loved, &track.song))
                    .collect();
                let src = id.qualified().split(':').next().unwrap_or("unknown").to_string();
                emit(json!({
                    "event": "detail",
                    "key": id.qualified(),
                    "kind": "album",
                    "title": album.name,
                    "rows": rows,
                    "cover_url": cover_of(&src, &album.cover_url),
                }));
            }
            TaskEvent::AlbumDetailFailed { id } => {
                emit(json!({
                    "event": "detail",
                    "key": id.qualified(),
                    "kind": "album",
                    "error": "专辑不可用或音源未登录",
                }));
            }
            TaskEvent::ArtistDetailFetched { id, artist } => {
                let rows: Vec<Value> = artist
                    .songs
                    .iter()
                    .map(|song| cache_song(&mut self.songs, &self.loved, song))
                    .collect();
                emit(json!({
                    "event": "detail",
                    "key": id.qualified(),
                    "kind": "artist",
                    "title": artist.name,
                    "rows": rows,
                    "subtitle": artist.description,
                }));
            }
            TaskEvent::ArtistAlbumsFetched { id, albums, .. } => {
                let rows: Vec<Value> = albums.iter().map(album_json).collect();
                emit(json!({
                    "event": "detail",
                    "key": id.qualified(),
                    "kind": "artist",
                    "title": "",
                    "rows": rows,
                }));
            }
            TaskEvent::ArtistAlbumsPageFailed { id, .. } => {
                emit(json!({
                    "event": "detail",
                    "key": id.qualified(),
                    "kind": "artist",
                    "title": "",
                    "rows": [],
                }));
            }
            TaskEvent::PlaylistWriteDone { error, .. } => {
                if let Some(error) = error {
                    emit(json!({"event":"notice","level":"error","text":format!("歌单操作失败：{error}")}));
                } else {
                    emit(json!({"event":"notice","level":"info","text":"歌单已更新"}));
                }
            }
            TaskEvent::LyricsReady { song_id, lyrics } => {
                emit(json!({
                    "event": "lyrics",
                    "key": song_id.qualified(),
                    "lines": lyric_json(&lyrics),
                }));
            }
            _ => {}
        }
    }

    async fn handle(&mut self, msg: Value) {
        let cmd = msg.get("cmd").and_then(Value::as_str).unwrap_or("");
        match cmd {
            "connect" => self.connect().await,
            "launch_daemon" => self.launch_daemon().await,
            "shutdown_daemon" => {
                if let Some(client) = self.client() {
                    let _ = client.shutdown().await;
                }
                self.client = None;
                emit(json!({"event":"connection","connected":false,"message":"播放已停止"}));
            }
            "restart_daemon" => {
                if let Some(client) = self.client() {
                    let _ = client.shutdown().await;
                }
                self.client = None;
                self.launched = None;
                emit(json!({"event":"notice","level":"info","text":"Restarting Mineral daemon to apply configuration..."}));
                tokio::time::sleep(Duration::from_millis(350)).await;
                self.launch_daemon().await;
            }
            "cycle_audio_output" => {
                let (cur_target, pending_devices) = {
                    let Some(client) = self.client() else {
                        emit(json!({"event":"notice","level":"error","text":"Daemon disconnected; cannot switch audio output device"}));
                        return;
                    };
                    let cur_target = client.mirror().read_playback(|pb| {
                        pb.anchor()
                            .output
                            .as_ref()
                            .map(|o| o.target.clone())
                            .unwrap_or(OutputTarget::SystemDefault)
                    });
                    (cur_target, client.audio_outputs())
                };
                if let Ok(pending) = pending_devices
                    && let Some(devices) = pending.outcome().await.into_success()
                {
                    let (next_target, label) = if devices.is_empty() {
                        (OutputTarget::SystemDefault, "System Default".to_owned())
                    } else {
                        match &cur_target {
                            OutputTarget::SystemDefault => {
                                let d = &devices[0];
                                (OutputTarget::Device(d.id.clone()), d.name.clone())
                            }
                            OutputTarget::Device(cur_id) => {
                                if let Some(pos) = devices.iter().position(|d| &d.id == cur_id) {
                                    if pos + 1 < devices.len() {
                                        let d = &devices[pos + 1];
                                        (OutputTarget::Device(d.id.clone()), d.name.clone())
                                    } else {
                                        (OutputTarget::SystemDefault, "System Default".to_owned())
                                    }
                                } else {
                                    (OutputTarget::SystemDefault, "System Default".to_owned())
                                }
                            }
                        }
                    };
                    let set_res = self.client().map(|c| c.set_audio_output(next_target));
                    if let Some(Ok(pending_set)) = set_res {
                        match pending_set.outcome().await {
                            mineral_client::operation::Outcome::Applied(()) => {
                                emit(json!({"event":"notice","level":"info","text":format!("Audio output switched to: {label}")}));
                            }
                            _ => {
                                emit(json!({"event":"notice","level":"error","text":"Failed to switch audio output device"}));
                            }
                        }
                    }
                } else {
                    emit(json!({"event":"notice","level":"error","text":"Failed to query audio output devices from daemon"}));
                }
            }
            "pause" => self.fire(Request::Pause),
            "resume" => self.fire(Request::Resume),
            "toggle" => {
                let playing = self
                    .client()
                    .is_some_and(|client| client.mirror().read_playback(|playback| playback.anchor().playing));
                if playing {
                    self.fire(Request::Pause);
                } else {
                    self.fire(Request::Resume);
                }
            }
            "next" => self.fire(Request::NextSong),
            "prev" => self.fire(Request::PrevOrRestart),
            "seek" => {
                let ms = msg.get("ms").and_then(Value::as_u64).unwrap_or(0);
                self.fire(Request::Seek(ms));
            }
            "volume" => {
                let pct = msg.get("pct").and_then(Value::as_u64).unwrap_or(0).min(100) as u8;
                self.fire(Request::SetVolume(pct));
            }
            "cycle_mode" => self.fire(Request::CyclePlayMode),
            "search" => self.search(&msg).await,
            "open" => self.open(&msg).await,
            "play" => self.play(&msg).await,
            "enqueue" => self.enqueue(&msg, false).await,
            "play_next" => self.enqueue(&msg, true).await,
            "queue_move" => self.queue_move(&msg).await,
            "queue_remove" => self.queue_remove(&msg).await,
            "love" => self.love(&msg).await,
            "download" => self.download(&msg).await,
            "save_playlist" => self.save_playlist(&msg).await,
            "retry" => self.fire(Request::NextSong),
            _ => emit(json!({"event":"notice","level":"error","text":format!("未知命令 {cmd}")})),
        }
    }

    fn fire(&self, request: Request) {
        let Some(client) = self.client() else {
            emit(json!({"event":"notice","level":"error","text":"Daemon 断连，操作未执行"}));
            return;
        };
        if let Err(error) = client.try_fire(request) {
            emit(json!({"event":"notice","level":"error","text":format!("操作未提交：{error}")}));
        }
    }

    fn songs_from(&self, msg: &Value) -> Option<Vec<Song>> {
        let keys = msg.get("keys")?.as_array()?;
        let mut songs = Vec::new();
        for key in keys {
            let Some(key) = key.as_str() else {
                continue;
            };
            let Some(song) = self.songs.get(key) else {
                emit(json!({"event":"notice","level":"error","text":format!("歌曲不在本地缓存：{key}")}));
                return None;
            };
            songs.push(song.clone());
        }
        if songs.is_empty() {
            None
        } else {
            Some(songs)
        }
    }

    fn context_of(&self, msg: &Value) -> QueueContextWire {
        let kind = msg.get("context").and_then(|value| value.get("kind")).and_then(Value::as_str);
        let key = msg.get("context").and_then(|value| value.get("key")).and_then(Value::as_str);
        let name = msg
            .get("context")
            .and_then(|value| value.get("name"))
            .and_then(Value::as_str)
            .map(str::to_owned);
        match kind {
            Some("search") => QueueContextWire::Search {
                query: name.unwrap_or_default(),
            },
            Some("playlist") => key.and_then(playlist_id).map_or(QueueContextWire::Unknown, |id| {
                QueueContextWire::Playlist { id, name }
            }),
            Some("album") => key.and_then(album_id).map_or(QueueContextWire::Unknown, |id| {
                QueueContextWire::Album { id, name }
            }),
            Some("artist") => key.and_then(artist_id).map_or(QueueContextWire::Unknown, |id| {
                QueueContextWire::Artist { id, name }
            }),
            Some("manual") => QueueContextWire::Manual,
            _ => QueueContextWire::Unknown,
        }
    }

    async fn play(&mut self, msg: &Value) {
        let Some(songs) = self.songs_from(msg) else {
            return;
        };
        let target = msg.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
        let context = self.context_of(msg);
        let pending = {
            let Some(client) = self.client() else {
                emit(json!({"event":"notice","level":"error","text":"Daemon 断连，无法播放"}));
                return;
            };
            client.play_queue(songs, target, context)
        };
        match pending {
            Ok(handle) => match handle.outcome().await {
                mineral_client::operation::Outcome::Failed { kind, .. } => {
                    emit(json!({"event":"play_error","text":fail_text(&kind)}));
                }
                mineral_client::operation::Outcome::Unknown { .. } => {
                    emit(json!({"event":"play_error","text":"播放未确认，Daemon 可能已断连"}));
                }
                _ => emit(json!({"event":"loading","loading":true})),
            },
            Err(error) => emit(json!({"event":"play_error","text":format!("播放未提交：{error}")})),
        }
    }

    async fn enqueue(&mut self, msg: &Value, next: bool) {
        let Some(songs) = self.songs_from(msg) else {
            return;
        };
        let context = self.context_of(msg);
        if next {
            self.fire(Request::QueueInsertNext { songs, context });
        } else {
            self.fire(Request::QueueAppend { songs, context });
        }
    }

    async fn queue_move(&mut self, msg: &Value) {
        let index = msg.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
        let key = msg.get("key").and_then(Value::as_str).unwrap_or("");
        let Some(id) = song_id(key) else {
            return;
        };
        let to = match msg.get("to").and_then(Value::as_str).unwrap_or("down") {
            "up" => QueuePos::Up,
            "top" => QueuePos::Top,
            "bottom" => QueuePos::Bottom,
            "next" => QueuePos::AfterCurrent,
            _ => QueuePos::Down,
        };
        let op = QueueOp::Move {
            at: QueueAnchor::new(index, id),
            to,
        };
        self.finish_edit(op).await;
    }

    async fn queue_remove(&mut self, msg: &Value) {
        let index = msg.get("index").and_then(Value::as_u64).unwrap_or(0) as usize;
        let key = msg.get("key").and_then(Value::as_str).unwrap_or("");
        let Some(id) = song_id(key) else {
            return;
        };
        self.finish_edit(QueueOp::Remove(QueueAnchor::new(index, id))).await;
    }

    async fn finish_edit(&mut self, op: QueueOp) {
        let Some(client) = self.client() else {
            emit(json!({"event":"notice","level":"error","text":"Daemon 断连，队列未修改"}));
            return;
        };
        match client.queue_edit(op).await {
            mineral_client::operation::Outcome::Applied(mineral_protocol::QueueEditOutcome::Stale) => {
                emit(json!({"event":"notice","level":"error","text":"队列已变化，请重试"}));
            }
            mineral_client::operation::Outcome::Failed { kind, .. } => {
                emit(json!({"event":"notice","level":"error","text":fail_text(&kind)}));
            }
            mineral_client::operation::Outcome::Unknown { .. } => {
                emit(json!({"event":"notice","level":"error","text":"队列操作未确认"}));
            }
            _ => {}
        }
    }

    async fn love(&mut self, msg: &Value) {
        let key = msg.get("key").and_then(Value::as_str).unwrap_or("");
        let Some(song) = self.songs.get(key).cloned() else {
            emit(json!({"event":"notice","level":"error","text":"找不到要收藏的歌曲"}));
            return;
        };
        let pending = {
            let Some(client) = self.client() else {
                emit(json!({"event":"notice","level":"error","text":"Daemon 断连，无法收藏"}));
                return;
            };
            client.toggle_love(song)
        };
        match pending {
            Ok(pending) => match pending.outcome().await {
                mineral_client::operation::Outcome::Applied(loved) => {
                    self.loved.insert(key.to_owned(), loved);
                    emit(json!({"event":"love","key":key,"loved":loved}));
                }
                mineral_client::operation::Outcome::Failed { kind, .. } => {
                    emit(json!({"event":"notice","level":"error","text":fail_text(&kind)}));
                }
                _ => emit(json!({"event":"notice","level":"error","text":"收藏操作未确认"})),
            },
            Err(error) => emit(json!({"event":"notice","level":"error","text":format!("收藏未提交：{error}")})),
        }
    }

    async fn download(&mut self, msg: &Value) {
        let key = msg.get("key").and_then(Value::as_str).unwrap_or("");
        if let Some(song) = self.songs.get(key).cloned() {
            self.fire(Request::Download(mineral_protocol::DownloadTarget::Song(Box::new(song))));
            return;
        }
        if let Some(id) = playlist_id(key) {
            self.fire(Request::Download(mineral_protocol::DownloadTarget::Playlist(id)));
        }
    }

    async fn save_playlist(&mut self, msg: &Value) {
        let name = msg.get("name").and_then(Value::as_str).unwrap_or("跨源歌单").to_owned();
        let pending = {
            let Some(client) = self.client() else {
                emit(json!({"event":"notice","level":"error","text":"Daemon 断连，未保存歌单"}));
                return;
            };
            client.save_queue_as_playlist(name)
        };
        match pending {
            Ok(pending) => match pending.outcome().await {
                mineral_client::operation::Outcome::Applied(id) => {
                    emit(json!({"event":"notice","level":"info","text":format!("已保存本地歌单 {}", id.qualified())}));
                }
                mineral_client::operation::Outcome::Failed { kind, .. } => {
                    emit(json!({"event":"notice","level":"error","text":fail_text(&kind)}));
                }
                _ => emit(json!({"event":"notice","level":"error","text":"保存歌单未确认"})),
            },
            Err(error) => emit(json!({"event":"notice","level":"error","text":format!("未提交：{error}")})),
        }
    }

    async fn search(&mut self, msg: &Value) {
        let query = msg.get("query").and_then(Value::as_str).unwrap_or("").trim().to_owned();
        if query.is_empty() {
            return;
        }
        let sources: Vec<String> = msg
            .get("sources")
            .and_then(Value::as_array)
            .map(|items| items.iter().filter_map(Value::as_str).map(str::to_owned).collect())
            .unwrap_or_default();
        let kinds: Vec<SearchKind> = msg
            .get("kinds")
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .filter_map(|k| match k {
                        "song" => Some(SearchKind::Song),
                        "artist" => Some(SearchKind::Artist),
                        "album" => Some(SearchKind::Album),
                        "playlist" => Some(SearchKind::Playlist),
                        _ => None,
                    })
                    .collect()
            })
            .filter(|v: &Vec<SearchKind>| !v.is_empty())
            .unwrap_or_else(|| {
                vec![
                    SearchKind::Song,
                    SearchKind::Artist,
                    SearchKind::Album,
                    SearchKind::Playlist,
                ]
            });
        let mut to_submit = Vec::new();
        for source_name in sources {
            let source = source_from(&source_name);
            let supported = self.searchable.get(&source_name);
            for &kind in &kinds {
                if let Some(list) = supported
                    && !list.contains(&kind)
                {
                    emit(json!({
                        "event": "search",
                        "source": source.name(),
                        "kind": search_kind_name(kind),
                        "query": query,
                        "items": [],
                    }));
                } else {
                    to_submit.push((source, kind));
                }
            }
        }
        let Some(client) = self.client() else {
            emit(json!({"event":"notice","level":"error","text":"Daemon 断连，无法搜索"}));
            return;
        };
        for (source, kind) in to_submit {
            let _ = client.try_fire(Request::SubmitTask(
                TaskKind::ChannelFetch(ChannelFetchKind::Search {
                    source,
                    kind,
                    query: query.clone(),
                    page: Page::new(0, 30),
                }),
                Priority::User,
            ));
        }
    }

    async fn open(&mut self, msg: &Value) {
        let Some(client) = self.client() else {
            emit(json!({"event":"notice","level":"error","text":"Daemon 断连，无法打开"}));
            return;
        };
        let key = msg.get("key").and_then(Value::as_str).unwrap_or("");
        let kind = msg.get("kind").and_then(Value::as_str).unwrap_or("");
        match kind {
            "playlist" => {
                if let Some(id) = playlist_id(key) {
                    let _ = client.try_fire(Request::SubmitTask(
                        TaskKind::ChannelFetch(ChannelFetchKind::PlaylistDetail {
                            id,
                            load: PlaylistLoad::Complete,
                        }),
                        Priority::User,
                    ));
                }
            }
            "album" => {
                if let Some(id) = album_id(key) {
                    let _ = client.try_fire(Request::SubmitTask(
                        TaskKind::ChannelFetch(ChannelFetchKind::AlbumDetail { id }),
                        Priority::User,
                    ));
                }
            }
            "artist" => {
                if let Some(id) = artist_id(key) {
                    let _ = client.try_fire(Request::SubmitTask(
                        TaskKind::ChannelFetch(ChannelFetchKind::ArtistDetail { id: id.clone() }),
                        Priority::User,
                    ));
                    let _ = client.try_fire(Request::SubmitTask(
                        TaskKind::ChannelFetch(ChannelFetchKind::ArtistAlbums {
                            id,
                            page: Page::new(0, 30),
                        }),
                        Priority::User,
                    ));
                }
            }
            "song" => {
                if let Some(id) = song_id(key) {
                    let _ = client.try_fire(Request::SubmitTask(
                        TaskKind::ChannelFetch(ChannelFetchKind::Lyrics { song_id: id }),
                        Priority::User,
                    ));
                }
            }
            _ => {}
        }
    }
}

fn search_kind_name(kind: SearchKind) -> &'static str {
    match kind {
        SearchKind::Song => "song",
        SearchKind::Album => "album",
        SearchKind::Artist => "artist",
        SearchKind::Playlist => "playlist",
        SearchKind::User => "user",
    }
}

fn pcm_json(pcm: &[f32]) -> Value {
    let mut rms = 0.0_f32;
    for sample in pcm {
        rms += sample * sample;
    }
    rms = (rms / pcm.len().max(1) as f32).sqrt();
    let bins = 48;
    let mut peaks = Vec::with_capacity(bins);
    let chunk = (pcm.len() / bins).max(1);
    for index in 0..bins {
        let start = index * chunk;
        if start >= pcm.len() {
            peaks.push(0.0);
            continue;
        }
        let end = (start + chunk).min(pcm.len());
        let mut acc = 0.0_f32;
        for sample in &pcm[start..end] {
            acc += sample.abs();
        }
        peaks.push(acc / (end - start) as f32);
    }
    let mut scope = Vec::with_capacity(96);
    let step = (pcm.len() / 96).max(1);
    let mut index = 0;
    while index < pcm.len() && scope.len() < 96 {
        scope.push(pcm[index]);
        index += step;
    }
    json!({"event":"pcm","peaks":peaks,"scope":scope,"rms":rms})
}

fn netease_config_from(
    section: &mineral_server::config::NeteaseSection,
) -> mineral_channel_netease::NeteaseConfig {
    use mineral_channel_netease::config::{AlbumCacheConfig, PlaylistFetchConfig, RequestsConfig};
    mineral_channel_netease::NeteaseConfig::builder()
        .album_cache(
            AlbumCacheConfig::builder()
                .ttl_days(*section.album_cache().ttl_days())
                .ttl_jitter_days(*section.album_cache().ttl_jitter_days())
                .build(),
        )
        .playlist_fetch(
            PlaylistFetchConfig::builder()
                .batch_size(*section.playlist_fetch().batch_size())
                .max_concurrent(*section.playlist_fetch().max_concurrent())
                .build(),
        )
        .requests(
            RequestsConfig::builder()
                .album_detail_requests_per_second(
                    *section.requests().album_detail_requests_per_second(),
                )
                .retry_delays(
                    section
                        .requests()
                        .retry_delays_ms()
                        .iter()
                        .map(|delay| Duration::from_millis(delay.get()))
                        .collect(),
                )
                .build(),
        )
        .max_connections(*section.max_connections())
        .proxy(section.proxy().clone())
        .timeout_secs(*section.timeout_secs())
        .build()
}

fn bilibili_config_from(
    section: &mineral_server::config::BilibiliSection,
) -> mineral_channel_bilibili::BilibiliConfig {
    mineral_channel_bilibili::BilibiliConfig::builder()
        .max_connections(*section.max_connections())
        .proxy(section.proxy().clone())
        .timeout_secs(*section.timeout_secs())
        .build()
}

fn run_channel_login(channel_id: &str) {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let code = rt.block_on(async move {
        let daemon_path = mineral_paths::config_dir()
            .unwrap_or_else(|_| std::env::temp_dir())
            .join("daemon.lua");
        let Ok((config, _)) = mineral_server::config::load_daemon(&daemon_path) else {
            return 1;
        };
        if channel_id == "bilibili" {
            let bc = bilibili_config_from(config.sources().bilibili());
            let cli = mineral_channel_bilibili::cli::BilibiliCli {
                command: mineral_channel_bilibili::cli::BilibiliCommand::Login,
            };
            match mineral_channel_bilibili::cli::run(cli, &bc).await {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            }
        } else {
            let nc = netease_config_from(config.sources().netease());
            let cli = mineral_channel_netease::cli::NeteaseCli {
                command: mineral_channel_netease::cli::NeteaseCommand::Login,
            };
            match mineral_channel_netease::cli::run(cli, &nc).await {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            }
        }
    });
    if code != 0 {
        std::process::exit(code);
    }
}

fn serve_embedded_daemon() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    let (cmd_tx, cmd_rx) = tokio::sync::mpsc::unbounded_channel();
    let (push_tx, push_rx) = tokio::sync::mpsc::unbounded_channel();
    let host = mineral_script::ScriptHost::new(cmd_tx.clone(), push_tx.clone());
    let dir = mineral_paths::config_dir().unwrap_or_else(|_| std::env::temp_dir());
    let daemon_path = dir.join("daemon.lua");
    let Ok(loaded) = mineral_server::config::load_daemon_with_vm(&daemon_path, |lua| {
        mineral_script::install_daemon_api(lua, &host)
    }) else {
        return;
    };

    rt.block_on(async move {
        let mineral_server::config::DaemonLoad {
            config,
            vm,
            tree: daemon_tree,
            ..
        } = loaded;
        let script = mineral_server::ScriptParts::new(vm, host, cmd_tx, cmd_rx, push_tx, push_rx);
        let persist = match mineral_paths::data_dir() {
            Ok(d) => {
                let _ = tokio::fs::create_dir_all(&d).await;
                mineral_server::ServerStore::open(&d.join("mineral.db"))
                    .await
                    .unwrap_or_else(|_| mineral_server::ServerStore::disabled())
            }
            Err(_) => mineral_server::ServerStore::disabled(),
        };

        let mut channels = Vec::<std::sync::Arc<dyn mineral_channel_core::MusicChannel>>::new();
        let mut providers = Vec::<std::sync::Arc<dyn mineral_playback::PlaybackProvider>>::new();
        channels.push(std::sync::Arc::new(
            mineral_channel_mineral::MineralChannel::new(std::sync::Arc::new(persist.clone())),
        ));
        if let Ok(d) = mineral_paths::data_dir() {
            let lib = std::sync::Arc::new(mineral_channel_local::LocalLibrary::new(
                std::sync::Arc::new(persist.scope(SourceKind::LOCAL)),
                d.join("local-covers"),
                config.sources().local().roots().clone(),
            ));
            channels.push(lib.clone());
            providers.push(lib);
        }
        if let Ok(Some(auth)) = mineral_channel_netease::load_stored() {
            let nc = netease_config_from(config.sources().netease());
            if let Ok(ch) = mineral_channel_netease::NeteaseChannel::with_credential(
                &nc,
                &auth.music_u,
                auth.user_id,
                Some(std::sync::Arc::new(persist.scope(SourceKind::NETEASE))),
            ) {
                let arc = std::sync::Arc::new(ch);
                channels.push(arc.clone());
                providers.push(arc);
            }
        }
        {
            let bc = bilibili_config_from(config.sources().bilibili());
            let ch_res = match mineral_channel_bilibili::load_stored() {
                Ok(Some(auth)) => {
                    mineral_channel_bilibili::BilibiliChannel::with_credential(&bc, &auth)
                }
                _ => mineral_channel_bilibili::BilibiliChannel::new(&bc),
            };
            if let Ok(ch) = ch_res {
                let arc = std::sync::Arc::new(ch);
                channels.push(arc.clone());
                providers.push(arc);
            }
        }
        let Ok(playback) = mineral_playback::PlaybackRegistry::new(providers) else {
            return;
        };
        let sources = mineral_server::SourceBackends::builder()
            .channels(channels)
            .playback(playback)
            .build();

        let Ok(sock_path) = socket_path() else {
            return;
        };
        if mineral_protocol::SocketWire::connect(&sock_path).await.is_ok() {
            return;
        }
        let _ = tokio::fs::remove_file(&sock_path).await;

        #[cfg(unix)]
        let listener = match tokio::net::UnixListener::bind(&sock_path) {
            Ok(l) => l,
            Err(_) => return,
        };
        #[cfg(windows)]
        let listener = {
            let Ok(l) = tokio::net::TcpListener::bind("127.0.0.1:0").await else {
                return;
            };
            let Ok(addr) = l.local_addr() else {
                return;
            };
            if tokio::fs::write(&sock_path, addr.to_string()).await.is_err() {
                return;
            }
            l
        };

        let audio_mode = mineral_server::resolve_audio_mode(
            std::env::var_os("MINERAL_AUDIO_NULL").is_some(),
            *config.audio().backend(),
        );
        let watchdog = mineral_script::WatchdogConfig::builder()
            .instruction_interval(*config.script().watchdog_instruction_interval())
            .soft_wall(Duration::from_millis(*config.script().watchdog_soft_wall_ms()))
            .hard_wall(Duration::from_millis(*config.script().watchdog_hard_wall_ms()))
            .build();
        let script_sender = script.sender().clone();
        let (script_runtime, pumps) = script.spawn_runtime(watchdog, sources.channels());
        let stats_store = match mineral_paths::data_dir() {
            Ok(d) => mineral_stats::StatsStore::open(&d.join("stats.db"))
                .await
                .unwrap_or_else(|_| mineral_stats::StatsStore::disabled()),
            Err(_) => mineral_stats::StatsStore::disabled(),
        };
        let (stats, stats_actor) = mineral_server::StatsRecorder::spawn(
            stats_store,
            mineral_server::params_from_config(config.stats()),
        );
        let stats_stop = stats.clone();
        let Ok(server) = mineral_server::Server::spawn(
            sources,
            audio_mode,
            persist,
            mineral_server::ServerConfig::from_config(&config),
            daemon_tree,
            Some(script_sender.clone()),
            stats,
        )
        .await
        else {
            return;
        };
        let reload_parts = server.attach_script_pumps(pumps);
        mineral_server::spawn_script_reloader(
            daemon_path,
            script_runtime,
            script_sender,
            reload_parts,
        );
        let _ = server.start_media_service();

        tokio::select! {
            _ = server.serve(listener) => {}
            _ = tokio::signal::ctrl_c() => {}
            _ = server.shutdown_requested() => {}
        }

        stats_stop.daemon_stopped();
        stats_stop.flush().await;
        server.shutdown().await;
        stats_stop.shutdown().await;
        drop(stats_stop);
        let _ = tokio::time::timeout(Duration::from_secs(2), stats_actor).await;
        let _ = tokio::fs::remove_file(&sock_path).await;
    });
}

async fn run_bridge() {
    let (tx, mut rx) = mpsc::unbounded_channel::<Value>();
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let Ok(line) = line else {
                break;
            };
            if line.trim().is_empty() {
                continue;
            };
            match serde_json::from_str::<Value>(&line) {
                Ok(value) => {
                    if tx.send(value).is_err() {
                        break;
                    }
                }
                Err(error) => emit(json!({"event":"notice","level":"error","text":format!("无法解析命令：{error}")})),
            }
        }
    });
    emit(json!({"event":"ready"}));
    let mut app = App::new();
    loop {
        tokio::select! {
            incoming = rx.recv() => {
                match incoming {
                    Some(msg) => {
                        if msg.get("cmd").and_then(Value::as_str) == Some("quit") {
                            break;
                        }
                        app.handle(msg).await;
                    }
                    None => break,
                }
            }
            _ = tokio::time::sleep(Duration::from_millis(200)) => {
                if app.client().is_some() {
                    app.publish();
                } else if app.client.is_some() {
                    app.client = None;
                    emit(json!({"event":"connection","connected":false,"message":"Daemon 断连"}));
                }
            }
        }
    }
    let _ = std::io::stdout().flush();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("serve") {
        serve_embedded_daemon();
        return;
    }
    if args.get(1).map(String::as_str) == Some("channel") {
        let ch = args.get(2).map(String::as_str).unwrap_or("netease");
        let sub = args.get(3).map(String::as_str).unwrap_or("login");
        if sub == "login" {
            run_channel_login(ch);
            return;
        }
    }
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    rt.block_on(run_bridge());
}
