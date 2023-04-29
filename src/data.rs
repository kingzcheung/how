use serde::Deserialize;



#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub n: String,
    pub p: String,
    pub d: String,
}