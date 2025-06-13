struct Solution {}

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon.len();
        let m = dungeon[0].len();

        let mut curr = vec![i32::MAX; m];
        let mut up = vec![0; m];

        curr[m - 1] = 1.max(1 - dungeon[n - 1][m - 1]);

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if j != m - 1 {
                    curr[j] = curr[j].min(1.max(curr[j + 1] - dungeon[i][j]));
                }

                if i != 0 {
                    up[j] = 1.max(curr[j] - dungeon[i - 1][j]);
                }
            }

            //println!("{:?}", curr);

            if i != 0 {
                curr.copy_from_slice(&up);
            }
        }

        return curr[0];
    }
}

fn main() {
    Solution::calculate_minimum_hp(vec![vec![0]]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let dungeon: Vec<Vec<i32>> = vec![vec![-2, -3, 3], vec![-5, -10, 1], vec![10, 30, -5]];
        let output = 7;

        assert_eq!(Solution::calculate_minimum_hp(dungeon), output);
    }

    #[test]
    fn test_1() {
        let dungeon: Vec<Vec<i32>> = vec![vec![0]];
        let output = 1;

        assert_eq!(Solution::calculate_minimum_hp(dungeon), output);
    }

    #[test]
    fn test_2() {
        let dungeon: Vec<Vec<i32>> = vec![vec![1, -3, 3], vec![0, -2, 0], vec![-3, -3, -3]];
        let output = 3;

        assert_eq!(Solution::calculate_minimum_hp(dungeon), output);
    }

    #[test]
    fn test_3() {
        let dungeon: Vec<Vec<i32>> = vec![vec![0, 0]];
        let output = 1;

        assert_eq!(Solution::calculate_minimum_hp(dungeon), output);
    }
}
