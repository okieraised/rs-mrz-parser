use crate::utils::utils::is_value_valid;

#[derive(Debug, Clone)]
pub struct MrzField {
    pub value: String,
    pub raw_value: String,
    pub check_digit: String,
    pub is_valid: bool,
}

impl MrzField {
    pub fn new(value: String, raw_value: String, check_digit: String, is_valid: bool) -> Self {
        Self {
            value,
            raw_value,
            check_digit,
            is_valid,
        }
    }

    // get_names: Returns the name field as a Vec<String>.
    pub fn get_names(&self) -> String {
        // match self.value {
        //     Some(names) => Ok(names),
        //     None => Err("Not a name field"),
        // }
        self.value.clone()
    }

    // get_value: Returns the parsed value (Option<Vec<String>> in this implementation).
    pub fn get_value(&self) -> String {
        self.value.clone()
    }

    // get_raw_value: Returns the field value as a string.
    pub fn get_raw_value(&self) -> &str {
        &self.raw_value
    }

    // get_check_digit: Returns the check digit value as a string.
    pub fn get_check_digit(&self) -> &str {
        &self.check_digit
    }

    // is_valid: Performs field validity check.
    pub fn is_valid(&mut self) -> bool {
        if self.check_digit == "<" {
            if !self.raw_value.trim_matches('<').is_empty() {
                return false;
            }
            self.check_digit = "0".to_string();
        } else if self.check_digit.parse::<i32>().is_err() {
            return false;
        }

        is_value_valid(&self.raw_value, &self.check_digit)
    }
}
