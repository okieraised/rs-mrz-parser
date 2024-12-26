# rs-mrz-parser

## Introduction
Automatically determine and parse MRZ string in Rust

## Installation
```rust
cargo add rs_mrz_parser
```

## Example

```rust
use rs_mrz_parser::constants::mrz_utils::MRZ_TYPE1;
use rs_mrz_parser::MRZParser;

fn main() {
    let mrz_string: Vec<String> = vec![
        "I<UTOD231458907<<<<<<<<<<<<<<<".to_string(),
        "7408122F1204159UTO<<<<<<<<<<<6".to_string(),
        "ERIKSSON<<ANNA<MARIA<<<<<<<<<<".to_string(),
    ];

    let mut parser = MRZParser::new_mrz_line_parser(mrz_string);
    let mrz_type = parser.get_mrz_type().unwrap();
    assert_eq!(mrz_type, MRZ_TYPE1);
    let result = parser.parse().unwrap();
    assert_eq!(result.is_valid, true);
}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details