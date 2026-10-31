use serde::{Deserialize, Serialize};

pub const SCHEMES: &[(&str, &str, &str, &str)] = &[
    ("classic", "经典三栏", "同时展示导航、内容和正在播放", "workspace"),
    ("keyboard", "键盘指令中心", "把命令入口叠加到当前工作区", "input"),
    ("spotlight", "搜索优先", "以全局搜索和结果预览为中心", "workspace"),
    ("immersive", "全屏沉浸剧场", "全屏展示封面、歌词和可视化", "overlay"),
    ("queue", "队列工作台", "把播放队列放到画面中心", "workspace"),
    ("covers", "封面墙", "用封面网格浏览已有专辑和歌单", "workspace"),
    ("single", "单栏渐进导航", "在一个主栏里逐级展开", "workspace"),
    ("mini", "迷你播放器", "收起工作区，只保留播放控制", "window"),
    ("touch", "触控与平板", "加大点击目标，并给出可见的手势替代", "input"),
    ("modular", "模块化工作台", "自由组合曲库、队列、歌词和状态模块", "workspace"),
];

pub const WORKSPACES: &[&str] = &["classic", "spotlight", "queue", "covers", "single", "modular"];
#[allow(dead_code)]
pub const MODULES: &[&str] = &[
    "library", "tracks", "queue", "lyrics", "detail", "viz", "downloads", "daemon",
];

pub fn preset(name: &str) -> Option<Vec<String>> {
    let modules: &[&str] = match name {
        "browse" => &["library", "tracks", "detail", "lyrics"],
        "immersive" => &["detail", "lyrics", "viz"],
        "queue" => &["library", "queue", "detail"],
        "simple" => &["tracks", "lyrics"],
        _ => return None,
    };
    Some(modules.iter().map(|item| (*item).to_owned()).collect())
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Item {
    pub key: String,
    #[serde(default = "song_kind")]
    pub kind: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub album: String,
    #[serde(default)]
    pub source: String,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default)]
    pub loved: bool,
    #[serde(default)]
    pub cover_url: Option<String>,
    #[serde(default)]
    pub index: Option<usize>,
    #[serde(default)]
    pub current: bool,
}

fn song_kind() -> String {
    "song".to_owned()
}

impl Item {
    pub fn nav(id: &str, title: &str, subtitle: &str) -> Self {
        Self {
            key: format!("nav:{id}"),
            kind: "nav".to_owned(),
            title: title.to_owned(),
            subtitle: subtitle.to_owned(),
            album: String::new(),
            source: id.to_owned(),
            duration_ms: None,
            unavailable: false,
            loved: false,
            cover_url: None,
            index: None,
            current: false,
        }
    }

    pub fn source_row(id: &str, title: &str, subtitle: &str) -> Self {
        Self {
            key: format!("source:{id}"),
            kind: "source".to_owned(),
            title: title.to_owned(),
            subtitle: subtitle.to_owned(),
            album: String::new(),
            source: id.to_owned(),
            duration_ms: None,
            unavailable: false,
            loved: false,
            cover_url: None,
            index: None,
            current: false,
        }
    }

    pub fn setting(key: &str, title: &str, subtitle: &str) -> Self {
        Self {
            key: key.to_owned(),
            kind: "setting".to_owned(),
            title: title.to_owned(),
            subtitle: subtitle.to_owned(),
            album: String::new(),
            source: String::new(),
            duration_ms: None,
            unavailable: false,
            loved: false,
            cover_url: None,
            index: None,
            current: false,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LyricWord {
    #[serde(default)]
    pub t: u64,
    #[serde(default)]
    pub dur: u64,
    #[serde(default)]
    pub text: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LyricLine {
    #[serde(default)]
    pub t: Option<u64>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub translation: Option<String>,
    #[serde(default)]
    pub romanization: Option<String>,
    #[serde(default)]
    pub words: Vec<LyricWord>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceInfo {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default = "yes")]
    pub available: bool,
    #[serde(default)]
    pub logged_in: bool,
    #[serde(default)]
    pub searchable: Vec<String>,
}

fn yes() -> bool {
    true
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ViewState {
    #[serde(default)]
    pub scroll: i32,
    #[serde(default)]
    pub expanded: String,
    #[serde(default = "content_focus")]
    pub focus: String,
    #[serde(default)]
    pub nav: Vec<NavCrumb>,
    #[serde(default)]
    pub cover_mode: String,
}

fn content_focus() -> String {
    "content".to_owned()
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NavCrumb {
    #[serde(default)]
    pub nav: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModeBlob {
    #[serde(default)]
    pub name: String,
    pub workspace: String,
    pub input_mode: String,
    pub window_form: String,
    pub immersive: bool,
    pub cover_mode: String,
    #[serde(default)]
    pub view: ViewState,
}

#[derive(Clone, Debug, Serialize)]
pub struct Notice {
    pub id: u64,
    pub level: String,
    pub text: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Snapshot {
    pub scheme_id: String,
    pub scheme_label: String,
    pub workspace: String,
    pub input_mode: String,
    pub window_form: String,
    pub touch: bool,
    pub keyboard: bool,
    pub immersive: bool,
    pub mini: bool,
    pub reduce_motion: bool,
    pub daemon_connected: bool,
    pub daemon_message: String,
    pub sources: Vec<SourceInfo>,
    pub source_filter: Option<String>,
    pub nav: Vec<Item>,
    pub content_kind: String,
    pub content_title: String,
    pub crumbs: Vec<String>,
    pub rows: Vec<Item>,
    pub selected_key: String,
    pub empty: String,
    pub list_filter: String,
    pub cover_mode: String,
    pub search_query: String,
    pub search_updating: bool,
    pub search_summary: String,
    pub search_songs: Vec<Item>,
    pub search_artists: Vec<Item>,
    pub search_albums: Vec<Item>,
    pub search_playlists: Vec<Item>,
    pub rec_albums: Vec<Item>,
    pub rec_genre: String,
    pub recent_searches: Vec<String>,
    pub recent_plays: Vec<Item>,
    pub queue: Vec<Item>,
    pub cursor: Option<usize>,
    pub follow_queue: bool,
    pub queue_open: bool,
    pub player_title: String,
    pub player_subtitle: String,
    pub player_source: String,
    pub player_key: String,
    pub cover_url: Option<String>,
    pub playing: bool,
    pub loading: bool,
    pub play_error: String,
    pub position_ms: u64,
    pub duration_ms: Option<u64>,
    pub volume: u8,
    pub play_mode: String,
    pub loved: bool,
    pub unavailable: bool,
    pub lyric_track: String,
    pub lyrics: Vec<LyricView>,
    pub viz_style: String,
    pub peaks: Vec<f32>,
    pub scope: Vec<f32>,
    pub waterfall: Vec<Vec<f32>>,
    pub rms: f32,
    pub notices: Vec<Notice>,
    pub single_drawer: String,
    pub panel_left: i32,
    pub panel_right: i32,
    pub modules: Vec<String>,
    pub modular_preset: String,
    pub schemes: Vec<SchemeRow>,
    pub custom_modes: Vec<String>,
    pub mode_center_open: bool,
    pub palette_open: bool,
    pub commands: Vec<CommandRow>,
    pub help_open: bool,
    pub shortcuts_enabled: bool,
    pub close_prompt: bool,
    pub close_policy: String,
    pub auto_adapt: bool,
    pub locked: bool,
    pub can_undo_auto: bool,
    pub quit: bool,
    pub theme_bg: String,
    pub theme_fg: String,
    pub theme_accent: String,
    pub settings: Vec<Item>,
    pub scroll: i32,
    pub expanded_key: String,
    pub focus_area: String,
    pub search_tab: String,
    pub mode_query: String,
    pub palette_query: String,
    pub library_items: Vec<Item>,
    pub downloads_rows: Vec<Item>,
    pub next_title: String,
    pub selected_title: String,
    pub selected_subtitle: String,
    pub selected_album: String,
    pub selected_source: String,
    pub selected_kind: String,
    pub selected_cover: Option<String>,
    pub selected_duration_ms: Option<u64>,
    pub selected_loved: bool,
    pub total_duration_ms: u64,
    pub typing: bool,
    pub generation: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct LyricView {
    pub t: Option<u64>,
    pub text: String,
    pub sub_text: String,
    pub active: bool,
    pub words: Vec<LyricWord>,
}

#[derive(Clone, Debug, Serialize)]
pub struct SchemeRow {
    pub id: String,
    pub title: String,
    pub purpose: String,
    pub current: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct CommandRow {
    pub id: String,
    pub title: String,
}

#[derive(Clone, Debug)]
pub enum DaemonEvent {
    Connection { connected: bool, message: String },
    Sources { sources: Vec<SourceInfo>, message: String },
    Player {
        playing: bool,
        position_ms: u64,
        duration_ms: Option<u64>,
        volume: Option<u8>,
        queue: Vec<Item>,
        cursor: Option<usize>,
        mode: String,
        current: Option<Item>,
        lyrics: Vec<LyricLine>,
    },
    Library { playlists: Vec<Item>, partial: bool },
    Search {
        source: String,
        kind: String,
        query: String,
        items: Vec<Item>,
        error: String,
    },
    Detail {
        key: String,
        kind: String,
        title: String,
        rows: Vec<Item>,
        error: String,
    },
    Loved { keys: Vec<String> },
    Love { key: String, loved: bool },
    Downloads { items: Vec<DownloadRow> },
    Pcm { peaks: Vec<f32>, scope: Vec<f32>, rms: f32 },
    Lyrics { key: String, lines: Vec<LyricLine> },
    Notice { level: String, text: String },
    PlayError(String),
    Loading(bool),
    Ready,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DownloadRow {
    #[serde(default)]
    pub song: Option<Item>,
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub failure: Option<String>,
}
