use super::Solution;
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let pos : Vec<_> = nums.iter().filter(|&&x| x > 0).collect();
        let neg : Vec<_> = nums.iter().filter(|&&x| x < 0).collect();
        let res = pos.iter().zip(neg.iter()).flat_map(|(&a, &b)| vec![a, b]).cloned().collect();
        res
    }
}
