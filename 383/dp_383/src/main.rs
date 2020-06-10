use crate::problem_solving::{repeats, same_necklace};

fn main() {
    println!("{}", repeats(""));
    println!("{}", same_necklace("", ""));
}

mod problem_solving {
    pub fn repeats(a: &str) -> usize {
        let mut a_char_vec: Vec<char> = a.chars().collect();
        let mut count = 0;
        let mut counter = 0;

        loop {
            count += 1;

            let state: String = a_char_vec.iter().collect();
            if state == a.to_string() {
                counter += 1;
            }

            if a_char_vec.len() <= count  {
                break;
            }

            a_char_vec.rotate_left(1);
        }
        return counter;
    }

    pub fn same_necklace(a: &str, b: &str) -> bool {
        let mut a_char_vec: Vec<char> = a.chars().collect();
        let mut count = 0;

        loop {
            count += 1;

            let state: String = a_char_vec.iter().collect();
            if state == b.to_string() {
                return true;
            }

            if a_char_vec.len() <= count  {
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
            assert_eq!(repeats("abc"), 1);
        }

        #[test]
        fn test_repeat_2() {
            assert_eq!(repeats("abcabcabc"), 3);
        }

        #[test]
        fn test_repeat_3() {
            assert_eq!(repeats("abcabcabcx"), 1);
        }

        #[test]
        fn test_repeat_4() {
            assert_eq!(repeats("aaaaaa"), 6);
        }

        #[test]
        fn test_repeat_5() {
            assert_eq!(repeats("a"), 1);
        }

        #[test]
        fn test_repeat_6() {
            assert_eq!(repeats(""), 1);
        }

        #[test]
        fn test_same_necklace_1() {
            assert!(same_necklace("nicole", "icolen"));
        }

        #[test]
        fn test_same_necklace_2() {
            assert!(same_necklace("nicole", "lenico"));
        }

        #[test]
        fn test_same_necklace_3() {
            assert!(!same_necklace("nicole", "coneli"));
        }

        #[test]
        fn test_same_necklace_4() {
            assert!(same_necklace("aabaaaaabaab", "aabaabaabaaa"));
        }

        #[test]
        fn test_same_necklace_5() {
            assert!(!same_necklace("abc", "cba"));
        }

        #[test]
        fn test_same_necklace_6() {
            assert!(!same_necklace("xxyyy", "xxxyy"));
        }

        #[test]
        fn test_same_necklace_7() {
            assert!(!same_necklace("xyxxz", "xxyxz"));
        }

        #[test]
        fn test_same_necklace_8() {
            assert!(same_necklace("x", "x"));
        }

        #[test]
        fn test_same_necklace_9() {
            assert!(!same_necklace("x", "xx"));
        }

        #[test]
        fn test_same_necklace_10() {
            assert!(!same_necklace("x", ""));
        }

        #[test]
        fn test_same_necklace_11() {
            assert!(same_necklace("", ""));
        }
    }
}
