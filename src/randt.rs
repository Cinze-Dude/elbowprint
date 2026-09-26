use std::{error::Error, fmt::Display};

use rand::Rng;

use crate::country::{Country, load_countries};

#[derive(Debug)]
pub enum RandomError {
    RandomCountryError,
}

impl Display for RandomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for RandomError {}

pub trait Rand: Sized {
    fn rand<U>(args: &[U]) -> Result<Self, Box<dyn Error>>;
}

impl Rand for Country {
    fn rand<U>(args: &[U]) -> Result<Self, Box<dyn Error>> {
        let countries = load_countries()?;

        let mut rng = rand::rng();

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
