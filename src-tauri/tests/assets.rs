#[test]
fn all_letter_audio_files_exist() {
    for c in 'A'..='Z' {
        let path = format!("{}/assets/audios/de/{}.ogg", env!("CARGO_MANIFEST_DIR"), c);
        assert!(std::path::Path::new(&path).exists(), "Missing: {}", path);
    }
}
