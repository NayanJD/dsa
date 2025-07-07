use std::cmp::{max};

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut largest = -1;

        let mut i = 0;
        let mut j = height.len() - 1;

        while i < j {
            largest = max(largest, (j - i) as i32 * height[i].min(height[j]));

            if height[i] == height[j] {
                i += 1;
                j -= 1;
            } else if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        };

        largest
    }
}

fn main() {
    Solution::max_area(vec![1, 2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let heights = vec![1,8,6,2,5,4,8,3,7];

        assert_eq!(Solution::max_area(heights), 49);
    }
}
