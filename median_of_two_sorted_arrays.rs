impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums = nums1.to_vec();
        nums.extend_from_slice(&nums2);
        nums.sort();
        let mid = (nums.len() - 1) as f64 / 2.0;
        let i = mid as usize;
        if mid == mid.floor() {
            return nums[i] as f64;
        } else {
            return (nums[i] + nums[i + 1]) as f64 / 2.0;
        }
    }
}