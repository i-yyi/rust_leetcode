impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        for i in 2..nums.len() {
            if nums[i] == nums[i-2] {
                nums.remove(i)
            }
            i -= 1
        }
        nums.len() as i32
    }
}