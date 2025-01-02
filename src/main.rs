use std::{error::Error,fs::File, io::{self, BufReader}, thread, time::Duration};
use duck_comunicate::{get_public_ip, send_update, send_update_no_ip};
use logger::entry_for_log;
use models::entry::Entry;
use tokio::task::LocalEnterGuard;

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
    //Almaceno una variable con la ultima ip
    let mut last_ip = String::new();
    //Bolean para determinar si debo actualizar la ip o no 
    let mut _update_ip = true;
    let mut _sites_count: usize = 0;
    loop {
        //Obtengo una nueva Ip
        let new_ip = match get_public_ip() {
            Ok(ip) => ip,
            Err(_) => String::new()
        };
        //Comparo si es diferente con la anterior
        if new_ip != last_ip {
            _update_ip = true;
            last_ip = new_ip.clone();
        }else{
            _update_ip = false;
        }
        if _update_ip {
            match read_file() {
                Ok(entries) => {
                    _sites_count = entries.len();
                    let mut log_res = Vec::<String>::with_capacity(_sites_count);
                    for entry in entries {
                        if last_ip.is_empty() {
                            match send_update_no_ip(&entry.domain,&entry.token,entry.txt).await {
                                Ok(respuesta) => {
                                    let status = respuesta.status();
                                    let res_body = respuesta.text().await.unwrap();
                                    if status.is_success() {
                                        println!("Dominio {} actualizado correctamente. Response: {}",entry.domain,&res_body);
                                        log_res.push(format!("Dominio {} actualizado correctamente. Response: {}",entry.domain,&res_body));
                                    }else{
                                        println!("Error al actualizar Dominio {}",entry.domain);
                                        log_res.push(format!("Error al actualizar Dominio {}",entry.domain));
                                    }
                                },
                                Err(_) => {
                                    log_res.push(format!("Error al actualizar Dominio {}",entry.domain));
                                }
                            }
                        }else{
                            match send_update(&entry.domain,&last_ip ,&entry.token,entry.txt).await {
                                Ok(respuesta) => {

                                    let status = respuesta.status();
                                    let res_body = respuesta.text().await.unwrap();
                                    if status.is_success() {
                                        println!("Dominio {} actualizado correctamente. Response: {}",entry.domain,&res_body);
                                        log_res.push(format!("Dominio {} actualizado correctamente. Response: {}",entry.domain,&res_body));
                                    }else{
                                        println!("Error al actualizar Dominio {}",entry.domain);
                                        log_res.push(format!("Error al actualizar Dominio {}",entry.domain));
                                    }
                                },
                                Err(_) => {
                                    log_res.push(format!("Error al actualizar Dominio {}",entry.domain));
                                }
                            }
                            
                        }
                    }
                    let texto = log_res.join("\n");
                    _= logger::overwrite_file("log.txt", &texto);
                }
                Err(_e) => {
                    // Loguear el error si ocurre
                    _ = entry_for_log("Error reading file");
                }
            }
            _ = logger::purge_log();
        }
        // Pausa de 30 segundos
        thread::sleep(Duration::from_secs(30));
    }
}
