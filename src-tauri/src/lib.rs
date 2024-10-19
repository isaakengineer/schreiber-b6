use std::{collections::HashMap, fs};
use fs::File;
use tauri_plugin_cli::CliExt;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Seek;

use std::sync::Mutex;
use tauri::{AppHandle, Builder, Emitter, Listener, Manager};

use csv::StringRecord;

mod pfad;

use pfad::{
	AppIdentitaet,
	identitaet_ausgeben,
	datei_waehlen,
	dateipfad_eingegeben,
	neue_datei_erstellen,
};

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

type Record = HashMap<String, String>;

#[tauri::command]
fn csv_lesen_kopf(app: AppHandle) -> Result<Vec<String>, String> {
	let character = identitaet_ausgeben(app.clone());
	let file = File::open(character.dateipfad.unwrap());
	let mut result:Vec<String>;
	match file {
		Ok(file) => {
			let mut rdr = csv::Reader::from_reader(file);
			let headers = rdr.headers();
			match headers {
				Ok(headers) => {
					let res = headers.deserialize(None);
					match res {
						Ok(res) => {
							result = res;
						}
						Err(e) => {
							let m = format!("Datei hat Fehlern! Siehe {:?}", e);
	   						return Err(m);
						}
					}
			 	},
			  	Err(e) => {
			   		let m = format!("Datei hat Fehlern! Siehe {:?}", e);
				 	return Err(m);
			   }
			}
		},
		Err(e) => {
			let m = format!("Datei könnte nicht gelesen werden! Siehe {:?}", e);
   			return Err(m);
		}
	}
	Ok(result)
}

#[tauri::command]
fn csv_lesen_reihen(app: AppHandle) -> Result<Vec<Record>, String> {
	let character = identitaet_ausgeben(app.clone());
	let file = File::open(character.dateipfad.unwrap());
	match file {
		Ok(file) => {
			let mut vec = Vec::new();
			let mut rdr = csv::Reader::from_reader(file);
			for result in rdr.deserialize() {
				match result {
					Ok(record) => {
						println!("{:?}", record);
						vec.push(record);
				 	},
				  	Err(e) => {
				   		let m = format!("Datei hat Fehlern! Siehe {:?}", e);
					 	return Err(m);
				   }
				}
			}
			Ok(vec)
		},
		Err(e) => {
			let m = format!("Datei könnte nicht gelesen werden! Siehe {:?}", e);
   			return Err(m);
		}
	}
}
#[tauri::command]
fn csv_schreiben(app: AppHandle, reihe: Vec<String>) ->Result<bool, String> {
	let character = identitaet_ausgeben(app.clone());
	let mut file = OpenOptions::new()
		.append(true)
		.open(character.dateipfad.unwrap());
	match file {
		Ok(file) => {
			// let needs_headers = file.seek(std::io::SeekFrom::End(0))? == 0;
			let mut wtr = csv::WriterBuilder::new()
				// .has_headers(needs_headers)
		  		.from_writer(file);

			wtr.write_record(reihe);

	  		wtr.flush().unwrap();
			()
		},
		Err(e) => {
			let m = format!("error");
			()
		}
	}
	Ok(true)
}
#[tauri::command]
fn init_pruefen(app: AppHandle) -> Result<AppIdentitaet, String> {
	let charakter = identitaet_ausgeben(app.clone());
	println!("init pruefen {:?}", charakter.clone());
	if charakter.dateipfad.is_some() {
		Ok(charakter.clone())
	} else {
		Err("nicht vorhanden".to_string())
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
			init_pruefen,
			lesen, schreiben,

			csv_lesen_kopf,
			csv_lesen_reihen,
			csv_schreiben,

			datei_waehlen,
			dateipfad_eingegeben,
			neue_datei_erstellen,
		])
		.setup(|app| {
			app.manage(Mutex::new(AppIdentitaet::default()));
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
					let mut message;
					let arg_data = matches.args.get("dateipfad").unwrap().value.clone();
					println!("args = {:?}", arg_data);
					if arg_data.is_boolean() && arg_data.as_bool().unwrap() == false {
						message = format!("no file supplied");
						println!("{:?}", message);
					} else {
						let eingabeString = arg_data.to_string();
						let init = dateipfad_eingegeben(app.handle().clone(), &eingabeString[1..eingabeString.len() - 1]);
					}
			   }
			   Err(_) => {}
		   	}
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
