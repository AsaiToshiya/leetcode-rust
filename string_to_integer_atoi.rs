impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let trim_str = s.trim();
        let chars: Vec<char> = trim_str.chars().collect();
        for i in 0..chars.len() {
            if let '+' | '-' | '0'..='9' = chars[i] {
                let mut end: usize = 0;
                for j in i + 1..chars.len() {
                    if let '0'..='9' = chars[j] {
                        end = j;
                    } else {
                        break;
                    }
                }
                let num_str = &trim_str[i..end + 1];
                if let Ok(n) = &num_str.parse::<i32>() {
                    return *n as i32;
                }
                if num_str == "+" {
                    return 0;
                }
                if num_str == "-" {
                    return 0;
                }
                if &trim_str[0..1] == "-" {
                    return i32::MIN;
                } else {
                    return i32::MAX;
                }
            }
            break;
        }

        return 0;
    }
}
