
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort_unstable();
        let sum: Vec<_> = nums
            .iter()
            .scan(0, |acc: &mut i64, &num| {
                *acc += num as i64;
                Some(*acc)
            })
            .collect();

        sum.iter()
            .enumerate()
            .rev()
            .skip(1)
            .find(|&(i, &v)| v > nums[i + 1] as i64)
            .map_or(-1, |(i, _)| sum[i] + nums[i + 1] as i64)
    }
}
