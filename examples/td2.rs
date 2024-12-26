use rs_mrz_parser::MRZParser;

fn main() {
    let mrz_string: Vec<String> = vec![
        "I<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<".to_string(),
        "D231458907UTO7408122F1204159<<<<<<<6".to_string(),
    ];

    let mut parser = MRZParser::new_mrz_line_parser(mrz_string);
    let mrz_type = parser.get_mrz_type().unwrap();
    assert_eq!(mrz_type, 2);
    let result = parser.parse().unwrap();
    assert_eq!(result.is_valid, true);
}
