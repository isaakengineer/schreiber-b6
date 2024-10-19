use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tokio::sync::oneshot;

extern crate dirs;

use serde;

use tauri::{AppHandle, Builder, Emitter, Listener, Manager};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

const PFAD_ENDUNGEN: [&'static str; 4] = ["tex", "txt", "md", "cvs"];

#[derive(Serialize, Default, Clone, Debug)]
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

fn datei_aktivieren_oder_herstellen(app: AppHandle, path: String) -> Result<String, String> {
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
pub fn neue_datei_erstellen(app: AppHandle, name: &str, inhalt: &str) -> Result<bool, String> {
	println!("datei mit Name {:?} und Ihhalt {:?} erstellen", name.clone(), inhalt.clone());
	let dokumente_pfad = dirs::document_dir().unwrap();
	let mut datei_pfad = PathBuf::from(&dokumente_pfad);
	datei_pfad.push(name);
	println!("{:#?}", datei_pfad.clone());
	if datei_pfad.exists() {
		let m = format!("Eine datei mit dieser Name bereit exitiert");
		println!("{:?}", m);
		return Err(m);
	} else {
		let msg = format!("Fehlermeldung beim Erstellung des Datei.");
		let mut datei = File::create(datei_pfad.clone()).expect(&msg);
		let res = datei.write_all(inhalt.as_bytes());
		match res {
			Ok(_) => {
				identitaet_fuellen(app.clone(), datei_pfad.to_str().unwrap().to_string());
				let identitaet = identitaet_ausgeben(app.clone());
				app.emit("datei-gewaehlt", identitaet.clone()).unwrap();
				Ok(true)
			},
			Err(e) => {
				let msg = format!("Könnte nicht auf den Datei zuschreiben. Siehe: {:?}", e);
				return Err(msg)
			}
		}
	}
}

#[tauri::command]
pub fn dateipfad_eingegeben(app: AppHandle, pfad: &str) -> Result<AppIdentitaet, String> {
	// TODO: noch weitere Checks hinzufügen, falls die "pfad" manipuliert worden sein sollte!
	// 	ähnlich wie "neue_dateipfad_pruefen" aber die erste Check soll anderes rum sein
	// 		Messane = "this app cannot access the path provided!"
	// TODO: name ändern "dateipfad_einsetzen"
	println!("dateipfad ({:?}) eingegeben und wird bearbeitet", pfad);
	let res = datei_aktivieren_oder_herstellen(app.clone(), pfad.to_string().clone());
	match res {
		Ok(_) => {
			identitaet_fuellen(app.clone(), pfad.to_string().clone());
			let identitaet = identitaet_ausgeben(app.clone());
			app.emit("datei-gewaehlt", identitaet.clone()).unwrap();
			println!("{:?}", identitaet.clone());
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
        let res = datei_aktivieren_oder_herstellen(app.clone(), file_path.clone());
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
