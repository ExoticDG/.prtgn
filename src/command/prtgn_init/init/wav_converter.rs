use audio_engine::{AudioEngine, WavDecoder};
use std::io;
use std::fs::File;


pub fn wav_to_prtgn(filename_prt: String) -> io::Result<()> {
    let audio_engine = AudioEngine::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let file = File::open(&filename_prt)?;
    let decoder = WavDecoder::new(file).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let mut sound = audio_engine.new_sound(decoder).map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    sound.play();
    Ok(())
}