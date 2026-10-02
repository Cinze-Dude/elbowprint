use rand::Rng;

use crate::{country::Country, randt::Rand};

pub struct PhoneNumber {
    pub code: u16,
    pub number: u64,
    pub formatted: String,
}

impl Rand for PhoneNumber {
    fn rand(args: &[String]) -> Result<Self, Box<dyn std::error::Error>> {
        let code = Country::rand(args)?.code;

        let mut rng = rand::rng();
        let number: u64 = rng.random_range(1_000_000_000..10_000_000_000);

        Ok(PhoneNumber {
            code,
            number,
            formatted: format!("+{} {}", code, number),
        })
    }
}
