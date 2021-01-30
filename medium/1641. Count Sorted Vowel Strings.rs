impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut dp = [[0; 6]; 60];
        dp[1] = [1, 1, 1, 1, 1, 1];
        for i in 2..=n as usize {
            for j in 1..=5 {
                for k in j..=5 {
                    dp[i][j] += dp[i-1][k];
                }
            }
        }
        let mut ans = 0;
        for i in 1..=5 {
            ans += dp[n as usize][i];
        }
        ans
    }
}
