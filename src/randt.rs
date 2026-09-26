use std::{error::Error, fmt::Display};

use rand::Rng;

use crate::country::{Address, Country, Race, load_countries};

#[derive(Debug)]
pub enum RandomError {
    RandomCountryError,
    RandomRaceError,
    RandomAddressError,
}

impl Display for RandomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl Error for RandomError {}

pub trait Rand: Sized {
    fn rand(args: &[String]) -> Result<Self, Box<dyn Error>>;
}

impl Rand for Country {
    fn rand(args: &[String]) -> Result<Self, Box<dyn Error>> {
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

impl Rand for Race {
    fn rand(args: &[String]) -> Result<Self, Box<dyn Error>> {
        if args.len() == 1 {
            if let Ok(x) = args[0].parse::<u8>() {
                return Ok(match x {
                    0 => Race::African,
                    1 => Race::Arab,
                    2 => Race::Chinese,
                    3 => Race::EastEuropean,
                    4 => Race::European,
                    5 => Race::Hindu,
                    6 => Race::Hispanic,
                    7 => Race::Japanese,
                    _ => return Err(Box::new(RandomError::RandomRaceError)),
                });
            }
        }

        if args.len() == 0 {
            let mut rng = rand::rng();

            let race = rng.random_range(0..8);
            return Race::rand(&[race.to_string()]);
        }

        Err(Box::new(RandomError::RandomRaceError))
    }
}

impl Rand for Address {
    fn rand(args: &[String]) -> Result<Self, Box<dyn Error>> {
        let mut rng = rand::rng();
        let citi = rng.random_range(0..1);

        let country = Country::rand(&[])?;
        let city = country
            .cities
            .keys()
            .nth(citi)
            .ok_or(RandomError::RandomAddressError)?
            .to_string();

        Ok(Self {
            country_code: country.code,
            race: Race::rand(args)?,
            province: country.cities[&city][rng.random_range(0..2)].clone(),
            city,
            zip_code: rng.random_range(10000..99999),
        })
    }
}
