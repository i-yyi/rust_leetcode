impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut cur: usize;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[cur] = nums[i];
                cur += 1;
            }
        }
        cur
    }
}