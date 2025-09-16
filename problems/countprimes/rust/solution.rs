struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }

        let mut sieve = vec![1; n as usize];
        sieve[1] = 0;

        for i in 1..=n.isqrt() {
            if sieve[i as usize] == 1 {
                let mut powered_value: i32 = i * i;
                while powered_value < n {
                    sieve[powered_value as usize] = 0;
                    powered_value += i;
                }
            }
        }

        let mut prime_count: i32 = 0;

        for i in 1..n {
            if sieve[i as usize] == 1 {
                prime_count += 1;
            }
        }

        return prime_count;
    }
}

fn main() {
    Solution::count_primes(9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(Solution::count_primes(10), 4);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::count_primes(0), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_primes(1), 0);
    }
}
