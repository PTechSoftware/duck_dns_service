use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug,Serialize,Deserialize)]
pub struct Entry {
    pub domain :String,
    pub token :String,
    pub txt :Option<String>
}