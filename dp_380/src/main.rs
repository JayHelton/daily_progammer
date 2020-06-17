use crate::smorse_morse::smorse;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::borrow::Borrow;
use std::time::{Duration, Instant};
use std::collections::HashMap;

fn main() {
    let start = Instant::now();
    let mut morse_vec: HashMap<String, vec<String>> = HashMap::new();

    let file = File::open("../enable1.txt").expect("couldn't open file");
    let smorse_lines: Vec<String> = BufReader::new(file)
        .lines()
        .fold(morse_vec, |mut accum, curr| {
            let morse_word: &str = curr.expect("").as_str();
            let smorse_word: String = smorse(morse_word);
            let map_thing: Option<Vec<String>> = accum.get(smorse_word.borrow());
            match map_thing {
                Some(val) => {
                    val.push(smorse_word)
                }
                None => {
                    accum.insert(smorse_word, vec! {morse_word})
                }
            }
            accum
        })
        .collect();
    let duration = start.elapsed();

    println!("{:?}", smorse_lines);
    println!("{}", duration.as_secs());
}

mod smorse_morse {
    pub fn smorse(word: &str) -> String {
        let morse_vec: Vec<&str> = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..".split(" ").collect();
        let offset: char = 'a';
        word.chars()
            .map(|x| {
                let index = ((x as u32) - (offset as u32)) as usize;
                *morse_vec.get(index).expect("Error getting morse value")
            })
            .collect()
    }

    #[cfg(test)]
    mod problem_solving_tests {
        use super::*;

        #[test]
        fn test_smoosh_1() {
            assert_eq!(smorse("sos"), "...---...");
        }

        #[test]
        fn test_smoosh_2() {
            assert_eq!(smorse("daily"), "-...-...-..-.--");
        }

        #[test]
        fn test_smoosh_3() {
            assert_eq!(smorse("programmer"), ".--..-.-----..-..-----..-.");
        }

        #[test]
        fn test_smoosh_4() {
            assert_eq!(smorse("bits"), "-.....-...");
        }

        #[test]
        fn test_smoosh_5() {
            assert_eq!(smorse("three"), "-.....-...");
        }
    }
}
