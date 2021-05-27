use std::cmp::min;

const MOD: i32 = 1e9 as i32 + 7;

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        let m: usize = min(steps, arr_len) as usize;
        
        let mut dp: Vec<Vec<i32>> = vec![vec![0; m + 1]; steps as usize + 1];
        dp[0][0] = 1;
        for i in 1..=steps as usize{
            for j in 0..m {
                dp[i][j] = dp[i - 1][j];
                if j > 0 {
                    dp[i][j] = (dp[i][j] + dp[i - 1][j - 1]) % MOD;
                }
                dp[i][j] = (dp[i][j] + dp[i - 1][j + 1]) % MOD;
            }
        }
        dp[steps as usize][0]
    }
}
