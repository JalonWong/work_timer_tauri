use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Settings {
    file_name: PathBuf,
    info: SettingInfo,
}

impl Settings {
    pub fn new() -> Self {
        let mut file_name = get_config_dir();

        let mut cache_name = file_name.clone();
        cache_name.push("cache.toml");

        file_name.push("settings.toml");

        let info = Self::load_settings(&file_name);

        Self { file_name, info }
    }

    fn load_settings(file_name: &Path) -> SettingInfo {
        let mut need_save = true;

        let mut info = SettingInfo {
            theme: "System".to_string(),
            audio_file: "assets/notify.wav".to_string(),
            play_audio: true,
            timer_list: vec![
                TimerSetting {
                    label: "\u{2615} Break".to_string(),
                    limit_time: 5,
                    for_work: false,
                    count_up: false,
                    notify: true,
                },
                TimerSetting {
                    label: "\u{1F4BB} Work".to_string(),
                    limit_time: 25,
                    for_work: true,
                    count_up: true,
                    notify: false,
                },
            ],
        };

        // Load
        if file_name.exists() {
            let toml_str = fs::read_to_string(file_name).unwrap();
            if let Ok(i) = toml::from_str(&toml_str) {
                info = i;
                need_save = false;
            }
        }

        // Save
        if need_save {
            fs::write(file_name, toml::to_string(&info).unwrap()).unwrap();
        }
        info
    }

    pub fn save(&self) {
        fs::write(&self.file_name, toml::to_string(&self.info).unwrap()).unwrap();
    }

    pub fn theme(&self) -> &str {
        &self.info.theme
    }

    pub fn set_theme(&mut self, theme: &str) {
        self.info.theme = theme.to_string();
    }

    pub fn timer_list(&self) -> &[TimerSetting] {
        &self.info.timer_list
    }

    pub fn mut_timer_list(&mut self) -> &mut Vec<TimerSetting> {
        &mut self.info.timer_list
    }

    pub fn audio_file(&self) -> Option<&str> {
        if self.info.play_audio {
            Some(&self.info.audio_file)
        } else {
            None
        }
    }

    pub fn mut_audio_file(&mut self) -> &mut String {
        &mut self.info.audio_file
    }

    pub fn set_play_audio(&mut self, v: bool) {
        self.info.play_audio = v;
    }

    pub fn play_audio(&self) -> bool {
        self.info.play_audio
    }
}

pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    #[cfg(debug_assertions)]
    path.push("work_timer_tauri_dbg");
    #[cfg(not(debug_assertions))]
    path.push("work_timer_tauri");

    if !path.exists() {
        fs::create_dir_all(&path).unwrap();
    }
    path
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
struct SettingInfo {
    theme: String,
    play_audio: bool,
    audio_file: String,
    timer_list: Vec<TimerSetting>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TimerSetting {
    pub label: String,
    /// in minutes
    pub limit_time: u64,
    pub for_work: bool,
    pub count_up: bool,
    pub notify: bool,
}

impl Default for TimerSetting {
    fn default() -> Self {
        Self {
            label: "new".to_string(),
            limit_time: 1,
            for_work: false,
            count_up: false,
            notify: false,
        }
    }
}
