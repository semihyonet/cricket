use std::path::Path;
use std::process::Command;

fn strip_mid_extension(filename: String) -> String {
    if let Some(stripped) = filename.strip_suffix(".mid") {
        stripped.to_string()
    } else {
        filename
    }
}

pub fn render_midi_to_wav(
    midi_path: &str,
    soundfont_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = strip_mid_extension(String::from(midi_path));
    let wav_path = format!("{}.wav", file_name);
    print!("{:?} {:?} {:?}", midi_path, wav_path, soundfont_path);
    if !Path::new(midi_path).exists() {
        return Err(format!("MIDI file not found: {}", midi_path).into());
    }
    if !Path::new(soundfont_path).exists() {
        return Err(format!("SoundFont file not found: {}", soundfont_path).into());
    }

    let status = Command::new("fluidsynth")
        .arg("-ni") // no interactive mode
        .arg(soundfont_path)
        .arg(midi_path)
        .arg("-F") // output to file
        .arg(wav_path)
        .arg("-r")
        .arg("44100") // sample rate
        .status()?;

    if !status.success() {
        return Err("Failed to render MIDI to WAV using FluidSynth".into());
    }

    Ok(())
}
