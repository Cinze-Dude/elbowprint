use rand::{Rng, rngs::ThreadRng};

use crate::{country::Country, randt::Rand};

#[derive(Debug)]
pub struct PhoneNumber {
    pub code: u16,
    pub number: u64,
    pub formatted: String,
}

impl Rand for PhoneNumber {
    fn rand(rng: &mut ThreadRng, args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let code = Country::rand(rng, args)?.code;

        let number: u64 = rng.random_range(1_000_000_000..10_000_000_000);

        Ok(PhoneNumber {
            code,
            number,
            formatted: format!("+{} {}", code, number),
        })
    }
}
