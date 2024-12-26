use rs_mrz_parser::MRZParser;

fn main() {
    let mrz_string: Vec<String> = vec![
        "I<UTOD231458907<<<<<<<<<<<<<<<".to_string(),
        "7408122F1204159UTO<<<<<<<<<<<6".to_string(),
        "ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string(),
    ];

    let mut parser = MRZParser::new_mrz_line_parser(mrz_string);
    let mrz_type = parser.get_mrz_type().unwrap();
    assert_eq!(mrz_type, 1);
    let result = parser.parse().unwrap();
    assert_eq!(result.is_valid, true);
}
