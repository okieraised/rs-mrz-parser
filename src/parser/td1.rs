use crate::constants::constants::{ISSUING_COUNTRY_CODES, TYPE1_NUMBER_OF_CHARACTERS_PER_LINE};
use crate::constants::mrz_field_name::{
    BIRTHDATE_FIELD, COUNTRY_CODE_FIELD, DOCUMENT_NUMBER_FIELD, DOCUMENT_TYPE_FIELD, EXPIRY_DATE_FIELD,
    FINAL_CHECK_DIGIT_FIELD, NAME_FIELD, NATIONALITY_FIELD, OPTIONAL_DATA_1_FIELD, OPTIONAL_DATA_2_FIELD, SEX_FIELD,
};
use crate::parser::field_formatter::FieldFormatter;
use crate::parser::field_formatter::FieldType::{
    Birthdate, CountryCode, DocumentNumber, DocumentType, ExpiryDate, Hash, Names, Nationality, OptionalData, Sex,
};
use crate::parser::mrz_field::MrzField;
use crate::parser::parser::{IMRZParser, MRZResult};
use crate::utils::utils::calculate_check_digits;
use std::collections::HashMap;

#[derive(Default, Debug, Clone)]
pub struct TD1 {}

impl TD1 {
    pub fn new() -> Self {
        TD1 {}
    }

    pub fn validate_all_check_digits(
        &self, document_number: &MrzField, optional_data1: &MrzField, birthdate: &MrzField, expiry_date: &MrzField,
        optional_data2: &MrzField, final_check_digit: &MrzField,
    ) -> Result<bool, &'static str> {
        let composite_str = format!(
            "{}{}{}{}{}{}{}{}{}{}",
            document_number.raw_value,
            document_number.check_digit,
            optional_data1.raw_value,
            optional_data1.check_digit,
            birthdate.raw_value,
            birthdate.check_digit,
            expiry_date.raw_value,
            expiry_date.check_digit,
            optional_data2.raw_value,
            optional_data2.check_digit
        );

        let calculated_check_digit = calculate_check_digits(&composite_str)?;

        Ok(document_number.is_valid
            && birthdate.is_valid
            && expiry_date.is_valid
            && (calculated_check_digit == final_check_digit.value))
    }
}

impl IMRZParser for TD1 {
    fn parse(&self, input: Vec<String>) -> Result<MRZResult, &'static str> {
        if input.len() != 3 {
            return Err("invalid mrz length");
        }

        for line in &input {
            if line.len() != TYPE1_NUMBER_OF_CHARACTERS_PER_LINE {
                return Err("invalid mrz type 1 line length");
            }
        }

        let first_line = &input[0];
        let second_line = &input[1];
        let third_line = &input[2];

        let formatter = FieldFormatter::new(true);

        // Parse first line
        let document_type = formatter.field(DocumentType, first_line, 0, 2, false)?;
        let country_code = formatter.field(CountryCode, first_line, 2, 3, false)?;
        let document_number = formatter.field(DocumentNumber, first_line, 5, 9, true)?;
        let optional_data1 = formatter.field(OptionalData, first_line, 15, 15, false)?;

        // Parse second line
        let birthdate = formatter.field(Birthdate, second_line, 0, 6, true)?;
        let sex = formatter.field(Sex, second_line, 7, 1, false)?;
        let expiry_date = formatter.field(ExpiryDate, second_line, 8, 6, true)?;
        let nationality = formatter.field(Nationality, second_line, 15, 3, false)?;
        let optional_data2 = formatter.field(OptionalData, second_line, 18, 11, false)?;
        let final_check_digit = formatter.field(Hash, second_line, 29, 1, false)?;

        // Parse third line
        let name = formatter.field(Names, third_line, 0, 30, false)?;

        // Validate all check digits
        let is_valid = self.validate_all_check_digits(
            &document_number, &optional_data1, &birthdate, &expiry_date, &optional_data2, &final_check_digit,
        )?;

        let mut parsed_result: HashMap<String, MrzField> = HashMap::new();
        parsed_result.insert(NAME_FIELD.to_string(), name);
        parsed_result.insert(DOCUMENT_TYPE_FIELD.to_string(), document_type);
        parsed_result.insert(COUNTRY_CODE_FIELD.to_string(), country_code);
        parsed_result.insert(DOCUMENT_NUMBER_FIELD.to_string(), document_number);
        parsed_result.insert(OPTIONAL_DATA_1_FIELD.to_string(), optional_data1);
        parsed_result.insert(BIRTHDATE_FIELD.to_string(), birthdate);
        parsed_result.insert(SEX_FIELD.to_string(), sex);
        parsed_result.insert(EXPIRY_DATE_FIELD.to_string(), expiry_date);
        parsed_result.insert(NATIONALITY_FIELD.to_string(), nationality);
        parsed_result.insert(OPTIONAL_DATA_2_FIELD.to_string(), optional_data2);
        parsed_result.insert(FINAL_CHECK_DIGIT_FIELD.to_string(), final_check_digit);

        let issuing_state = ISSUING_COUNTRY_CODES
            .get(&parsed_result[COUNTRY_CODE_FIELD].value as &str)
            .unwrap_or(&"Unknown")
            .to_string();

        Ok(MRZResult {
            is_visa: false,
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
    fn test_parse_td1() {
        let mrz_string: Vec<String> = vec![
            "I<UTOD231458907<<<<<<<<<<<<<<<".to_string(),
            "7408122F1204159UTO<<<<<<<<<<<6".to_string(),
            "ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string(),
        ];

        let mut td1 = TD1::new();
        let result = td1.parse(mrz_string).unwrap();

        println!("{:?}", result)
    }
}
