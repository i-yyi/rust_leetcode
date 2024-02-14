use std::collections::HashSet;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut cnt = vec![0; 26];
        let mut vis = HashSet::new();
        let mut ans = 0;
        for ch in s.bytes() {
            cnt[ ch as usize - 'a' as usize ] += 1;
        }
        for i in 0..26 {
            while cnt[i] && vis.Contains(cnt[i]) {
                cnt[i] -= 1;
                ans += 1;
            }
            vis.insert(cnt[i]);
        }
    }
}