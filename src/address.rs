use std::error::Error;

use rand::{Rng, rngs::ThreadRng};

use crate::{
    country::Country,
    randt::{Rand, RandomError},
};

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

impl Rand for Race {
    fn rand(rng: &mut ThreadRng, args: &[String]) -> Result<Self, Box<dyn Error>> {
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
            let race = rng.random_range(0..8);
            return Race::rand(rng, &[race.to_string()]);
        }

        Err(Box::new(RandomError::RandomRaceError))
    }
}

#[derive(Debug)]
pub struct Address {
    pub race: Race,
    pub country_code: u16,
    pub province: String,
    pub city: String,
    pub zip_code: u32,
}

impl Rand for Address {
    fn rand(rng: &mut ThreadRng, args: &[String]) -> Result<Self, Box<dyn Error>> {
        let citi = rng.random_range(0..1);

        let country = Country::rand(rng, &[])?;
        let city = country
            .cities
            .keys()
            .nth(citi)
            .ok_or(RandomError::RandomAddressError)?
            .to_string();

        Ok(Self {
            country_code: country.code,
            race: Race::rand(rng, args)?,
            province: country.cities[&city][rng.random_range(0..2)].clone(),
            city,
            zip_code: rng.random_range(10000..99999),
        })
    }
}
