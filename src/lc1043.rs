use super::Solution;
impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {

        let k = k as usize;
        let len = arr.len();
        let mut dp = vec![0; arr.len()];
        
        for i in 0..k {
            dp[i] = *arr.iter().take(i+1).max().unwrap() * (i as i32 + 1);
        }
        for i in 0..len {
            let mut max = 0;
            for j in 1..k {
                // dp i -> dp[i+1] .. dp[i+k]
                if i+j >= len {
                    break;
                }
                max = max.max(arr[i+j]);
                dp[i+j] = dp[i+j].max(dp[i] + max * (j as i32));
            }
        }
        dp[len-1]
    }
}