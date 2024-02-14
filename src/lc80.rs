impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        
        let mut l = 2, mut r = 2;
        if nums.len() <= 2 {
            return nums.len()
        }
        while r < nums.len() {
            if nums[r] != nums[l-2] {
                nums[l] = nums[r];
                l += 1;
                r += 1;
            } else {
                r += 1;
            }
        }
        l as i32
    }
}