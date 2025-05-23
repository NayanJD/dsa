use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0, 0];

        let mut table: HashMap<i32, i32> = HashMap::new();

        for i in 0..nums.len() {
            match table.get(&(target - nums[i])) {
                Some(index) => {
                    result[0] = *index;
                    result[1] = i as i32;
                    break;
                }
                None => {
                    table.insert(nums[i], i as i32);
                }
            }
        }
        return result;
    }
}

fn main() {
    Solution::two_sum(vec![2, 7, 11, 5], 9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 5], 9), vec![0, 1]);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![3,2,4], 6), vec![1, 2]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3,3], 6), vec![0, 1]);
    }
}
