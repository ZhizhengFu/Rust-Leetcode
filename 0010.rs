#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();
        let match_char =
            |i: usize, j: usize| -> bool { i > 0 && (p[j - 1] == '.' || s[i - 1] == p[j - 1]) };
        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..=s.len() {
            for j in 1..=p.len() {
                dp[i][j] = if p[j - 1] == '*' {
                    match_char(i, j - 1) && dp[i - 1][j] || dp[i][j - 2]
                } else {
                    match_char(i, j) && dp[i - 1][j - 1]
                };
            }
        }
        dp[s.len()][p.len()]
    }
}
