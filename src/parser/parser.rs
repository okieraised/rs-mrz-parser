use crate::parser::mrz_field::MrzField;
use std::collections::HashMap;

#[derive(Debug)]
pub struct MRZResult {
    pub is_visa: bool,
    pub is_valid: bool,
    pub fields: HashMap<String, MrzField>,
    pub issuing_state: String,
}

pub trait IMRZParser {
    fn parse(&self, input: &Vec<String>) -> Result<MRZResult, &'static str>;
}
