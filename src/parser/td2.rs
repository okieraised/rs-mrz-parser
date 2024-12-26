use crate::constants::constants::{ISSUING_COUNTRY_CODES, TYPE2_NUMBER_OF_CHARACTERS_PER_LINE};
use crate::constants::mrz_field_name::{
    BIRTHDATE_FIELD, COUNTRY_CODE_FIELD, DOCUMENT_NUMBER_FIELD, DOCUMENT_TYPE_FIELD, EXPIRY_DATE_FIELD,
    FINAL_CHECK_DIGIT_FIELD, NAME_FIELD, NATIONALITY_FIELD, OPTIONAL_DATA_1_FIELD, SEX_FIELD,
};
use crate::parser::field_formatter::FieldFormatter;
use crate::parser::field_formatter::FieldType::{
    Birthdate, CountryCode, DocumentNumber, DocumentType, ExpiryDate, Hash, Names, Nationality, PersonalNumber, Sex,
};
use crate::parser::mrz_field::MrzField;
use crate::parser::parser::{IMRZParser, MRZResult};
use crate::utils::utils::calculate_check_digits;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct TD2 {}

impl TD2 {
    pub fn new() -> Self {
        TD2 {}
    }

    fn validate_all_check_digits(
        &self, document_number: &MrzField, birthdate: &MrzField, expiry_date: &MrzField, optional_data: &MrzField,
        final_check_digit: Option<&MrzField>,
    ) -> Result<bool, &'static str> {
        if let Some(final_check) = final_check_digit {
            let composite_str = format!(
                "{}{}{}{}{}{}{}",
                document_number.raw_value,
                document_number.check_digit,
                birthdate.raw_value,
                birthdate.check_digit,
                expiry_date.raw_value,
                expiry_date.check_digit,
                optional_data.raw_value
            );

            let calculated_check_digit = calculate_check_digits(&composite_str)?;
            Ok(document_number.is_valid
                && birthdate.is_valid
                && expiry_date.is_valid
                && calculated_check_digit == final_check.raw_value)
        } else {
            Ok(document_number.is_valid && birthdate.is_valid && expiry_date.is_valid)
        }
    }
}


impl IMRZParser for TD2 {
    fn parse(&self, input: &Vec<String>) -> Result<MRZResult, &'static str> {
        if input.len() != 2 {
            return Err("invalid mrz length");
        }

        for line in input {
            if line.len() != TYPE2_NUMBER_OF_CHARACTERS_PER_LINE {
                return Err("invalid mrz type 2 line length");
            }
        }

        let mut is_visa = false;
        let first_line = &input[0];
        let second_line = &input[1];
        let formatter = FieldFormatter::new(true);

        if first_line.chars().nth(0) == Some('V') {
            is_visa = true;
        }

        // Parse first line
        let document_type = formatter.field(DocumentType, first_line, 0, 2, false)?;

        let country_code = formatter.field(CountryCode, first_line, 2, 3, false)?;

        let name = formatter.field(Names, first_line, 5, 31, false)?;

        // Parse second line
        let document_number = formatter.field(DocumentNumber, second_line, 0, 9, true)?;

        let nationality = formatter.field(Nationality, second_line, 10, 3, false)?;

        let birthdate = formatter.field(Birthdate, second_line, 13, 6, true)?;

        let sex = formatter.field(Sex, second_line, 20, 1, false)?;

        let expiry_date = formatter.field(ExpiryDate, second_line, 21, 6, true)?;

        let mut optional_data: MrzField = MrzField {
            value: "".to_string(),
            raw_value: "".to_string(),
            check_digit: "".to_string(),
            is_valid: true,
        };

        let mut final_check_digit: MrzField = MrzField {
            value: "".to_string(),
            raw_value: "".to_string(),
            check_digit: "".to_string(),
            is_valid: true,
        };

        let mut is_valid: bool = false;

        if is_visa {
            optional_data = formatter.field(PersonalNumber, second_line, 28, 8, false)?;

            is_valid =
                self.validate_all_check_digits(&document_number, &birthdate, &expiry_date, &optional_data, None)?;
        } else {
            optional_data = formatter.field(PersonalNumber, second_line, 28, 7, true)?;

            final_check_digit = formatter.field(Hash, second_line, 35, 1, false)?;

            is_valid = self.validate_all_check_digits(
                &document_number,
                &birthdate,
                &expiry_date,
                &optional_data,
                Some(&final_check_digit),
            )?;
        }

        let mut parsed_result: HashMap<String, MrzField> = HashMap::new();
        parsed_result.insert(NAME_FIELD.to_string(), name);
        parsed_result.insert(DOCUMENT_TYPE_FIELD.to_string(), document_type);
        parsed_result.insert(COUNTRY_CODE_FIELD.to_string(), country_code);
        parsed_result.insert(DOCUMENT_NUMBER_FIELD.to_string(), document_number);
        parsed_result.insert(NATIONALITY_FIELD.to_string(), nationality);
        parsed_result.insert(BIRTHDATE_FIELD.to_string(), birthdate);
        parsed_result.insert(SEX_FIELD.to_string(), sex);
        parsed_result.insert(EXPIRY_DATE_FIELD.to_string(), expiry_date);
        parsed_result.insert(OPTIONAL_DATA_1_FIELD.to_string(), optional_data);
        parsed_result.insert(FINAL_CHECK_DIGIT_FIELD.to_string(), final_check_digit);

        let issuing_state = ISSUING_COUNTRY_CODES
            .get(&parsed_result[COUNTRY_CODE_FIELD].value as &str)
            .unwrap_or(&"Unknown")
            .to_string();

        Ok(MRZResult {
            is_visa,
            is_valid,
            fields: parsed_result,
            issuing_state,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_td2() {
        let mrz_string: Vec<String> = vec![
            "I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<".to_string(),
            "D231458907UTO7408122F1204159<<<<<<<6".to_string(),
        ];

        let mut td2 = TD2::new();
        let result = td2.parse(&mrz_string).unwrap();
        assert_eq!(result.is_visa, false);
        assert_eq!(result.is_valid, true);
        println!("{:?}", result)
    }

    #[test]
    fn test_parse_td2_visa() {
        let mrz_string: Vec<String> = vec![
            "V<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<".to_string(),
            "L8988901C4XXX4009078F9612109<<<<<<<<".to_string(),
        ];

        let mut td2 = TD2::new();
        let result = td2.parse(&mrz_string).unwrap();
        assert_eq!(result.is_visa, true);
        assert_eq!(result.is_valid, true);
        println!("{:?}", result)
    }
}
