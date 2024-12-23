use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref alphabet_mapping: HashMap<char, i32> = {
        let m: HashMap<char, i32> = ('A'..='Z')
        .zip(10..36)
        .collect();
        m
    };
}

pub const WEIGHTS: [i32; 3] =[7, 3, 1];

