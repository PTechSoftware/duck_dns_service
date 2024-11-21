use std::{error::Error, fs::File, io::{self, BufReader}, thread, time::Duration};
use duck_comunicate::send_update;
use logger::entry_for_log;
use models::entry::Entry;

mod models;
mod duck_comunicate;
mod logger;

// Función para leer el archivo `config.json` y deserializarlo
fn read_file() -> Result<Vec<Entry>, Box<dyn Error>> {
    // Intentar abrir el archivo
    let ref_file = File::open("config.json")?;
    let reader = BufReader::new(ref_file);

    // Deserializar el JSON en un vector de `Entry`
    let entradas: Vec<Entry> = serde_json::from_reader(reader)?;

    // Retornar las entradas
    Ok(entradas)
}


#[tokio::main]
async fn main() -> io::Result<()>{
    loop {
        match read_file() {
            Ok(entries) => {
                for entry in entries {
                    _ =send_update(&entry.domain, &entry.token,entry.txt).await
                }
            }
            Err(_e) => {
                // Loguear el error si ocurre
                _ = entry_for_log("Error reading file");
            }
        }
        _ = logger::purge_log();
        // Pausa de 30 segundos
        thread::sleep(Duration::from_secs(30));
    }
}
