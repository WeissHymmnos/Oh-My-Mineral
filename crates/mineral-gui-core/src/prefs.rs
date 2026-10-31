use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::model::{ModeBlob, ViewState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Prefs {
    pub version: u32,
    pub preferred_workspace: String,
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
    pub custom_modes: Vec<ModeBlob>,
    pub modular_preset: String,
    pub modules: Vec<String>,
    pub panel_left: i32,
    pub panel_right: i32,
    pub view: BTreeMap<String, ViewState>,
    pub recent_searches: Vec<String>,
    pub recent_plays: Vec<String>,
    #[serde(default)]
    pub load_error: String,
}

impl Default for Prefs {
    fn default() -> Self {
        Self {
            version: 1,
            preferred_workspace: "classic".to_owned(),
            input_mode: "pointer".to_owned(),
            window_form: "full".to_owned(),
            immersive: false,
            lyric_track: "original".to_owned(),
            viz_style: "bars".to_owned(),
            reduce_motion: false,
            auto_adapt: true,
            locked: false,
            close_policy: "ask".to_owned(),
            follow_queue: true,
            cover_mode: "grid".to_owned(),
            source_filter: None,
            shortcuts: BTreeMap::new(),
            custom_modes: Vec::new(),
            modular_preset: "browse".to_owned(),
            modules: vec![
                "library".into(),
                "tracks".into(),
                "detail".into(),
                "lyrics".into(),
            ],
            panel_left: 240,
            panel_right: 320,
            view: BTreeMap::new(),
            recent_searches: Vec::new(),
            recent_plays: Vec::new(),
            load_error: String::new(),
        }
    }
}

pub fn load(path: &Path) -> Prefs {
    let mut prefs = Prefs::default();
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return prefs,
    };
    match serde_json::from_str::<Prefs>(&text) {
        Ok(loaded) => loaded,
        Err(_) => {
            prefs.load_error = "布局配置无法读取，已使用默认模式".to_owned();
            prefs
        }
    }
}

pub fn save(path: &Path, prefs: &Prefs) {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if let Ok(text) = serde_json::to_string_pretty(prefs) {
        let _ = fs::write(path, text);
    }
}
