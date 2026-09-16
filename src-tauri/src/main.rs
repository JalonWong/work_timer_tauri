// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // For tittle bar issue
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
        std::env::set_var("WAYLAND_DISPLAY", "");
    }

    work_timer_tauri_lib::run()
}
