use std::path::PathBuf;

use mineral_gui_core::{DaemonEvent, Item, MemoryBackend, Session, SourceInfo, SCHEMES};
use serde_json::json;

fn song(key: &str, title: &str) -> Item {
    Item {
        key: key.to_owned(),
        kind: "song".to_owned(),
        title: title.to_owned(),
        subtitle: "artist".to_owned(),
        album: "album".to_owned(),
        source: "netease".to_owned(),
        duration_ms: Some(180_000),
        unavailable: false,
        loved: false,
        cover_url: None,
        index: None,
        current: false,
    }
}

fn online(session: &mut Session<MemoryBackend>) {
    session.backend_mut().push(DaemonEvent::Connection { connected: true, message: "已连接 Mineral daemon".into() });
    session.backend_mut().push(DaemonEvent::Sources {
        sources: vec![
            SourceInfo { id: "netease".into(), label: "♫ netease".into(), available: true, logged_in: true, searchable: vec!["song".into()] },
            SourceInfo { id: "bilibili".into(), label: "bilibili".into(), available: true, logged_in: true, searchable: vec!["song".into()] },
        ],
        message: String::new(),
    });
    session.tick();
}

fn play(session: &mut Session<MemoryBackend>, tracks: Vec<Item>) {
    let current = tracks.first().cloned();
    session.backend_mut().push(DaemonEvent::Player {
        playing: true,
        position_ms: 1200,
        duration_ms: Some(180_000),
        volume: Some(40),
        queue: tracks,
        cursor: Some(0),
        mode: "repeat_all".into(),
        current,
        lyrics: Vec::new(),
    });
    session.tick();
}

fn act(session: &mut Session<MemoryBackend>, action: &str, extra: serde_json::Value) {
    session.dispatch_action(action, extra);
}

#[test]
fn mode_center_lists_ten_schemes() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    let snap = session.snapshot();
    assert_eq!(snap.schemes.len(), 10);
    assert_eq!(snap.schemes.iter().map(|item| item.id.as_str()).collect::<Vec<_>>(), SCHEMES.iter().map(|item| item.0).collect::<Vec<_>>());
    session.close();
}

#[test]
fn switching_modes_keeps_one_queue_and_search() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    play(&mut session, vec![song("netease:1", "甲"), song("netease:2", "乙")]);
    act(&mut session, "search", json!({"query": "mineral"}));
    for (scheme, _, _, _) in SCHEMES {
        act(&mut session, "scheme", json!({"scheme": scheme}));
        let snap = session.snapshot();
        assert_eq!(snap.queue.iter().map(|row| (row.key.as_str(), row.title.as_str())).collect::<Vec<_>>(), vec![("netease:1", "甲"), ("netease:2", "乙")]);
        assert_eq!(snap.player_title, "甲");
        assert_eq!(snap.play_mode, "repeat_all");
        assert_eq!(snap.search_query, "mineral");
        assert!(!snap.quit);
    }
    session.close();
}

#[test]
fn keyboard_touch_and_windows_restore_context() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    act(&mut session, "scheme", json!({"scheme": "queue"}));
    act(&mut session, "report_view", json!({"scroll": 42, "focus": "queue", "expanded": "netease:9"}));
    act(&mut session, "scheme", json!({"scheme": "keyboard"}));
    assert_eq!(session.snapshot().workspace, "queue");
    assert!(session.snapshot().keyboard);
    act(&mut session, "scheme", json!({"scheme": "touch"}));
    assert!(session.snapshot().touch);
    assert_eq!(session.snapshot().workspace, "queue");
    act(&mut session, "scheme", json!({"scheme": "immersive"}));
    assert!(session.snapshot().immersive);
    assert_eq!(session.snapshot().workspace, "queue");
    act(&mut session, "back", json!({}));
    assert!(!session.snapshot().immersive);
    act(&mut session, "scheme", json!({"scheme": "mini"}));
    assert!(session.snapshot().mini);
    act(&mut session, "back", json!({}));
    assert!(!session.snapshot().mini);
    assert_eq!(session.snapshot().workspace, "queue");
    act(&mut session, "scheme", json!({"scheme": "classic"}));
    assert_eq!(session.snapshot().scroll, 0);
    act(&mut session, "scheme", json!({"scheme": "queue"}));
    assert_eq!(session.snapshot().scroll, 42);
    session.close();
}

#[test]
fn queue_commands_refused_when_disconnected_and_shared_later() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    act(&mut session, "queue_move", json!({"index": 0, "to": "up"}));
    assert!(session.queue.is_empty());
    assert!(session.backend_mut().commands.is_empty());
    assert!(session.snapshot().notices.iter().any(|notice| notice.level == "error"));
    online(&mut session);
    play(&mut session, vec![song("netease:1", "甲"), song("netease:2", "乙")]);
    act(&mut session, "scheme", json!({"scheme": "queue"}));
    act(&mut session, "queue_move", json!({"index": 1, "to": "top"}));
    assert_eq!(session.backend_mut().commands.last().unwrap().0, "queue_move");
    act(&mut session, "scheme", json!({"scheme": "classic"}));
    act(&mut session, "queue_remove", json!({"index": 0}));
    assert_eq!(session.backend_mut().commands.last().unwrap().0, "queue_remove");
    session.close();
}

#[test]
fn search_states_keep_previous_results_while_updating() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    act(&mut session, "search", json!({"query": "old"}));
    session.backend_mut().push(DaemonEvent::Search {
        source: "netease".into(),
        kind: "song".into(),
        query: "old".into(),
        items: vec![song("netease:9", "旧歌")],
        error: String::new(),
    });
    session.tick();
    assert_eq!(session.snapshot().search_songs[0].title, "旧歌");
    act(&mut session, "search", json!({"query": "new"}));
    assert_eq!(session.snapshot().search_songs[0].title, "旧歌");
    assert_eq!(session.snapshot().search_summary, "正在更新搜索结果");
    session.backend_mut().push(DaemonEvent::Search { source: "netease".into(), kind: "song".into(), query: "new".into(), items: vec![song("netease:3", "新歌")], error: String::new() });
    session.backend_mut().push(DaemonEvent::Search { source: "bilibili".into(), kind: "song".into(), query: "new".into(), items: vec![], error: "音源不可用或未登录".into() });
    for kind in ["artist", "album", "playlist"] {
        for source in ["netease", "bilibili"] {
            session.backend_mut().push(DaemonEvent::Search { source: source.into(), kind: kind.into(), query: "new".into(), items: vec![], error: String::new() });
        }
    }
    session.tick();
    assert!(session.snapshot().search_summary.contains("部分音源失败"));
    session.close();
}

#[test]
fn custom_mode_persists_and_reset_keeps_policy() {
    let dir = tempfile();
    let path = dir.join("state.json");
    let mut session = Session::open(MemoryBackend::new(), &path);
    online(&mut session);
    act(&mut session, "scheme", json!({"scheme": "covers"}));
    act(&mut session, "scheme", json!({"scheme": "touch"}));
    act(&mut session, "save_combo", json!({"name": "封面触控"}));
    session.close();
    let mut again = Session::open(MemoryBackend::new(), &path);
    act(&mut again, "apply_combo", json!({"name": "封面触控"}));
    assert_eq!(again.snapshot().workspace, "covers");
    assert!(again.snapshot().touch);
    act(&mut again, "reset_layout", json!({}));
    act(&mut again, "reset_modes", json!({}));
    assert_eq!(again.snapshot().workspace, "classic");
    assert_eq!(again.snapshot().modular_preset, "browse");
    assert!(again.queue.is_empty());
    again.close();
}

#[test]
fn auto_switch_is_undoable_and_lock_wins() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    act(&mut session, "scheme", json!({"scheme": "classic"}));
    act(&mut session, "window_width", json!({"width": 640}));
    assert_eq!(session.snapshot().workspace, "single");
    act(&mut session, "undo_auto", json!({}));
    assert_eq!(session.snapshot().workspace, "classic");
    act(&mut session, "lock", json!({"on": true}));
    act(&mut session, "window_width", json!({"width": 500}));
    assert_eq!(session.snapshot().workspace, "classic");
    act(&mut session, "auto_adapt", json!({"on": false}));
    act(&mut session, "lock", json!({"on": false}));
    act(&mut session, "window_width", json!({"width": 500}));
    assert_eq!(session.snapshot().workspace, "classic");
    session.close();
}

#[test]
fn close_and_daemon_failure_are_global() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    act(&mut session, "scheme", json!({"scheme": "spotlight"}));
    session.backend_mut().push(DaemonEvent::Connection { connected: false, message: "Daemon 断连".into() });
    session.tick();
    assert!(!session.snapshot().daemon_connected);
    assert!(session.snapshot().empty.contains("Daemon 断连"));
    act(&mut session, "request_close", json!({}));
    assert!(session.snapshot().close_prompt);
    assert!(!session.snapshot().quit);
    act(&mut session, "confirm_close", json!({"choice": "ui"}));
    assert!(session.snapshot().quit);
    assert!(session.backend_mut().commands.iter().all(|(name, _)| name != "shutdown_daemon"));
    session.close();
}

#[test]
fn no_recommendation_or_remote_management() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    let blob = serde_json::to_string(&session.snapshot()).unwrap();
    for word in ["推荐", "排行榜", "删除远端", "重命名远端"] {
        assert!(!blob.contains(word), "{word}");
    }
    let snap = session.snapshot();
    let titles: Vec<_> = snap.nav.iter().map(|item| item.title.as_str()).collect();
    assert_eq!(titles, ["曲库", "歌单", "收藏", "下载", "设置"]);
    session.close();
}

#[test]
fn batch_enqueue_and_typing_and_contrast() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    session.backend_mut().push(DaemonEvent::Search { source: "netease".into(), kind: "song".into(), query: String::new(), items: vec![song("netease:1", "甲"), song("netease:2", "乙")], error: String::new() });
    session.tick();
    act(&mut session, "select_many", json!({"keys": ["netease:1", "netease:2"]}));
    act(&mut session, "enqueue", json!({}));
    assert_eq!(session.backend_mut().commands.last().unwrap().0, "enqueue");
    assert!(session.snapshot().notices.iter().any(|notice| notice.text.contains("2 首")));
    act(&mut session, "set_typing", json!({"on": true}));
    assert!(!session.snapshot().shortcuts_enabled);
    act(&mut session, "set_shortcut", json!({"action": "next", "key": "Ctrl+N"}));
    act(&mut session, "lyric_track", json!({}));
    act(&mut session, "viz", json!({"style": "terrain"}));
    act(&mut session, "theme", json!({"r": 250, "g": 250, "b": 240}));
    let snap = session.snapshot();
    assert_eq!(snap.viz_style, "terrain");
    assert_eq!(snap.theme_fg, "#161616");
    session.close();
}

#[test]
fn search_works_in_classic_mode_and_supports_tabs_and_back() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    act(&mut session, "search", json!({"query": "洛天依"}));
    session.backend_mut().push(DaemonEvent::Search {
        source: "bilibili".into(),
        kind: "artist".into(),
        query: "洛天依".into(),
        items: vec![Item {
            key: "bilibili:66824231".into(),
            kind: "artist".into(),
            title: "洛天依".into(),
            subtitle: "bilibili · 艺人".into(),
            source: "bilibili".into(),
            ..Default::default()
        }],
        error: String::new(),
    });
    session.backend_mut().push(DaemonEvent::Search {
        source: "bilibili".into(),
        kind: "album".into(),
        query: "洛天依".into(),
        items: vec![Item {
            key: "bilibili:BV1xx".into(),
            kind: "album".into(),
            title: "洛天依合集".into(),
            subtitle: "bilibili".into(),
            source: "bilibili".into(),
            ..Default::default()
        }],
        error: String::new(),
    });
    session.tick();

    let snap = session.snapshot();
    assert_eq!(snap.content_kind, "search");
    assert_eq!(snap.rows.len(), 2);

    act(&mut session, "search_tab", json!({"tab": "artist"}));
    assert_eq!(session.snapshot().rows.len(), 1);
    assert_eq!(session.snapshot().rows[0].kind, "artist");

    act(&mut session, "search_tab", json!({"tab": "all"}));
    act(&mut session, "back", json!({}));
    assert_ne!(session.snapshot().content_kind, "search");
    session.close();
}

#[test]
fn artist_detail_merges_albums_and_supports_nested_navigation() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    act(&mut session, "open", json!({"key": "bilibili:66824231", "kind": "artist"}));
    session.backend_mut().push(DaemonEvent::Detail {
        key: "bilibili:66824231".into(),
        kind: "artist".into(),
        title: "洛天依".into(),
        rows: Vec::new(),
        error: String::new(),
    });
    session.backend_mut().push(DaemonEvent::Detail {
        key: "bilibili:66824231".into(),
        kind: "artist".into(),
        title: "洛天依".into(),
        rows: vec![Item {
            key: "bilibili:BV1abc".into(),
            kind: "album".into(),
            title: "三月雨".into(),
            subtitle: "洛天依".into(),
            source: "bilibili".into(),
            ..Default::default()
        }],
        error: String::new(),
    });
    session.tick();

    let snap = session.snapshot();
    assert_eq!(snap.content_title, "洛天依");
    assert_eq!(snap.rows.len(), 1);
    assert_eq!(snap.rows[0].key, "bilibili:BV1abc");

    act(&mut session, "activate", json!({"key": "bilibili:BV1abc"}));
    session.backend_mut().push(DaemonEvent::Detail {
        key: "bilibili:BV1abc".into(),
        kind: "album".into(),
        title: "三月雨".into(),
        rows: vec![song("bilibili:BV1abc_p1", "三月雨 P1")],
        error: String::new(),
    });
    session.tick();

    let snap2 = session.snapshot();
    assert_eq!(snap2.content_title, "三月雨");
    assert_eq!(snap2.crumbs, vec!["曲库", "洛天依", "三月雨"]);
    assert_eq!(snap2.rows.len(), 1);

    act(&mut session, "back", json!({}));
    assert_eq!(session.snapshot().content_title, "洛天依");
    session.close();
}

#[test]
fn lyric_fetch_is_deduplicated_across_player_ticks() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    play(&mut session, vec![song("netease:1", "甲")]);
    let open_count_1 = session.backend_mut().commands.iter().filter(|(cmd, _)| cmd == "open").count();
    assert_eq!(open_count_1, 1);

    // Subsequent tick for the same song with empty lyrics must not re-send "open"
    play(&mut session, vec![song("netease:1", "甲")]);
    let open_count_2 = session.backend_mut().commands.iter().filter(|(cmd, _)| cmd == "open").count();
    assert_eq!(open_count_2, 1);
    session.close();
}

#[test]
fn player_and_download_ticks_do_not_bump_generation_or_reset_selection() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);
    play(&mut session, vec![song("netease:1", "甲")]);
    session.backend_mut().push(DaemonEvent::Lyrics {
        key: "netease:1".into(),
        lines: vec![mineral_gui_core::LyricLine {
            t: Some(0),
            text: "第一句歌词".into(),
            ..Default::default()
        }],
    });
    session.tick();
    act(&mut session, "select", json!({"key": "source:bilibili"}));
    let gen_before = session.snapshot().generation;
    assert_eq!(session.snapshot().selected_key, "source:bilibili");

    // Push unchanged Player (with empty lyrics) and unchanged Downloads
    session.backend_mut().push(DaemonEvent::Player {
        playing: true,
        position_ms: 4800,
        duration_ms: Some(180_000),
        volume: Some(40),
        queue: vec![song("netease:1", "甲")],
        cursor: Some(0),
        mode: "repeat_all".into(),
        current: Some(song("netease:1", "甲")),
        lyrics: Vec::new(),
    });
    session.backend_mut().push(DaemonEvent::Downloads { items: Vec::new() });
    session.tick();

    let snap = session.snapshot();
    assert_eq!(snap.generation, gen_before);
    assert_eq!(snap.selected_key, "source:bilibili");
    assert_eq!(snap.lyrics.len(), 1);
    session.close();
}

#[test]
fn live_bridge_backend_lifecycle_and_autotest_steps() {
    let dir = tempfile();
    let mut session = Session::open(mineral_gui_core::BridgeBackend::launch(), dir.join("state.json"));
    for step in 1..=60 {
        session.tick();
        if let Some(action) = mineral_gui_core::autotest_step(step) {
            session.dispatch_value(&action);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let snap = session.snapshot();
    assert!(snap.quit);
    session.close();
}

#[test]
fn daemon_config_gui_roundtrip_and_full_schema_coverage() {
    let dir = tempfile();
    let mut session = Session::open(MemoryBackend::new(), dir.join("state.json"));
    online(&mut session);

    let snap = session.snapshot();
    for expected_key in [
        "settings:daemon:restart",
        "settings:daemon:audio_output",
        "settings:daemon:edit_lua",
        "settings:daemon:audio.playback_quality",
        "settings:daemon:audio.volume",
        "settings:daemon:audio.backend",
        "settings:daemon:audio.engine_tick_ms",
        "settings:daemon:audio.prefetch_bytes",
        "settings:daemon:audio.tap_capacity",
        "settings:daemon:audio.envelope.points",
        "settings:daemon:cache.audio_capacity",
        "settings:daemon:download.quality",
        "settings:daemon:download.dir",
        "settings:daemon:download.max_concurrent",
        "settings:daemon:download.tagging",
        "settings:daemon:sources.local.roots",
        "settings:daemon:sources.netease.proxy",
        "settings:daemon:sources.bilibili.proxy",
        "settings:daemon:gapless_prefetch_ms",
        "settings:daemon:channel_workers_per",
        "settings:daemon:script.watchdog_instruction_interval",
        "settings:daemon:stats.level",
        "settings:daemon:stats.retention_days",
    ] {
        assert!(
            snap.settings.iter().any(|item| item.key == expected_key),
            "Missing setting row: {expected_key}"
        );
    }

    // Cycle playback quality: exhigh -> lossless
    act(
        &mut session,
        "daemon_config_set",
        json!({"key": "audio.playback_quality", "value": "__cycle__"}),
    );
    assert_eq!(session.daemon_config.audio_playback_quality, "lossless");

    // Set cache capacity using human shorthand and local roots
    act(
        &mut session,
        "daemon_config_set",
        json!({"key": "cache.audio_capacity", "value": "20GB"}),
    );
    assert_eq!(
        session.daemon_config.cache_audio_capacity,
        20 * 1024 * 1024 * 1024
    );

    act(
        &mut session,
        "daemon_config_set",
        json!({"key": "sources.local.roots", "value": "/home/user/Music, /mnt/flac"}),
    );
    assert_eq!(
        session.daemon_config.sources_local_roots,
        vec!["/home/user/Music".to_owned(), "/mnt/flac".to_owned()]
    );

    // Reload from generated daemon.lua and verify roundtrip
    let loaded = mineral_gui_core::DaemonConfigData::load_from_path(&session.daemon_lua_path);
    assert_eq!(loaded.audio_playback_quality, "lossless");
    assert_eq!(loaded.cache_audio_capacity, 20 * 1024 * 1024 * 1024);
    assert_eq!(
        loaded.sources_local_roots,
        vec!["/home/user/Music".to_owned(), "/mnt/flac".to_owned()]
    );

    session.close();
}

fn tempfile() -> PathBuf {
    let path = std::env::temp_dir().join(format!("mineral-gui-{}", std::process::id()));
    let path = path.join(format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
    std::fs::create_dir_all(&path).unwrap();
    path
}
