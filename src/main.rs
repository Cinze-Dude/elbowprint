use crate::{creditcard::CreditCard, phonenumber::PhoneNumber, randt::Rand};
use std::error::Error;

pub mod address;
pub mod country;
pub mod creditcard;
pub mod credittype;
pub mod luhn;
pub mod phonenumber;
pub mod randt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();

    let cc = match PhoneNumber::rand(&mut rng, &[]) {
        Ok(x) => {
            println!("{:#?}", x);
            Some(x)
        }
        Err(x) => {
            eprintln!("{}", x);
            None
        }
    };

    Ok(())
}
