use std::{error::Error, fmt::Display};

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
