//! Shared state for the Mineral Qt and GTK shells.
//!
//! Playback stays in the Mineral daemon. This crate only remembers UI modes,
//! translates commands, and folds daemon events into one snapshot.

mod backend;
pub mod daemon_config;
mod fuzzy;
mod model;
mod prefs;
mod session;

pub use backend::{parse_event, Backend, BridgeBackend, MemoryBackend};
pub use daemon_config::DaemonConfigData;
pub use model::{DaemonEvent, Item, LyricLine, Snapshot, SourceInfo, SCHEMES};
pub use session::{prefs_path_default as session_path, Session};

pub fn is_autotest() -> bool {
    std::env::var_os("MINERAL_GUI_AUTOTEST").is_some_and(|v| v == "1")
        || std::env::args().any(|arg| arg == "--self-test")
}

pub fn autotest_step(step: usize) -> Option<serde_json::Value> {
    use serde_json::json;
    Some(match step {
        1 => json!({"action": "connect"}),
        4 => json!({"action": "search", "query": "洛天依"}),
        8 => json!({"action": "search_tab", "tab": "artist"}),
        10 => json!({"action": "search_tab", "tab": "all"}),
        12 => json!({"action": "open", "key": "bilibili:66824231", "kind": "artist"}),
        16 => json!({"action": "back"}),
        18 => json!({"action": "scheme", "scheme": "keyboard"}),
        20 => json!({"action": "scheme", "scheme": "spotlight"}),
        22 => json!({"action": "scheme", "scheme": "immersive"}),
        24 => json!({"action": "back"}),
        26 => json!({"action": "scheme", "scheme": "queue"}),
        28 => json!({"action": "scheme", "scheme": "covers"}),
        29 => json!({"action": "cover_mode", "mode": "list"}),
        30 => json!({"action": "cover_mode", "mode": "grid"}),
        32 => json!({"action": "scheme", "scheme": "single"}),
        33 => json!({"action": "single_drawer", "drawer": "queue"}),
        34 => json!({"action": "single_drawer", "drawer": "lyrics"}),
        36 => json!({"action": "scheme", "scheme": "mini"}),
        38 => json!({"action": "back"}),
        40 => json!({"action": "scheme", "scheme": "touch"}),
        42 => json!({"action": "scheme", "scheme": "modular"}),
        43 => json!({"action": "modular_preset", "preset": "lyrics"}),
        44 => json!({"action": "toggle_module", "module": "visualizer"}),
        46 => json!({"action": "mode_center", "open": true}),
        47 => json!({"action": "mode_query", "query": "封面"}),
        48 => json!({"action": "mode_center", "open": false}),
        50 => json!({"action": "palette", "open": true}),
        51 => json!({"action": "palette_query", "query": ">模式"}),
        52 => json!({"action": "palette", "open": false}),
        54 => json!({"action": "help", "open": true}),
        55 => json!({"action": "help", "open": false}),
        57 => json!({"action": "reset_layout"}),
        58 => json!({"action": "reset_modes"}),
        60 => json!({"action": "confirm_close", "choice": "ui"}),
        _ => return None,
    })
}
