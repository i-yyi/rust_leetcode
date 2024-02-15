use super::Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let total: i32 = nums.iter().sum();
        let n = nums.len();
        let mut current = 0;
        let mut left = 0;
        let mut maxi = -1;

        for right in 0..n {
            current += nums[right];
            while current > total - x && left <= right {
                current -= nums[left];
                left += 1;
            }
            if current == total - x {
                maxi = maxi.max((right - left + 1) as i32);
            }
        }

        if maxi != -1 { (n as i32) - maxi } else { -1 }
    }
}