use std::collections::HashMap;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut char_count = HashMap::new();
        let mut maxn = 0;
        let mut maxn_cnt = 0;
        for c in &tasks {
            let cnt = char_count.entry(*c).or_insert(0);
            *cnt += 1;
            maxn = std::cmp::max(*cnt, maxn)
        }

        for (_, &v) in &char_count {
            if v == maxn {
                maxn_cnt += 1;
            }
        }

        let lhs = (maxn-1) * n;
        let rhs = tasks.len() as i32 - maxn - maxn_cnt + 1;
        if lhs > rhs {
            (maxn-1) * n + maxn_cnt - 1
        } else {
            tasks.len() as i32 + maxn_cnt - 1
        }

    }   
}