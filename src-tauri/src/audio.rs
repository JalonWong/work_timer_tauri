use std::{
    fs::File,
    io::{BufReader, Result},
    path::Path,
    thread,
};

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
