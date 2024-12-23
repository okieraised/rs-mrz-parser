use crate::constants::constants::{ALPHABET_MAPPING, WEIGHTS};

pub fn is_value_valid(value: &str, check_digit: &str) -> bool {
    let mut total = 0;

    for (idx, ch) in value.to_uppercase().chars().enumerate() {
        let char_value = if ALPHABET_MAPPING.contains_key(&ch) {
            *ALPHABET_MAPPING.get(&ch).unwrap()
        } else if ch.is_digit(10) {
            ch.to_digit(10).unwrap() as i32
        } else if ch == '<' {
            0
        } else {
            return false;
        };
        total += char_value * WEIGHTS[idx % WEIGHTS.len()];
    }

    (total % 10).to_string() == check_digit
}

pub fn calculate_check_digits(value: &str) -> Result<String, &'static str> {
    let mut total = 0;

    for (idx, ch) in value.to_uppercase().chars().enumerate() {
        let char_value = if ALPHABET_MAPPING.contains_key(&ch) {
            *ALPHABET_MAPPING.get(&ch).unwrap()
        } else if ch.is_digit(10) {
            ch.to_digit(10).unwrap() as i32
        } else if ch == '<' {
            0
        } else {
            return Err("Invalid MRZ character");
        };

        total += char_value * WEIGHTS[idx % WEIGHTS.len()];
    }

    Ok((total % 10).to_string())
}

pub fn replace_digits(input: &str) -> String {
    let replacements = vec![
        ('0', 'O'),
        ('1', 'I'),
        ('2', 'Z'),
        ('8', 'B'),
    ];

    input
        .chars()
        .map(|ch| replacements.iter().find(|(k, _)| *k == ch).map_or(ch, |(_, v)| *v))
        .collect()
}

pub fn replace_letters(input: &str) -> String {
    let replacements = vec![
        ('O', '0'),
        ('Q', '0'),
        ('U', '0'),
        ('D', '0'),
        ('I', '1'),
        ('Z', '2'),
        ('B', '8'),
    ];

    input
        .chars()
        .map(|ch| replacements.iter().find(|(k, _)| *k == ch).map_or(ch, |(_, v)| *v))
        .collect()
}


pub fn trimming_filler(input: &str) -> String {
    input.trim_matches('<').to_string()
}

pub fn check_same<T: PartialEq>(slice: &[T]) -> bool {
    if slice.is_empty() {
        return true;
    }
    slice.iter().all(|v| *v == slice[0])
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_value_valid() {
    }
}