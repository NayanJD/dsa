use std::collections::HashMap;
use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let mut table: HashMap<i32, i32> = HashMap::new();
        let mut u_table: HashSet<i32> = HashSet::new();

        let mut m_nums = nums;
        m_nums.sort();

        let mut prev = 1e5 as i32 + 1;

        for i in 0..m_nums.len() {
            if m_nums[i] == prev {
                continue;
            }

            let target = -m_nums[i];

            for j in i + 1..m_nums.len() {
                if table.contains_key(&m_nums[j]) && !u_table.contains(&m_nums[j]) {
                    result.push(vec![
                        m_nums[i],
                        m_nums[table[&m_nums[j]] as usize],
                        m_nums[j],
                    ]);
                    u_table.insert(m_nums[j]);
                } else {
                    table.insert(target - m_nums[j], j as i32);
                }
            }

            table.clear();
            u_table.clear();
            prev = m_nums[i];
        }

        return result;
    }
}

fn main() {
    Solution::three_sum(vec![0, 0, 0]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        let mut result = Solution::three_sum(nums);

        expected.sort();
        result.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_1() {
        let nums = vec![0, 1, 1];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::three_sum(nums), expected);
    }

    #[test]
    fn test_2() {
        let nums = vec![0, 0, 0];
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        assert_eq!(Solution::three_sum(nums), expected);
    }
}
