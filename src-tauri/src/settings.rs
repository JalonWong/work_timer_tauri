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
    pub fn new(config_dir: &Path) -> Self {
        let file_name = config_dir.join("settings.toml");
        let info = Self::load_settings(&file_name);
        Self { file_name, info }
    }

    fn load_settings(file_name: &Path) -> SettingInfo {
        let mut need_save = true;

        let mut info = SettingInfo {
            theme: "".to_string(),
            timer_list: vec![
                TimerSetting::new("\u{2615} Break", false),
                TimerSetting::new("\u{1F4BB} Work", true),
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
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
struct SettingInfo {
    theme: String,
    timer_list: Vec<TimerSetting>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TimerSetting {
    pub label: String,
    /// in minutes
    pub limit_time: u64,
    pub save_history: bool,
    pub count_up: bool,
    pub play_a_sound: bool,
    pub notification: bool,
    pub show_window: bool,
}

impl TimerSetting {
    fn new(label: &str, save_history: bool) -> Self {
        Self {
            label: label.to_string(),
            limit_time: 1,
            save_history,
            count_up: false,
            play_a_sound: false,
            notification: false,
            show_window: false,
        }
    }
}
