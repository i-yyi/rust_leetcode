impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort();
        let res: Vec<Vec<i32>> = nums.chunks(3).map(|chunk| chunk.to_vec()).collect();
        if res.iter().any(|sub_vec| sub_vec.len() == 3 && sub_vec[2] - sub_vec[0] > k) { vec![] } else {res}
    }
}
