use std::str::FromStr;

use crate::domain::game::models::fen::FenValidator;

pub struct Validator;

impl FenValidator for Validator {
    fn is_valid_fen(&self, fen: &str) -> bool {
        shakmaty::fen::Fen::from_str(fen).is_ok()
    }
}
