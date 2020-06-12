use crate::smoosh_morse::smorse;

fn main() {
    smorse("apple");
}

mod smoosh_morse {

    pub fn smorse(word: &str) -> String {
        let morse_vec: Vec<&str> = ".- -... -.-. -.. . ..-. --. .... .. .--- -.- .-.. -- -. --- .--. --.- .-. ... - ..- ...- .-- -..- -.-- --..".split(" ").collect();
        let offset: char = 'a';
        word.chars()
            .map(|x| {
                let index = ((x as u32) - (offset as u32)) as usize;
                *morse_vec.get(index).expect("Error getting morse value")
            })
            .into_iter()
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
