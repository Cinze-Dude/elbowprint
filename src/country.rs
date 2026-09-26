use std::collections::HashMap;

use serde_json::Value;

pub fn load_countries() -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("countries.json")?;

    let data = serde_json::from_str(&json)?;

    Ok(data)
}

pub fn load_names() -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("names.json")?;

    let data = serde_json::from_str(&json)?;

    Ok(data)
}

#[derive(Debug, serde::Deserialize)]
pub struct Country {
    #[serde(rename = "Code")]
    pub code: u16,

    #[serde(rename = "Cities")]
    pub cities: HashMap<String, [String; 2]>,
}

#[derive(Debug)]
pub enum Race {
    Hispanic,
    European,
    EastEuropean,
    Chinese,
    Japanese,
    African,
    Arab,
    Hindu,
}

#[derive(Debug)]
pub struct Address {
    pub race: Race,
    pub country_code: u16,
    pub province: String,
    pub city: String,
    pub zip_code: u32,
}
