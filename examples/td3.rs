use rs_mrz_parser::MRZParser;

fn main() {
    let mrz_string: Vec<String> = vec![
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<".to_string(),
        "L898902C36UTO7408122F1204159ZE184226B<<<<<10".to_string(),
    ];

    let mut parser = MRZParser::new_mrz_line_parser(mrz_string);
    let mrz_type = parser.get_mrz_type().unwrap();
    assert_eq!(mrz_type, 3);
    let result = parser.parse().unwrap();
    assert_eq!(result.is_valid, true);
}