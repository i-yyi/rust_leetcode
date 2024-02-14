impl Solution {
    pub fn merge(nums1 : &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut index1: usize = m - 1;
        let mut index2 = n - 1;
        let mut insert_cursor = m + n - 1;
        while index1 > 0 && index2 > 0 {
            if nums1[index1] > nums2[index2] {
                nums1[insert_cursor] = nums1[index1];
                index1 -= 1;
            } else {
                nums1[insert_cursor] = nums2[index2];
                index2 -= 1;
            }
            insert_cursor -= 1;
        }
        while index2 >= 0 {
            nums1[insert_cursor] = nums2[index2];
            index2 -= 1;
            insert_cursor -= 1;
        }
    }
}