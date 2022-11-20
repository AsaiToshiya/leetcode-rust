impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let mut start = 0;
        let mut end = 0;
        let mut a: i16 = 0;
        let mut b: usize = 0;
        for i in 0..bytes.len() {
            for j in 0..2 {
                a = i as i16;
                b = i + j;
                while 0 <= a && b < bytes.len() {
                    if bytes[a as usize] != bytes[b] {
                        break;
                    }
                    if end - start < b - a as usize {
                        start = a as usize;
                        end = b;
                    }
                    a -= 1;
                    b += 1;
                }
            }
        }
        String::from_utf8((&bytes[start..end + 1]).to_vec()).unwrap()
    }
}