use std::collections::HashMap;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut num_count = HashMap::new();
        let mut res = Vec::new();
        for num in nums {
            let cnt = num_count.entry(num).or_insert(0);
            *cnt += 1;
            if *cnt == 2 {
                res.push(num);
            }
        }
        res
    }
}