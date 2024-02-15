use super::Solution;

impl Solution {
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let index = nums[i].abs() as usize;
            if nums[index] < 0 {
                return index as i32;
            }
            nums[index] = -nums[index];
        }
        0
    }
}