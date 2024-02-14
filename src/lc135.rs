use std::cmp::max;
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut l = vec![0; ratings.len()];
        let mut r = vec![0; ratings.len()];
        let mut ans = 0;
        for i in 0..ratings.len() {
            if i < 1 || ratings[i] <= ratings[i-1] {
                l[i] = 1;
            } else {
                l[i] = l[i-1] + 1;
            }
        }
        for i in (0..ratings.len()).rev() {
            if i == ratings.len()-1 || ratings[i] <= ratings[i+1] {
                r[i] = 1;
            } else {
                r[i] = r[i+1] + 1;
            }
            ans += max(l[i], r[i]);
        }
        println!(" l : {:?}, r : {:?}", l, r);
        ans
    }
}