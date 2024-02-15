
use super::Solution;
use std::mem::swap;
impl Solution {
    fn mid_segment<'a>(nums: &'a[i32]) -> &'a[i32] {
        if nums.len() <= 4 {
            return nums;
        }
        
        let lower = ((nums.len() - 1) as f64 / 2.0).floor() as usize - 1;
        let upper = ((nums.len() - 1) as f64 / 2.0).ceil() as usize + 2;
        return &nums[lower..upper];
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut n1, mut n2) = (&nums1[..], &nums2[..]);

        let med = |arr: &[i32]| {
            let mid = (arr.len() - 1) as f64 / 2.0;
            let (left, right) = (mid.floor() as usize, mid.ceil() as usize);
            ((arr[left] + arr[right]) as f64 / 2.0, mid)
        };
        while n1.len() > 2 && n2.len() > 2 {
            let (mut m1, mut m2) = (med(n1), med(n2));

            if m1.0 > m2.0 {
                swap(&mut n1, &mut n2);
                swap(&mut m1, &mut m2);
            }

            let remove = std::cmp::min(m1.1.floor() as usize, n2.len() - m2.1.ceil() as usize - 1);
            n1 = &n1[remove..];
            n2 = &n2[..(n2.len()-remove)];

            println!("{:?} {:?}", m1, m2);
        }
        let mut res = Self::mid_segment(n1).to_vec();
        res.extend(Self::mid_segment(n2));
        res.sort();            

        med(&res).0
    }
}
