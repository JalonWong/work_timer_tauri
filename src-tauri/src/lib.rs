use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{fs, sync::Mutex, time::SystemTime};
use tauri::{Manager, State};
use tauri_plugin_window_state::StateFlags;

mod history;
mod settings;
mod timer;

use history::History;
use settings::{Settings, TimerSetting};
use timer::Timer;

struct AppState {
    settings: Mutex<Settings>,
    history: Mutex<History>,
    count_timer: Mutex<Timer>,
    state: Mutex<MainState>,
}

struct MainState {
    total_time: u64,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .with_state_flags(
                    StateFlags::POSITION
                        | StateFlags::SIZE
                        | StateFlags::MAXIMIZED
                        | StateFlags::FULLSCREEN,
                )
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            cmd_start_timer,
            cmd_stop_timer,
            cmd_get_timer_count,
            cmd_get_timer_status,
            cmd_get_history,
            cmd_delete_record,
            cmd_modify_record,
            cmd_export_to_csv,
            cmd_get_settings,
            cmd_save_settings,
        ])
        .setup(|app| {
            let mut data_dir = app.path().app_data_dir().unwrap();
            #[cfg(debug_assertions)]
            data_dir.push("debug");

            if !data_dir.exists() {
                fs::create_dir_all(&data_dir).unwrap();
            }

            let settings = Settings::new(&data_dir);
            let history = History::new(&data_dir);
            let total_time = init_total_time(&history);
            app.manage(AppState {
                settings: Mutex::new(settings),
                history: Mutex::new(history),
                count_timer: Mutex::new(Timer::new()),
                state: Mutex::new(MainState { total_time }),
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn cmd_start_timer(label: &str, state: State<AppState>) {
    let mut timer = state.count_timer.lock().unwrap();
    stop_and_save(&mut timer, &state);

    let settings = state.settings.lock().unwrap();
    for t in settings.timer_list() {
        if t.label == label {
            timer.start(t);
            break;
        }
    }
}

#[tauri::command]
fn cmd_stop_timer(state: State<AppState>) {
    let mut timer = state.count_timer.lock().unwrap();
    stop_and_save(&mut timer, &state);
}

#[derive(Serialize, Deserialize)]
pub struct TimerCountRst {
    pub is_time_out: bool,
    pub count_string: String,
}

#[tauri::command]
fn cmd_get_timer_count(state: State<AppState>) -> TimerCountRst {
    let mut timer = state.count_timer.lock().unwrap();
    let (is_time_out, count_string) = timer.update();
    TimerCountRst {
        is_time_out,
        count_string,
    }
}

#[derive(Serialize, Deserialize)]
pub struct TimerStatusRst {
    pub is_running: bool,
    pub label: String,
    pub limit_mins: u64,
    pub total_time: u64,
}

#[tauri::command]
fn cmd_get_timer_status(state: State<AppState>) -> TimerStatusRst {
    let total_time = state.state.lock().unwrap().total_time;
    let timer = state.count_timer.lock().unwrap();
    let timer_setting = timer.get_setting();
    let is_running = timer.status() != timer::Status::Stopped;
    timer_setting.map_or_else(
        || TimerStatusRst {
            is_running,
            label: "".to_string(),
            limit_mins: 0,
            total_time,
        },
        |s| TimerStatusRst {
            is_running,
            label: s.label.clone(),
            limit_mins: s.limit_time,
            total_time,
        },
    )
}

#[derive(Serialize, Deserialize)]
pub struct HistoryInfo {
    pub key: u64,
    pub start_time: String,
    pub duration: String,
    pub duration_secs: u64,
    pub label: String,
}

#[tauri::command]
fn cmd_get_history(
    offset_days: Option<i64>,
    reverse: bool,
    state: State<AppState>,
) -> Vec<HistoryInfo> {
    let history = state.history.lock().unwrap();
    let start = match offset_days {
        Some(days) => crate::get_time_from_offset_days(days),
        None => SystemTime::UNIX_EPOCH,
    };
    let end = SystemTime::now();
    let records = history.get_records(&start, &end, reverse);
    records
        .iter()
        .map(|r| HistoryInfo {
            key: r.key,
            start_time: chrono::DateTime::<Local>::from(r.start_time)
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            duration: crate::timer::secs_to_string(r.duration, ""),
            duration_secs: r.duration,
            label: r.label.clone(),
        })
        .collect()
}

#[tauri::command]
fn cmd_delete_record(key: u64, state: State<AppState>) {
    let mut history = state.history.lock().unwrap();
    history.remove(key);
}

#[tauri::command]
fn cmd_modify_record(key: u64, duration: u64, label: &str, state: State<AppState>) {
    let mut history = state.history.lock().unwrap();
    history.modify_record(key, duration, label);
}

fn stop_and_save(timer: &mut Timer, state: &State<AppState>) {
    if let Some((duration, settings)) = timer.stop() {
        state.state.lock().unwrap().total_time += duration;
        state
            .history
            .lock()
            .unwrap()
            .add_record(timer.get_start_time(), duration, &settings.label);
    }
}

#[tauri::command]
fn cmd_export_to_csv(file_name: &str, state: State<AppState>) {
    let history = state.history.lock().unwrap();
    if let Err(e) = history.export_to_csv(file_name) {
        println!("{}", e);
    }
}

fn init_total_time(history: &History) -> u64 {
    let end = SystemTime::now();
    let start = get_time_from_offset_days(0);
    history
        .get_records(&start, &end, false)
        .iter()
        .map(|r| r.duration)
        .sum()
}

fn get_time_from_offset_days(days: i64) -> SystemTime {
    let date = Local::now().date_naive() + chrono::Duration::days(days);
    let time = date.and_hms_opt(0, 0, 0).unwrap();
    time.and_local_timezone(chrono::Local)
        .single()
        .unwrap()
        .into()
}

#[tauri::command]
fn cmd_get_settings(state: State<AppState>) -> UiSettings {
    let settings = state.settings.lock().unwrap();
    UiSettings {
        theme: settings.theme().to_string(),
        timers: settings.timer_list().into(),
    }
}

#[tauri::command]
fn cmd_save_settings(settings: UiSettings, state: State<AppState>) {
    let mut s = state.settings.lock().unwrap();
    s.set_theme(&settings.theme);
    *s.mut_timer_list() = settings.timers;
    s.save();
}

#[derive(Serialize, Deserialize)]
struct UiSettings {
    theme: String,
    timers: Vec<TimerSetting>,
    // play_audio: bool,
    // audio_file: String,
}
