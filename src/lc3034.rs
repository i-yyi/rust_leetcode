
use super::Solution;

impl Solution {
    fn mod_pow(base: i64, exp: u32, modulus: i64) -> i64 {
        if modulus == 1 {
            return 0;
        }
        let mut result = 1;
        let mut base = base % modulus;
        let mut exp = exp;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result * base % modulus;
            }
            exp >>= 1; // 相当于 exp /= 2
            base = base * base % modulus;
        }
        result
    }
    pub fn count_matching_subarrays(nums: Vec<i32>, pattern: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (n, m) = (nums.len(), pattern.len());
        let nums: Vec<i64>= nums.windows(2)
        .map(|window| match window {
            [a, b] if a < b => 1,
            [a, b] if a == b => 0,
            [a, b] if a > b => -1,
            _ => unreachable!(),
        })
        .collect();

        let base = 13131_i64;
        let p = 1_000_000_007_i64;
        let hash = nums.iter().fold(vec![0_i64], |mut hash, &num| {
            hash.push((hash.last().unwrap() * base + num + 1) % p);
            hash
        });


        let target_hash = pattern.iter().fold(0_i64, |hash, &num| (hash * base + num as i64 + 1) % p);
        for i in 0..n {
            if i + m >= n {
                break;
            }
            let hash =( hash[i + m] - (hash[i] * Self::mod_pow(base, m as u32, p) % p ) + p) % p;
            if hash == target_hash {
                ans += 1;
            }
        }
        ans
        
    }
}