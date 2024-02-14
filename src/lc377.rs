impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target];
        dp[0] = 1;
        for i in 0..target {
            for j in &nums {
                if i+j > target {
                    continue;
                }
                dp[i+j] += dp[i];
            }
        }
        dp[target]
    }
}