impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        nums.sort();
        let mut dp = vec![(None, 1); n + 1];
        let mut mxn = 1;
        for i in 1..len {
            for j in 0..i {
                if a[i] % a[j] == 0 && dp[i].2 < dp[j].2 + 1 {
                    dp[i].2 = dp[j].2 + 1;
                    dp[i].1 = j;

                    if dp[i].2 > dp[mxn].2 {
                        mxn = i;
                    }
                }
            }
        }
        let mut res = Vec::new();
        let mut now = mxn;
        for i in 0..dp[mxn].2 {
            res.push(nums[now]);
            now = dp[now].1;
        }
        res
    }
}