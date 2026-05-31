use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const DATA_FILE: &str = "data.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameData {
    pub volume: f32,
    pub music_volume: f32,
    pub fullscreen: bool,
    pub color: String,
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            volume: 0.5,
            music_volume: 0.5,
            fullscreen: false,
            color: "Green".to_string(),
        }
    }
}

impl GameData {
    pub fn load() -> Self {
        if !Path::new(DATA_FILE).exists() {
            let data = Self::default();
            data.save();
            return data;
        }

        fs::read_to_string(DATA_FILE)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(DATA_FILE, json);
        }
    }
}