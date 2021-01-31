impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let n: usize = s.len();
        let s: Vec<char> = s.chars().collect();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];
        for i in 0..n {
            dp[i][i] = 1;
            for j in (0..i).rev() {
                if s[i] == s[j] && (j + 1 == i || (dp[i - 1][j + 1] == 1)) {
                    dp[i][j] = 1;
                }
            }
        }
        let mut ans = false;
        for i in 1..n-1 {
            for j in i..n-1 {
                ans |= (dp[i-1][0] == 1) && (dp[j][i] == 1) && (dp[n-1][j+1] == 1);
            }
        }
        ans
    }
}
