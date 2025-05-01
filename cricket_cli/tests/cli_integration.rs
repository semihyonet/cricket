use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

/// Helper: write a minimal cricket_clir file
fn write_example(path: &PathBuf, content: &str) {
    let mut f = File::create(path).expect("failed to create example file");
    write!(f, "{}", content).expect("failed to write example");
}

#[test]
fn cli_help_and_version() {
    Command::cargo_bin("cricket_cli")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));

    Command::cargo_bin("cricket_cli")
        .unwrap()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap());
}

#[test]
fn generates_midi_file() {
    let tmp = tempfile::tempdir().unwrap();
    let cricket_file = tmp.path().join("simple.crkt");

    write_example(
        &cricket_file,
        "Instrument instrument_xyz:\n\ttype: Strings\n\tmidi_path: xyz\n\nPattern intro(): \n\treturn [1:8] Note(Am)\n\nSection Intro:\n\tChannel name_a:\n\t\treturn intro()\n\nSong HotlineBling: \n\n\treturn Intro() + Intro()",
    );

    let mut cmd = Command::cargo_bin("cricket_cli").unwrap();
    cmd.arg(cricket_file.to_str().unwrap())
        .arg("-g")
        .arg("midi");
    cmd.assert().success();
    println!("{:?}", tmp.path());
    let midi_path = PathBuf::from("HotlineBling.mid");
    assert!(midi_path.exists(), "MIDI file was not created");
    fs::remove_file(midi_path).unwrap();
}

#[test]
fn generates_wav_file() {
    let tmp = tempfile::tempdir().unwrap();
    let cricket_file = tmp.path().join("soundc.crkt");

    write_example(
        &cricket_file,
        "Instrument instrument_xyz:\n\ttype: Strings\n\tmidi_path: xyz\n\nPattern intro(): \n\treturn [1:8] Note(Am)\n\nSection Intro:\n\tChannel name_a:\n\t\treturn intro()\n\nSong HotlineBling: \n\n\treturn Intro() + Intro()",
    );

    let mut cmd = Command::cargo_bin("cricket_cli").unwrap();
    cmd.arg(cricket_file.to_str().unwrap())
        .arg("-g")
        .arg("sound");
    let err_text = "No SoundFont Path has been passed while trying to generate a Sound. Please use the --sf-path argument to pass a path to the soundfont.\n";
    cmd.assert().stderr(err_text);

    cmd = Command::cargo_bin("cricket_cli").unwrap();
    cmd.arg(cricket_file.to_str().unwrap());
    cmd.assert().stderr(err_text);
    //    note: SoundGen takes a bit of time and we only use a sepearate module to test. for now lets
    //    only check if the cli sound variable passes us into this sound gen flow.
    //    let wav_path = PathBuf::from("HotlineBling.mid");
    //    assert!(wav_path.exists(), "WAV file was not created");
    //    fs::remove_file(wav_path).unwrap();
}
