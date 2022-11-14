use std::cmp;
use std::collections::BTreeMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut len = 0;
        let mut map = BTreeMap::new();
        let mut start_index = 0;
        for i in 0..bytes.len() {
            if let Some(j) = map.get(&bytes[i]) {
                if *j >= start_index {
                    len = cmp::max(len, i - start_index);
                    start_index = *j + 1;
                }
            }
            map.insert(&bytes[i], i);
        }
        cmp::max(len, bytes.len() - start_index) as i32
    }
}