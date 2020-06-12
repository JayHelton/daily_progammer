use crate::problem_solving::{repeats, same_necklace};
use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

/// Problem 3 Solution
fn main() {
    let start = Instant::now();
    let size_to_find = 5;

    let file = File::open("./enable1.txt").expect("couldn't open file");
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|x| x.expect("couldnt parse line"))
        .filter(|x| x.len() == size_to_find)
        .collect();

    let new: Vec<String> = vec![];
    let result = lines
        .iter()
        .fold(new, |acc, x| {
            // i dont like full iteration here. I think i should be able to remove items that showed up in results
            // which means every iteration would have less items to compare against, but would require a lot of data shift
            let matches: Vec<String> = lines
                .clone()
                .into_iter()
                // this first filter actually makes it move a lot faster
                .filter(|y| x.clone().chars().all(|c| y.contains(c)))
                .filter(|y| same_necklace(x.to_string(), y.to_string()))
                .collect();

            return if acc.len() < matches.len() {
                println!("{} - {:?}", x, matches);
                matches
            } else {
                println!("{}", x);
                acc
            };
        });
    let duration = start.elapsed();

    println!("{:?} - {}", result, duration.as_secs())
}

mod problem_solving {
    /// Problem 2 Solution
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

    /// Problem 1 Solution
    pub fn same_necklace(a: String, b: String) -> bool {
        /// This method is overkill. You can just concat the string together twice and just do a .contains
        if a.len() != b.len() {
            return false;
        }
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
