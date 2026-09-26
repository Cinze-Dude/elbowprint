use std::{collections::HashMap, error::Error};

use rand::Rng;
use serde_json::Value;

use crate::randt::{Rand, RandomError};

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

impl Rand for Country {
    fn rand(_args: &[String]) -> Result<Self, Box<dyn Error>> {
        let countries = load_countries()?;

        let mut rng: rand::prelude::ThreadRng = rand::rng();

        let race = rng.random_range(0..countries.len());

        let group = countries
            .values()
            .nth(race)
            .ok_or(RandomError::RandomCountryError)?;

        let group = group.as_object().ok_or(RandomError::RandomCountryError)?;

        let index = rng.random_range(0..group.len());

        let country = group
            .values()
            .nth(index)
            .ok_or(RandomError::RandomCountryError)?;

        Ok(serde_json::from_value(country.clone())?)
    }
}
