use std::error::Error;

use rand::Rng;

use chrono::{Datelike, Local};

use crate::{credittype::CardType, luhn::Luhn, randt::Rand};

fn weighted_choice<K: Clone>(choices: &[(K, usize)]) -> K {
    let mut rng = rand::rng();
    let mut n = rng.random_range(0..choices.iter().map(|(_, w)| w).sum());

    for (k, w) in choices {
        if n < *w {
            return k.clone();
        }
        n -= w;
    }

    unreachable!()
}

#[derive(Debug)]
pub struct CreditCard {
    pub cardtype: CardType,
    pub pan: String,
    pub cvc: u16,
    pub expire: u16,
}

impl CreditCard {
    pub fn new(pan: String, cvc: u16, expire: u16) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            cardtype: CardType::new(pan.as_str())?
                .ok_or("PAN doesn't match any of the standard credit networks")?,
            pan,
            cvc,
            expire,
        })
    }
}

impl Rand for CreditCard {
    fn rand(args: &[String]) -> Result<Self, Box<dyn Error>> {
        let mut rng = rand::rng();

        // Generate Card Type
        let mut types = vec![
            (CardType::Visa, 400),
            (CardType::UnionPay, 200),
            (CardType::MasterCard, 210),
            (CardType::AmericanExpress, 90),
            (CardType::JapaneseBank, 20),
            (CardType::Discover, 10),
            (CardType::RuPay, 5),
            (CardType::Maestro, 3),
            (CardType::Elo, 2),
        ];

        if args.len() != 0 {
            let requested: Vec<CardType> = args
                .iter()
                .filter_map(|arg| CardType::try_from(arg.as_str()).ok())
                .collect();

            types.retain(|(card_type, _)| requested.contains(card_type));
        }

        let cardtype = weighted_choice(&types);

        // Generate Starting PAN
        let iin: String = match cardtype {
            CardType::Visa => "4".to_string(),

            CardType::UnionPay => "62".to_string(),

            CardType::JapaneseBank => rng.random_range(3528..3590).to_string(),

            CardType::AmericanExpress => match rng.random_range(0..2) {
                0 => "34",
                1 => "37",
                _ => unreachable!(),
            }
            .to_string(),

            CardType::MasterCard => match rng.random_range(0..2) {
                0 => rng.random_range(51..56),
                1 => rng.random_range(2221..2721),
                _ => unreachable!(),
            }
            .to_string(),

            CardType::Discover => match rng.random_range(0..4) {
                0 => 6011,
                1 => 65,
                2 => rng.random_range(644..650),
                3 => rng.random_range(622126..622926),
                _ => unreachable!(),
            }
            .to_string(),

            CardType::Maestro => [
                "5018", "5020", "5038", "5893", "6304", "6759", "6761", "6762", "6763",
            ][rng.random_range(0..9)]
            .to_string(),

            CardType::Elo => [
                "401178", "401179", "431274", "431275", "438935", "438936", "451416", "451417",
                "457631", "457632", "504175", "504176", "506699", "506698", "509000", "509001",
                "627780", "627781", "636297", "636296", "636368", "636369",
            ][rng.random_range(0..22)]
            .to_string(),

            CardType::RuPay => [
                "604", "605", "606", "607", "608", "609", "650", "651", "652", "653", "654", "655",
                "656", "657", "658", "659", "810", "811", "812", "813", "814", "815", "816", "817",
                "818", "819",
            ][rng.random_range(0..26)]
            .to_string(),
        };

        let rest_pan_l = 15u32 - u32::try_from(iin.len())?;
        let mut pan = format!(
            "{}{}",
            iin,
            rng.random_range(10u64.pow(rest_pan_l - 1)..10u64.pow(rest_pan_l))
        );

        pan = Luhn::luhn_pan(&mut pan);

        let cur_year: u16 = (Local::now().year() % 100).try_into()?;

        Ok(Self {
            cardtype,
            pan,
            cvc: rng.random_range(100..999),
            expire: rng.random_range(cur_year..cur_year + 5),
        })
    }
}
