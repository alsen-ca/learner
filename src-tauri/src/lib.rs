use std::{sync::Mutex,fs::File,io::BufReader};
use rodio::{OutputStream, OutputStreamBuilder, Sink, Decoder};


struct AudioState(Mutex<(OutputStream, Sink)>);

#[tauri::command]
fn play_sound(letter: String, state: tauri::State<AudioState>) -> Result<(), String> {
    println!("Sound clicked to be played");
    let path = format!("assets/audios/de/{}.ogg", letter.to_uppercase());

    let guard = state.0.lock().map_err(|e| e.to_string())?;
    let (_, sink) = &*guard;

    let file = BufReader::new(File::open(&path).map_err(|e: std::io::Error| e.to_string())?);
    let source = Decoder::new(file).map_err(|e: rodio::decoder::DecoderError| e.to_string())?;

    sink.append(source);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let stream_handle = OutputStreamBuilder::open_default_stream()
        .expect("failed to open audio stream");
    let sink = Sink::connect_new(&stream_handle.mixer());
    
    tauri::Builder::default()
        .manage(AudioState(Mutex::new((stream_handle, sink))))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![play_sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
