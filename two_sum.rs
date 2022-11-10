use std::collections::BTreeMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = BTreeMap::new();
        for (i, n) in nums.iter().enumerate() {
            map.insert(target - n, i);
        }
        for (i, m) in nums.iter().enumerate() {
            if map.get(&m) != None {
                let j = *map.get(&m).unwrap();
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }
        }
        panic!();
    }
}