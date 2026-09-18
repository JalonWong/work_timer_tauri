use std::{
    fs::File,
    io::{BufReader, Result},
    path::Path,
    thread,
};
use tauri::{AppHandle, Manager};
use tauri_plugin_notification::NotificationExt;

pub fn play_a_sound(file_name: &Path) -> Result<()> {
    let f = File::open(file_name)?;
    thread::spawn(move || {
        let mut sink_handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        sink_handle.log_on_drop(false);
        let file = BufReader::new(f);
        // Note that the playback stops when the player is dropped
        let player = rodio::play(&sink_handle.mixer(), file).unwrap();
        player.sleep_until_end();
    });
    Ok(())
}

pub fn notify(app: &AppHandle) {
    let _ = app
        .notification()
        .builder()
        .title("Timer done")
        .auto_cancel()
        .show();
}

pub fn focus_on_window(app: &AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        let _ = win.unminimize();
        let _ = win.set_focus();
    }
}
