use super::Solution;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut data = mat.iter().enumerate().collect::<Vec<_>>();
        data.sort_by_cached_key(|f| f.1.iter().sum::<i32>());
        data.iter().take(k as usize).map(|f| f.0 as i32).collect::<Vec<_>>()
    }
}