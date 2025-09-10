struct Solution {}

impl Solution {
    pub fn problem(input: i32) -> i32 {
        let mut result: i32 = 0;
        let mut last_digit: i32 = 0;
        let mut mutable_input: i32 = input;

        while mutable_input != 0 {
            last_digit = mutable_input % 10;
            if input > 0 && ((i32::MAX - last_digit) / 10) < result {
                return 0;
            } else if input < 0 && ((i32::MIN - last_digit) / 10) > result {
                return 0;
            }
            result = result * 10 + last_digit;
            mutable_input /= 10;
        }

        return result;
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
        let input: i32 = 9;
        let expected: i32 = 9;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_1() {
        let input: i32 = 12;
        let expected: i32 = 21;
        //println!("Value: {}", Solution::problem(input));

        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_2() {
        let input: i32 = 12345678;
        let expected: i32 = 87654321;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_3() {
        let input: i32 = 1234567892;
        let expected: i32 = 0;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_4() {
        let input: i32 = i32::MAX;
        let expected: i32 = 0;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_5() {
        let input: i32 = 0;
        let expected: i32 = 0;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_6() {
        let input: i32 = 123456789;
        let expected: i32 = 987654321;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_7() {
        let input: i32 = -123;
        let expected: i32 = -321;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_8() {
        let input: i32 = i32::MIN;
        let expected: i32 = 0;
        assert_eq!(Solution::problem(input), expected);
    }

    #[test]
    fn test_9() {
        let input: i32 = 1534236469;
        let expected: i32 = 0;
        assert_eq!(Solution::problem(input), expected);
    }
}
