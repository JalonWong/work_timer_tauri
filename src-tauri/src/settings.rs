use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Settings {
    cache_name: PathBuf,
    cache_info: CacheInfo,
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
        let mut cache_info = Self::load_cache(&cache_name);
        if !info.tags.contains(&cache_info.tag) {
            cache_info.tag = info.tags.get(0).map_or("".to_string(), |v| v.clone());
        }

        Self {
            cache_name,
            cache_info,
            file_name,
            info,
        }
    }

    fn load_settings(file_name: &Path) -> SettingInfo {
        let mut need_save = true;

        let mut info = SettingInfo {
            theme: "System".to_string(),
            audio_file: "assets/notify.wav".to_string(),
            play_audio: true,
            tags: vec![
                "Programming".to_string(),
                "English".to_string(),
                "Reading".to_string(),
            ],
            timer_list: vec![
                TimerSetting {
                    name: "Break".to_string(),
                    icon: "\u{2615}".to_string(),
                    limit_time: 5,
                    for_work: false,
                    count_up: false,
                    notify: true,
                },
                TimerSetting {
                    name: "Work".to_string(),
                    icon: "\u{1F4BB}".to_string(),
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

    fn load_cache(file_name: &Path) -> CacheInfo {
        let mut info = CacheInfo {
            window: None,
            tag: "".to_string(),
        };

        if file_name.exists() {
            let toml_str = fs::read_to_string(file_name).unwrap();
            if let Ok(i) = toml::from_str(&toml_str) {
                info = i;
            }
        }
        info
    }

    pub fn save(&self) {
        fs::write(&self.file_name, toml::to_string(&self.info).unwrap()).unwrap();
    }

    pub fn save_cache(&self) {
        fs::write(&self.cache_name, toml::to_string(&self.cache_info).unwrap()).unwrap();
    }

    pub fn window_info(&self) -> Option<WindowInfo> {
        self.cache_info.window.clone()
    }

    pub fn set_window_info(&mut self, info: &WindowInfo) {
        self.cache_info.window = Some(info.clone());
    }

    pub fn tags(&self) -> &[String] {
        self.info.tags.as_slice()
    }

    pub fn mut_tags(&mut self) -> &mut Vec<String> {
        &mut self.info.tags
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

    pub fn add_timer(&mut self, timer: TimerSetting) {
        self.info.timer_list.push(timer);
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

    pub fn set_current_tag(&mut self, v: &str) {
        self.cache_info.tag = v.to_string();
    }

    pub fn current_tag(&self) -> &str {
        &self.cache_info.tag
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
struct CacheInfo {
    window: Option<WindowInfo>,
    tag: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct WindowInfo {
    pub maximized: bool,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

// ----------------------------------------------------------------------------

#[derive(Deserialize, Serialize)]
struct SettingInfo {
    theme: String,
    play_audio: bool,
    audio_file: String,
    tags: Vec<String>,
    timer_list: Vec<TimerSetting>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TimerSetting {
    pub name: String,
    pub icon: String,
    /// in minutes
    pub limit_time: u64,
    pub for_work: bool,
    pub count_up: bool,
    pub notify: bool,
}

impl TimerSetting {
    pub fn new() -> Self {
        Self {
            name: "new".to_string(),
            icon: String::new(),
            limit_time: 1,
            for_work: false,
            count_up: false,
            notify: false,
        }
    }
}
