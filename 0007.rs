#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        fn reverse_integer(mut num: i32) -> Option<i32> {
            let mut rev: i32 = 0;
            while num != 0 {
                let digit = num % 10;
                rev = rev.checked_mul(10)?.checked_add(digit)?;
                num /= 10;
            }
            Some(rev)
        }
        reverse_integer(x).unwrap_or(0)
    }
}
