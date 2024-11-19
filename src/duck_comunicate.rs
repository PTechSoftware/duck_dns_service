use reqwest::Client;
use std::error::Error;

use crate::logger::entry_for_log;


#[allow(dead_code)]
pub async fn send_update(domain: &str, token: &str,txt :Option<String>) -> Result<(), Box<dyn Error>> {
    // Initialize the HTTP client
    let client = Client::new();

    let url = match txt {
        Some(reg) => {
            format!("https://www.duckdns.org/update?domains={}&token={}&txt={}&ip=",domain,token,reg)
        },
        None =>{
            format!("https://www.duckdns.org/update?domains={}&token={}&ip=",domain,token)
        }
    };

    println!("{}",&url);
    // Send the POST request
    let res = client
        .get(&url)
        .send()
        .await?;

    // Handle the response
    if res.status().is_success() {
        let _ = entry_for_log(&format!(r#"**************************************************
        Dominio {} actualizado con éxito. 
        {:?}
        **************************************************"#,domain,res));
        Ok(())
    } else {
        let _ = entry_for_log(&format!(r#"**************************************************
Error al actualizar Dominio {}. 
Detalles:
{:?}
**************************************************"#,domain,res));
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed with status: {}", res.status()),
        )))
    }
}

#[cfg(test)]
mod test {
    use super::send_update;
    #[tokio::test]
    async fn test_update(){
        send_update("ptechcloud", "b2f08b23-ebf1-4d4b-8132-327b9def1bf1" ,Some("google-site-verification=8ITNQvEh75v-MRsYr-sq3409kPQxeeus80vv8isjkms".to_string()) ).await.unwrap();

    }

}