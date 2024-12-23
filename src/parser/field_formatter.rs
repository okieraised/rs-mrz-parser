use crate::parser::mrz_field::MrzField;
use crate::utils::utils::{replace_digits, replace_letters};


#[derive(Debug, Clone, Copy)]
pub enum FieldType {
    Names,
    Birthdate,
    ExpiryDate,
    Sex,
    PersonalNumber,
    OptionalData,
    DocumentType,
    DocumentNumber,
    CountryCode,
    Nationality,
    Alphabetic,
    Numeric,
    Hash,
}

#[derive(Debug)]
pub struct FieldFormatter {
    ocr_correction: bool,
}

impl FieldFormatter {
    pub fn new(ocr_correction: bool) -> Self {
        FieldFormatter { ocr_correction }
    }

    pub fn field(
        &self,
        field_type: FieldType,
        from: &str,
        start_idx: usize,
        length: usize,
        check_digit_follow: bool,
    ) -> Result<MrzField, &'static str> {
        let end_idx = start_idx + length;
        let raw_value = &from[start_idx..end_idx];
        let mut check_digit = String::new();

        if check_digit_follow {
            check_digit = from.chars().nth(end_idx).map(|c| c.to_string()).unwrap_or_default();
        }

        let mut corrected_raw_value = raw_value.to_string();
        if self.ocr_correction {
            corrected_raw_value = self.correct(&raw_value, field_type);
        }

        let formatted_value = self.format(&corrected_raw_value, field_type)?;
        let is_valid = true;

        let mut result = MrzField {
            value: formatted_value,
            raw_value: corrected_raw_value,
            check_digit,
            is_valid,
        };

        if check_digit_follow {
            result.is_valid();
        }

        Ok(result)
    }

    pub fn sex(&self, from: &str) -> &str {
        match from {
            "M" => "MALE",
            "F" => "FEMALE",
            "<" => "UNSPECIFIED",
            _ => "",
        }
    }

    pub fn names(&self, from: &str) -> Vec<String> {
        let identifiers: Vec<&str> = from.split("<<").collect();
        let primary = identifiers[0].replace('<', " ");
        let secondary = if identifiers.len() > 1 {
            identifiers[1].replace('<', " ")
        } else {
            String::new()
        };
        vec![primary, secondary]
    }

    pub fn text(&self, from: &str) -> String {
        from.replace('<', " ")
    }

    pub fn date(&self, from: &str) -> Result<String, &'static str> {
        if from.contains('<') {
            return Ok(from.to_string());
        }

        for digit in from.chars() {
            if digit != '<' && !digit.is_digit(10) {
                return Err("Invalid birthdate character");
            }
        }

        // Simulate date parsing logic (adjust as necessary for your use case)
        if from.len() == 6 {
            Ok(from.to_string())
        } else {
            Err("Invalid date format")
        }
    }

    // Replace methods
    pub fn replace_digits(&self, from: &str) -> String {
        replace_digits(from)
    }

    pub fn replace_letters(&self, from: &str) -> String {
        replace_letters(from)
    }

    // Correction logic
    pub fn correct(&self, from: &str, field_type: FieldType) -> String {
        match field_type {
            FieldType::Birthdate
            | FieldType::ExpiryDate
            | FieldType::Hash
            | FieldType::Numeric => self.replace_letters(from),
            FieldType::Names
            | FieldType::DocumentType
            | FieldType::CountryCode
            | FieldType::Nationality
            | FieldType::Alphabetic => self.replace_digits(from),
            FieldType::Sex => from.replace('P', "F"),
            _ => from.to_string(),
        }
    }

    // Formatting logic
    pub fn format(&self, from: &str, field_type: FieldType) -> Result<String, &'static str> {
        match field_type {
            FieldType::Names => Ok(self.names(from).join(" ")),
            FieldType::Birthdate | FieldType::ExpiryDate => self.date(from),
            FieldType::Sex => Ok(self.sex(from).to_string()),
            _ => Ok(from.to_string()),
        }
    }
}
