use crate::constants::constants::{
    MRZ_TYPE1, MRZ_TYPE2, MRZ_TYPE3, TYPE1_NUMBER_OF_CHARACTERS_PER_LINE, TYPE1_TOTAL_NUMBER_OF_CHARACTERS,
    TYPE2_NUMBER_OF_CHARACTERS_PER_LINE, TYPE2_TOTAL_NUMBER_OF_CHARACTERS, TYPE3_NUMBER_OF_CHARACTERS_PER_LINE,
};
use crate::parser::parser::{IMRZParser, MRZResult};
use crate::parser::td1::TD1;
use crate::parser::td2::TD2;
use crate::parser::td3::TD3;
use crate::utils::utils::check_same;

mod utils;
mod parser;
mod constants;

pub struct MRZParser {
    mrz_type: u8,
    components: Vec<String>,
}

impl MRZParser {
    // Create a new MRZParser from a single MRZ string with lines separated by a newline character
    pub fn new_mrz_string_parser(mrz_str: &str) -> Self {
        let mrz_type: u8 = 0;
        let components: Vec<String> = if mrz_str.contains('\n') {
            mrz_str.lines().map(String::from).collect()
        } else if mrz_str.len() == TYPE1_TOTAL_NUMBER_OF_CHARACTERS {
            vec![
                mrz_str[..TYPE1_NUMBER_OF_CHARACTERS_PER_LINE].to_string(),
                mrz_str[TYPE1_NUMBER_OF_CHARACTERS_PER_LINE..2 * TYPE1_NUMBER_OF_CHARACTERS_PER_LINE].to_string(),
                mrz_str[2 * TYPE1_NUMBER_OF_CHARACTERS_PER_LINE..].to_string(),
            ]
        } else if mrz_str.len() == TYPE2_TOTAL_NUMBER_OF_CHARACTERS {
            vec![
                mrz_str[..TYPE2_NUMBER_OF_CHARACTERS_PER_LINE].to_string(),
                mrz_str[TYPE2_NUMBER_OF_CHARACTERS_PER_LINE..].to_string(),
            ]
        } else {
            vec![
                mrz_str[..TYPE3_NUMBER_OF_CHARACTERS_PER_LINE].to_string(),
                mrz_str[TYPE3_NUMBER_OF_CHARACTERS_PER_LINE..].to_string(),
            ]
        };

        MRZParser { mrz_type, components }
    }

    // Create a new MRZParser from a vector of MRZ lines
    pub fn new_mrz_line_parser(mrz_lines: Vec<String>) -> Self {
        let mrz_type: u8 = 0;
        MRZParser {
            mrz_type,
            components: mrz_lines,
        }
    }

    // Return the MRZ type
    pub fn get_mrz_type(&mut self) -> Result<u8, &'static str> {
        self.validate()?;
        Ok(self.mrz_type)
    }

    // Parse the MRZ information
    pub fn parse(&mut self) -> Result<MRZResult, &'static str> {
        self.validate()?;

        let mrz_parser: Box<dyn IMRZParser> = match self.mrz_type {
            MRZ_TYPE1 => Box::new(TD1::new()),
            MRZ_TYPE2 => Box::new(TD2::new()),
            MRZ_TYPE3 => Box::new(TD3::new()),
            _ => return Err("invalid mrz type"),
        };

        mrz_parser.parse(&self.components)
    }

    // Validate the input MRZ for formatting errors
    fn validate(&mut self) -> Result<(), &'static str> {
        let mut mrz_type = 0;

        match self.components.len() {
            3 => {
                for line in &self.components {
                    if line.len() != TYPE1_NUMBER_OF_CHARACTERS_PER_LINE {
                        return Err("invalid TD1 format line length");
                    }
                }
                mrz_type = MRZ_TYPE1;
            }
            2 => {
                let mut character_count = Vec::new();
                for line in &self.components {
                    if line.len() != TYPE2_NUMBER_OF_CHARACTERS_PER_LINE
                        && line.len() != TYPE3_NUMBER_OF_CHARACTERS_PER_LINE
                    {
                        return Err("invalid mrz line length");
                    }
                    character_count.push(line.len());
                }

                if check_same(&character_count) && character_count[0] == TYPE2_NUMBER_OF_CHARACTERS_PER_LINE {
                    mrz_type = MRZ_TYPE2;
                }
                if check_same(&character_count) && character_count[0] == TYPE3_NUMBER_OF_CHARACTERS_PER_LINE {
                    mrz_type = MRZ_TYPE3;
                }
            }
            _ => return Err("invalid mrz line length"),
        }

        self.mrz_type = mrz_type;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_td1_vec() {
        let mrz_string: Vec<String> = vec![
            "I<UTOD231458907<<<<<<<<<<<<<<<".to_string(),
            "7408122F1204159UTO<<<<<<<<<<<6".to_string(),
            "ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string(),
        ];

        let mut parser = MRZParser::new_mrz_line_parser(mrz_string);
        let result = parser.parse().unwrap();
        assert_eq!(result.is_valid, true);
        println!("{:?}", result)
    }

    #[test]
    fn test_td1_str() {
        let mrz_string: &str = "\
        I<UTOD231458907<<<<<<<<<<<<<<<\n\
        7408122F1204159UTO<<<<<<<<<<<6\n\
        ERIKSSON<<ANNA<MARIA<<<<<<<<<<";
        let mut parser = MRZParser::new_mrz_string_parser(mrz_string);
        let result = parser.parse().unwrap();
        assert_eq!(result.is_valid, true);
        println!("{:?}", result)
    }
}
