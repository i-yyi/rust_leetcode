use std::collections::HashMap;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        let mut group = HashMap::new();
        for (i, val) in group_sizes.iter().enumerate() {
            let mut v = group.entry(val).or_insert(Vec::new());
            v.push(i);
        }
        println!("{:?}", group);
        let mut ans = Vec::new();
        for (k, v) in group.iter() {
            let mut now = Vec::new();
            for (i, cur) in v.iter().enumerate() {
                if i as i32 % **k == 0 {
                    ans.push(now);
                    now = Vec::new();
                }
                now.push(*cur as i32);
            }
            if now.len() != 0 {
                ans.push(now);
            }
        }
        ans
    }
}