use std::cmp::min;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let n: usize = stones.len();
        if (n as i32 - 1) % (k - 1) != 0 {
            return -1;
        }
        let mut pref_sum: Vec<i32> = vec![0; stones.len() + 1];
        for i in 0..stones.len() {
            pref_sum[i+1] = pref_sum[i] + stones[i];
        }
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n + 1]; n + 1];
        for start in (1..n + 1).rev() {
            for end in start + k as usize - 1..n + 1 {
                dp[start][end] = std::i32::MAX;
                for x in (start..end).step_by(k as usize - 1) {
                    let cost: i32 = if (end - start) % (k as usize - 1) == 0 {
                        pref_sum[end] - pref_sum[start - 1]
                    } else {
                        0
                    };
                    dp[start][end] = min(dp[start][end], dp[start][x] + dp[x+1][end] + cost);
                }
            }
        }
        dp[1][n]
    }
}
