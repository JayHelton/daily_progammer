use crate::problem_solving::{repeats, same_necklace};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // This works but is very very slow
    let file = File::open("./enable1.txt").expect("couldn't open file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|x| x.expect("couldnt parse line"))
        .collect();

    let new: Vec<String> = vec![];
    let result = lines
        .iter()
        .fold(new, |mut acc, x| {
            // Find all the matches with the necklace challenge
            let matches: Vec<String> = lines
                .clone()
                .into_iter()
                .filter(|y| same_necklace(x.to_string(), y.to_string()))
                .collect();

            return if acc.len() < matches.len() {
                println!("{} - {:?}", x, matches);
                matches
            } else {
                acc
            }
        });

    println!("{:?}", result)
}

mod problem_solving {
    pub fn repeats(a: String) -> usize {
        let mut a_char_vec: Vec<char> = a.chars().collect();
        let mut count = 0;
        let mut counter = 0;

        loop {
            count += 1;

            let state: String = a_char_vec.iter().collect();
            if state == a.to_string() {
                counter += 1;
            }

            if a_char_vec.len() <= count {
                break;
            }

            a_char_vec.rotate_left(1);
        }
        return counter;
    }

    pub fn same_necklace(a: String, b: String) -> bool {
        let mut a_char_vec: Vec<char> = a.chars().collect();
        let mut count = 0;

        loop {
            count += 1;

            let state: String = a_char_vec.iter().collect();
            if state == b.to_string() {
                return true;
            }

            if a_char_vec.len() <= count {
                break;
            }

            a_char_vec.rotate_left(1);
        }
        return false;
    }

    #[cfg(test)]
    mod problem_solving_tests {
        use super::*;

        #[test]
        fn test_repeat_1() {
            assert_eq!(repeats("abc".to_string()), 1);
        }

        #[test]
        fn test_repeat_2() {
            assert_eq!(repeats("abcabcabc".to_string()), 3);
        }

        #[test]
        fn test_repeat_3() {
            assert_eq!(repeats("abcabcabcx".to_string()), 1);
        }

        #[test]
        fn test_repeat_4() {
            assert_eq!(repeats("aaaaaa".to_string()), 6);
        }

        #[test]
        fn test_repeat_5() {
            assert_eq!(repeats("a".to_string()), 1);
        }

        #[test]
        fn test_repeat_6() {
            assert_eq!(repeats("".to_string()), 1);
        }

        #[test]
        fn test_same_necklace_1() {
            assert!(same_necklace("nicole".to_string(), "icolen".to_string()));
        }

        #[test]
        fn test_same_necklace_2() {
            assert!(same_necklace("nicole".to_string(), "lenico".to_string()));
        }

        #[test]
        fn test_same_necklace_3() {
            assert!(!same_necklace("nicole".to_string(), "coneli".to_string()));
        }

        #[test]
        fn test_same_necklace_4() {
            assert!(same_necklace(
                "aabaaaaabaab".to_string(),
                "aabaabaabaaa".to_string()
            ));
        }

        #[test]
        fn test_same_necklace_5() {
            assert!(!same_necklace("abc".to_string(), "cba".to_string()));
        }

        #[test]
        fn test_same_necklace_6() {
            assert!(!same_necklace("xxyyy".to_string(), "xxxyy".to_string()));
        }

        #[test]
        fn test_same_necklace_7() {
            assert!(!same_necklace("xyxxz".to_string(), "xxyxz".to_string()));
        }

        #[test]
        fn test_same_necklace_8() {
            assert!(same_necklace("x".to_string(), "x".to_string()));
        }

        #[test]
        fn test_same_necklace_9() {
            assert!(!same_necklace("x".to_string(), "xx".to_string()));
        }

        #[test]
        fn test_same_necklace_10() {
            assert!(!same_necklace("x".to_string(), "".to_string()));
        }

        #[test]
        fn test_same_necklace_11() {
            assert!(same_necklace("".to_string(), "".to_string()));
        }
    }
}
