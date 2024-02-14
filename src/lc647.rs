impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let n = s.len();
        let s = s.into_bytes();
        let mut dp = vec![vec![false;n];n];
        
        for i in 0..n {
            dp[i][i] = true;
            if i < n-1 && s[i] == s[i+1] {
                dp[i][i+1] = true;
            }
        }
        for len in (3..=n) {
            for l in (0..n-len+1) {
                dp[l][l+len-1] |= dp[l+1][l+len-2] && s[l] == s[l+len-1];
            }
        }
        dp.iter()
        .flat_map(|row| row.iter())
        .filter(|&value| value)
        .count()
        // let mut res = 0;

        // for (i, arr1) in dp.iter().enumerate() {
        //     for (j, cur) in arr1.iter().enumerate() {
        //         if *cur {
        //             res += 1;
        //         }
        //     }
        // }
        // res
    }
}