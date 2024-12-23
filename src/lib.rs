mod utils;
mod parser;
mod constants;

pub struct MRZParser {
    mrz_type: u8,
    components: Vec<String>,
}



pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
