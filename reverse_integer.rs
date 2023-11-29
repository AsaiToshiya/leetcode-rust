impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let s = x.to_string();
        let sign_digits = if x < 0 {
            (&s[..1], &s[1..])
        } else {
            ("", &s[0..])
        };
        let reverse_digits = sign_digits.1.chars().rev().collect::<String>();
        let reverse_num_str = sign_digits.0.to_string() + &reverse_digits;
        match reverse_num_str.parse::<i32>() {
            Ok(num) => num,
            Err(_) => 0,
        }
    }
}
