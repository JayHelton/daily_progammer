mod utils;

use wasm_bindgen::prelude::*;
use crate::ggg_translation::ggg_translate;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn translate(key_mapping: &str, phrase: &str) -> String {
    ggg_translate(key_mapping, phrase)
}

mod ggg_translation {
    use std::collections::HashMap;

    pub fn ggg_translate(key_mapping: &str, phrase: &str) -> String {
        ggg_decode(key_mapping, phrase)
    }

    fn ggg_decode(key_mapping: &str, phrase: &str) -> String {
        let mut accu: String = String::new();

        let mut keys: Vec<char> = Vec::new();
        let mut values: Vec<&str> = Vec::new();
        let mut decoded: Vec<char> = Vec::new();

        let keymaps: Vec<&str> = key_mapping.split(" ").collect();
        for (i, k) in keymaps.iter().enumerate() {
            let thing: Vec<char> = k.chars().collect();

            if i % 2 == 0 {
                keys.push(thing[0]);
            } else {
                values.push(k)
            }
        }
        let char_vec: Vec<char> = phrase.chars().collect();
        for c in char_vec {
            println!("{}", c);
            if c != 'g' && c != 'G' {
                decoded.push(c);
            } else {
                accu.push(c);

                if values.contains(&accu.as_str()) {
                    decoded.push(keys[
                        values.iter().position(|&r| r == accu.as_str()).expect("messed up")
                        ]);
                    accu = String::new();
                }
            }
        }
        println!("{:?}", values);
        println!("{:?}", keys);
        println!("{:?}", decoded);
        return decoded.iter().collect();
    }

    fn ggg_encode() {}

    #[cfg(test)]
    mod ggg_translation_tests {
        use super::*;

        #[test]
        fn test_ggg_decode() {
            assert_eq!(ggg_decode("H GgG d gGg e ggG l GGg o gGG r Ggg w ggg", "GgGggGGGgGGggGG, ggggGGGggGGggGg!"), "Hello, world!".to_string());
        }
    }
}
