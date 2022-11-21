use std::cmp;

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows <= 1 {
            return s;
        }
        let bytes = s.as_bytes();
        let mut i = 0;
        let round_trip_length = num_rows + num_rows - 2;
        let mut row_index = 0;
        let mut rows = Vec::new();
        for i in 0..num_rows {
            rows.push(Vec::new());
        }
        while i < bytes.len() {
            for j in 0..round_trip_length {
                if i >= bytes.len() {
                    break;
                }
                row_index = (j - cmp::max(0, j - (round_trip_length - j))) as usize;
                rows[row_index].push(bytes[i]);
                i += 1;
            }
        }
        rows.iter().map(|r| String::from_utf8(r.to_vec()).unwrap()).collect::<Vec<_>>().join("")
    }
}