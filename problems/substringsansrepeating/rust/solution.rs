use std::{cmp::max, collections::HashMap};

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }

        let mut max_length: i32 = 1;

        let mut i: usize = 0;
        let mut j: usize = 1;

        let mut seen: HashMap<char, usize> = HashMap::new();
        
        seen.insert(s.chars().nth(0).unwrap(), 0);

        while j < s.len().try_into().unwrap() {
            let mut seen_idx: usize = 0;
            let mut is_seen = false;

            match seen.get(&s.chars().nth(j).unwrap()) {
                Some(idx) => {
                    seen_idx = *idx;
                    is_seen = true;
                }
                None => (),
            };

            while is_seen && i <= seen_idx.try_into().unwrap() {
                seen.remove(&s.chars().nth(i as usize).unwrap());
                i += 1;
            }

            seen.insert(s.chars().nth(j as usize).unwrap(), j);

            max_length = max(max_length, (j - i + 1) as i32);

            j += 1;
        }

        return max_length;
    }
}

// A clear and concise solution
//
//impl Solution {
//    pub fn length_of_longest_substring(s: String) -> i32 {
//        let s = s.chars().collect::<Vec<_>>();
//        let mut substr: HashSet<&char> = HashSet::new();
//        let mut head = 0usize;
//        let mut result = 0usize;
//
//        for ch in s.iter() {
//            while substr.contains(ch) {
//                let to_rm = s.get(head).expect("should exist");
//                substr.remove(to_rm);
//                head += 1;
//            }
//            substr.insert(ch);
//            result = usize::max(result, substr.len());
//        }
//
//        result as i32
//    }
//}

fn main() {
    Solution::length_of_longest_substring(String::from("abcd"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let s = String::from("abcabcbb");
        let expected = 3;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_1() {
        let s = String::from("bbbbb");
        let expected = 1;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_2() {
        let s = String::from("pwwkew");
        let expected = 3;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_3() {
        let s = String::from("");
        let expected = 0;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_4() {
        let s = String::from("abcdefghij");
        let expected = 10;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }

    #[test]
    fn test_5() {
        let s = String::from(" ");
        let expected = 1;

        assert_eq!(Solution::length_of_longest_substring(s), expected);
    }
}
