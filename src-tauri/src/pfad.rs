use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tokio::sync::oneshot;

use serde;

use tauri::{AppHandle, Builder, Emitter, Listener, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

const PFAD_ENDUNGEN: [&'static str; 4] = ["tex", "txt", "md", "cvs"];

#[derive(Serialize, Default, Clone)]
// #[derive(serde::Serialize)]
pub struct AppIdentitaet {
    pub dateipfad: Option<String>,
    pub name: Option<String>,
    pub endung: Option<String>,
}

fn dateipfad_auseinandernehmen(pfad: String) -> Option<(String, String)> {
    // vielleicht die checks sollten hier sein!
    let path: PathBuf = PathBuf::from(pfad);
    if let Some(file_name) = path.file_stem().and_then(|name| name.to_str()) {
        if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
            return Some((file_name.to_string(), extension.to_string()));
        }
    }
    return None;
}
fn identitaet_fuellen(app: AppHandle, pfad: String) {
    // name ändern "identitaet_fuellen"
    if let Some((file_name, extension)) = dateipfad_auseinandernehmen(pfad.clone()) {
        let charakter = app.state::<Mutex<AppIdentitaet>>();
        let mut charakter = charakter.lock().unwrap();
        charakter.dateipfad = Some(pfad);
        charakter.name = Some(file_name.to_string());
        charakter.endung = Some(extension.to_string());
    } else {
        println!("Could not extract file name and extension");
    }
}
pub fn identitaet_ausgeben(app: AppHandle) -> AppIdentitaet {
    let charakter = app.state::<Mutex<AppIdentitaet>>();
    let charakter = charakter.lock().unwrap();
    charakter.clone()
}

async fn datei_aktivieren_oder_herstellen(app: AppHandle, path: String) -> Result<String, String> {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(contents) => Ok(contents),
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
pub async fn dateipfad_eingegeben(app: AppHandle, pfad: String) -> Result<AppIdentitaet, String> {
	// TODO: noch weitere Checks hinzufügen, falls die "pfad" manipuliert worden sein sollte!
	// 	ähnlich wie "neue_dateipfad_pruefen" aber die erste Check soll anderes rum sein
	// 		Messane = "this app cannot access the path provided!"
	// TODO: name ändern "dateipfad_einsetzen"

	let res = datei_aktivieren_oder_herstellen(app.clone(), pfad.clone()).await;
	match res {
		Ok(_) => {
			identitaet_fuellen(app.clone(), pfad.clone());
			let identitaet = identitaet_ausgeben(app.clone());
			app.emit("datei-gewaehlt", identitaet.clone()).unwrap();
			Ok(identitaet)
		}
		Err(e) => {
			return Err(format!("Failed to create file: {:?}", e))
		}
	}
}

#[tauri::command]
pub async fn datei_waehlen(app: AppHandle) -> Result<AppIdentitaet, String> {
    // Create a channel to communicate between the callback and the async function
    let (tx, rx) = oneshot::channel();

    app.clone()
        .dialog()
        .file()
        .add_filter("Text", &PFAD_ENDUNGEN)
        .pick_file(move |file_path| {
            if let Some(file_path) = file_path {
                println!("gewählte Datei: {}", file_path.to_string());
                tx.send(file_path.to_string()).unwrap();
            } else {
                println!("keine Datei gewählt.");
                tx.send(String::new()).unwrap();
            }
        });

    // Wait for the file path from the callback
    let file_path = rx.await.unwrap();

    if !file_path.is_empty() {
        let res = datei_aktivieren_oder_herstellen(app.clone(), file_path.clone()).await;
        match res {
            Ok(_) => {
                identitaet_fuellen(app.clone(), file_path.clone());
                let identitaet = identitaet_ausgeben(app.clone());
                app.emit("datei-gewaehlt", identitaet.clone()).unwrap();
                Ok(identitaet)
            }
            Err(e) => return Err(format!("Failed to create file: {:?}", e)),
        }
    } else {
        Err("No file selected".to_string())
    }
}
