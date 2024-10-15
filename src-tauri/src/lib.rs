use std::fs;

use tauri_plugin_cli::CliExt;

use std::sync::Mutex;
use tauri::{AppHandle, Builder, Emitter, Listener, Manager};

mod pfad;

use pfad::{
    datei_waehlen, 
    identitaet_ausgeben, 
    dateipfad_eingegeben,
    AppIdentitaet};

const FILE_PATH:&'static str = "/media/isaak/bb7d74a4-10c5-462f-8359-3368c8f3346f/2024_K/2.projekt/ntwrfn/schreiber/demo/sample.text";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn lesen(app: AppHandle) -> Result<String, String> {
    let character = identitaet_ausgeben(app.clone());
    let contents = fs::read_to_string(character.dateipfad.unwrap())
        .expect("Should have been able to read the file");
    // println!("{:?}", contents);
    Ok(contents)
}

#[tauri::command]
fn schreiben(app: AppHandle, text: &str) -> Result<bool, String> {
    let character = identitaet_ausgeben(app.clone());
    let res = fs::write(character.dateipfad.unwrap(), text);
	match res {
	    Ok(_) => Ok(true),
	    Err(e) => {
	        let m = format!("Könnte nicht auf den Datei zuschreiben. Siehe {:?}", e);
	        Err(m)
	    }
	}
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            lesen, schreiben, datei_waehlen,
            dateipfad_eingegeben,
        ])
        .setup(|app| {
            // #[cfg(debug_assertions)] // only include this code on debug builds
            // {
            //   let window = app.get_webview_window("main").unwrap();
            //   window.open_devtools();
            //   window.close_devtools();
            // }
            match app.cli().matches() {
               // `matches` here is a Struct with { args, subcommand }.
               // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurrences }.
               // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
               Ok(matches) => {
                   println!("cli arguments: {:?}", matches)
               }
               Err(_) => {}
           }
            app.manage(Mutex::new(AppIdentitaet::default()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
