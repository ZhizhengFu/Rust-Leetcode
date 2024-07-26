#![allow(dead_code)]
struct Solution;
impl Solution {
    fn expand(s: &[u8], i: usize) -> (usize, usize) {
        let n = s.len();
        let (mut i1, mut j1, mut d1) = (i, i + 1, 0);
        while i1 < n && j1 < n && s[i1] == s[j1] {
            i1 -= 1;
            j1 += 1;
            d1 += 2;
        }
        let (mut i2, mut j2, mut d2) = (i - 1, i + 1, 1);
        while i2 < n && j2 < n && s[i2] == s[j2] {
            i2 -= 1;
            j2 += 1;
            d2 += 2;
        }
        if d1 > d2 {
            (d1, i1 + 1)
        } else {
            (d2, i2 + 1)
        }
    }
    pub fn longest_palindrome(s: String) -> String {
        let s = s.as_bytes();
        let (mut d_res, mut p_res) = (0, 0);
        for i in 0..s.len() {
            let (d, p) = Self::expand(&s, i);
            if d > d_res {
                (d_res, p_res) = (d, p);
            }
        }
        String::from_utf8_lossy(&s[p_res..p_res + d_res]).into_owned()
    }
}
