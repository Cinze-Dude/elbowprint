use rand::seq::IndexedRandom;
use regex::Regex;
use std::error::Error;

use crate::randt::Rand;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardType {
    Visa,
    MasterCard,
    AmericanExpress,
    UnionPay,
    Maestro,
    Discover,
    JapaneseBank,
    RuPay,
    Elo,
}

impl CardType {
    pub fn new(pan: &str) -> Result<Option<Self>, Box<dyn Error>> {
        if pan.len() != 16 || !pan.chars().all(|c| c.is_ascii_digit()) {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "PAN must contain exactly 16 digits and be numeric",
            )))?;
        }

        let card_regex: Vec<(Regex, CardType)> = vec![
            (Regex::new("^4").unwrap(), CardType::Visa),
            (Regex::new("^(5[1-5]|2(2[2-9][1-9]|[3-6][0-9]{2}|7[01][0-9]|720))").unwrap(), CardType::MasterCard),
            (Regex::new("^(34|37)").unwrap(), CardType::AmericanExpress),
            (Regex::new("^62").unwrap(), CardType::UnionPay),
            (Regex::new("^(50(18|20|38)|5893|6304|6759|676[1-3])").unwrap(), CardType::Maestro),
            (Regex::new("^(6011|64[4-9]|65|6221[2-9][6-9]|622[2-8][0-9]{2}|6229[0-1][0-9]{2}|62292[0-5])").unwrap(), CardType::Discover),
            (Regex::new("^(352[8-9]|35[3-8][0-9])").unwrap(), CardType::JapaneseBank),
            (Regex::new("^(60(4|5|6|7|8|9)|65(0|1|2|3|4|5|6|7|8|9)|81(0|1|2|3|4|5|6|7|8|9))").unwrap(), CardType::RuPay),
            (Regex::new("^(4011(78|79)|4312(74|75)|4389(35|36)|4514(16|17)|4576(31|32)|4576(50|51)|5041(75|76)|5066(99|98)|5090(00|01)|6277(80|81)|6362(97|96)|6363(68|69))").unwrap(), CardType::Elo),
        ];

        let cardtype = card_regex
            .iter()
            .find(|(regex, _)| regex.is_match(&pan))
            .map(|(_, card_type)| card_type.clone());

        Ok(cardtype)
    }
}

impl Rand for CardType {
    fn rand(_args: &[String]) -> Result<Self, Box<dyn Error>> {
        let card_types = [
            CardType::Visa,
            CardType::MasterCard,
            CardType::AmericanExpress,
            CardType::UnionPay,
            CardType::Maestro,
            CardType::Discover,
            CardType::JapaneseBank,
            CardType::RuPay,
            CardType::Elo,
        ];

        let mut rng = rand::rng();

        card_types
            .choose(&mut rng)
            .copied()
            .ok_or_else(|| "no card types available".into())
    }
}

impl TryFrom<&str> for CardType {
    type Error = Box<dyn Error>;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "visa" => Ok(CardType::Visa),
            "unionpay" => Ok(CardType::UnionPay),
            "mastercard" => Ok(CardType::MasterCard),
            "amex" | "american express" => Ok(CardType::AmericanExpress),
            "jcb" => Ok(CardType::JapaneseBank),
            "discover" => Ok(CardType::Discover),
            "rupay" => Ok(CardType::RuPay),
            "maestro" => Ok(CardType::Maestro),
            "elo" => Ok(CardType::Elo),
            _ => Err("Unknown card type".into()),
        }
    }
}
