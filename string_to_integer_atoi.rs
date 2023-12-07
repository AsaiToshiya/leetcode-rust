impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        for i in 0..chars.len() {
            if chars[i] == ' ' {
                continue;
            }

            if let '+' | '-' | '0'..='9' = chars[i] {
            } else {
                break;
            }

            let mut end: usize = i;
            for j in i + 1..chars.len() {
                if let '0'..='9' = chars[j] {
                    end = j;
                } else {
                    break;
                }
            }
            let num_str = &s[i..end + 1];
            if let Ok(n) = &num_str.parse::<i32>() {
                return *n as i32;
            }
            if num_str == "+" {
                return 0;
            }
            if num_str == "-" {
                return 0;
            }
            if &s[i..i + 1] == "-" {
                return i32::MIN;
            } else {
                return i32::MAX;
            }
        }

        return 0;
    }
}
