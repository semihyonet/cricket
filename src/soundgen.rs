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
) -> Result<String, Box<dyn std::error::Error>> {
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
        .arg(&wav_path)
        .arg("-r")
        .arg("44100") // sample rate
        .status()?;

    if !status.success() {
        return Err("Failed to render MIDI to WAV using FluidSynth".into());
    }

    Ok(wav_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    /// Helper to generate a unique temp path
    fn temp_path(file_name: &str) -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("cricket_test_{}", file_name));
        p
    }

    #[test]
    fn strip_mid_extension_strips() {
        assert_eq!(strip_mid_extension("song.mid".into()), "song".to_string());
        assert_eq!(
            strip_mid_extension("long.name.with.dots.mid".into()),
            "long.name.with.dots".to_string()
        );
    }

    #[test]
    fn strip_mid_extension_leaves_otherwise() {
        assert_eq!(
            strip_mid_extension("song.MID".into()),
            "song.MID".to_string()
        );
        assert_eq!(
            strip_mid_extension("track.txt".into()),
            "track.txt".to_string()
        );
        assert_eq!(
            strip_mid_extension("noextension".into()),
            "noextension".to_string()
        );
    }

    #[test]
    fn render_midi_to_wav_errors_if_midi_missing() {
        let midi = temp_path("does_not_exist.mid");
        let sf2 = temp_path("dummy.sf2");
        // ensure both are gone
        let _ = std::fs::remove_file(&midi);
        let _ = std::fs::remove_file(&sf2);

        let err = render_midi_to_wav(midi.to_str().unwrap(), sf2.to_str().unwrap())
            .unwrap_err()
            .to_string();

        assert!(err.contains("MIDI file not found"));
        assert!(err.contains(midi.to_str().unwrap()));
    }

    #[test]
    fn render_midi_to_wav_errors_if_soundfont_missing() {
        // create a dummy midi file
        let midi = temp_path("dummy.mid");
        let mut f = File::create(&midi).expect("couldn't create temp midi");
        write!(f, "fake midi").unwrap();

        let sf2 = temp_path("also_missing.sf2");
        let _ = std::fs::remove_file(&sf2);

        let err = render_midi_to_wav(midi.to_str().unwrap(), sf2.to_str().unwrap())
            .unwrap_err()
            .to_string();

        assert!(err.contains("SoundFont file not found"));
        assert!(err.contains(sf2.to_str().unwrap()));

        // clean up
        let _ = std::fs::remove_file(&midi);
    }
}
