use std::collections::BTreeMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = BTreeMap::new();
        for j in 0..nums.len() {
            if let Some(i) = map.get(&nums[j]) {
                return vec![*i as i32, j as i32];
            }
            map.insert(target - nums[j], j);
        }
        panic!();
    }
}