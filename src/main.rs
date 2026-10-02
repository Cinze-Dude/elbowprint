use crate::{creditcard::CreditCard, randt::Rand};
use std::error::Error;

pub mod address;
pub mod country;
pub mod creditcard;
pub mod credittype;
pub mod luhn;
pub mod phonenumber;
pub mod randt;

fn main() -> Result<(), Box<dyn Error>> {
    let cc = match CreditCard::rand(&[]) {
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
