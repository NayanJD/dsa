use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn problem(input: i32) -> i32 {
        return input;
    }
}

fn main() {
    Solution::problem(9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::problem(9), 9);
    }
}
