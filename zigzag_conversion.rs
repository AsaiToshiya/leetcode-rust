use std::cmp;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let mut i = 0;
        let mut j = 0;
        let round_trip_length = num_rows + num_rows - 2;
        let mut row_index = 0;
        let mut rows = Vec::new();
        for i in 0..num_rows {
            rows.push("".to_string());
        }
        while i < s.len() {
            j = i as i32 % round_trip_length;
            row_index = (j - cmp::max(0, j - (round_trip_length - j))) as usize;
            rows[row_index].push_str(&s[i..i + 1]);
            i += 1;
        }
        rows.join("")
    }
}