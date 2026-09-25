use std::collections::HashMap;

use serde_json::Value;

#[derive(Debug)]
pub struct Country {
    code: u8,
    cities: Vec<String>,
}

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
