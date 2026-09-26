use crate::credittypes::CardType;
use std::error::Error;

pub mod address;
pub mod country;
pub mod credittypes;
pub mod randt;

fn main() -> Result<(), Box<dyn Error>> {
    let cards = [
        ("Visa", "4111111111111111"),
        ("MasterCard 51", "5111111111111111"),
        ("MasterCard 55", "5511111111111111"),
        ("MasterCard 2229", "2229111111111111"),
        ("MasterCard 2720", "2720111111111111"),
        ("UnionPay", "6212345678901234"),
        ("Maestro", "5018123412345678"),
        ("Discover", "6011123412345678"),
        ("JCB", "3528123412345678"),
        ("RuPay", "6085123412345678"),
        ("Elo", "4011781234567890"),
        ("Unknown", "1234567890123456"),
    ];

    for (name, pan) in cards {
        match CardType::new(pan.to_string()) {
            Ok(Some(card_type)) => {
                println!("{name}: {pan} -> {card_type:?}");
            }
            Ok(None) => {
                println!("{name}: {pan} -> Unknown");
            }
            Err(error) => {
                println!("{name}: {pan} -> Error: {error}");
            }
        }
    }

    println!("\n--- Invalid PAN tests ---");

    let invalid_cards = [
        ("Too short", "411111111111111"),
        ("Too long", "41111111111111111"),
        ("Non-numeric", "411111111111111a"),
        ("Spaces", "4111 1111 1111 1111"),
        ("Empty", ""),
    ];

    for (name, pan) in invalid_cards {
        match CardType::new(pan.to_string()) {
            Ok(Some(card_type)) => {
                println!("{name}: {pan:?} -> {card_type:?}");
            }
            Ok(None) => {
                println!("{name}: {pan:?} -> Unknown");
            }
            Err(error) => {
                println!("{name}: {pan:?} -> Error: {error}");
            }
        }
    }

    Ok(())
}
