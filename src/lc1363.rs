use super::Solution;
impl Solution {
    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut threes: Vec<i32> = digits.iter().cloned().filter(|&x| x % 3 == 0).collect();
        let digits: Vec<i32> = digits.into_iter().filter(|&x| x % 3 != 0).collect();
        let n = digits.len();
        let mut dp = vec![vec![-1; 3]; n+1];
        let mut from = vec![vec![(n, 0); 3]; n+1];
        for i in 0..n {
            dp[i][digits[i] as usize % 3] = 1;
            for j in 0..i {
                for k in 0..3 {
                    let to = (digits[i] as usize + k) % 3;
                    if dp[j][k] == -1 {
                        continue;
                    }
                    if dp[i][to] < dp[j][k] + 1 || (dp[i][to] == dp[j][k] + 1 && digits[from[i][to].0] < digits[j]){
                        dp[i][to] = dp[j][k] + 1;
                        from[i][to] = (j, k);
                    }
                }
            }
        }
        
        let mut maxn = (0..n).max_by_key(|&i| (dp[i][0], digits[i])).unwrap_or(0);
        if dp[maxn][0] == -1 {
            if threes.len() == 0 {
                return "".to_string();
            }
            maxn = n;
        }
        println!("{:?}\n{:?}", dp, digits);
        let mut res = Vec::with_capacity((dp[maxn][0] + threes.len() as i32) as usize);
        let mut cur = 0;
        while maxn < n { 
            let last = from[maxn][cur];
            res.push(digits[maxn]);
            maxn = last.0;
            cur = last.1;
        }
        res.extend(threes);
        res.sort();
        if res.last() == Some(&0) {
            return "0".to_string();
        }
        let res = res.into_iter().rev().map(|i| i.to_string()).collect::<Vec<String>>().join("");
        res.to_string()
    }
}
