use std::cmp::max;

impl Solution {
    pub fn max_value(mut events: Vec<Vec<i32>>, k: i32) -> i32 {
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut dp: Vec<Vec<i32>> = vec![vec![0; k as usize + 1]; events.len()];
        dp[0][1] = events[0][2];
        for i in 1..events.len() {
            let _start = events[i][0];
            let _val = events[i][2];
            let (mut start, mut end, mut idx): (i32, i32, i32) = (0, i as i32 - 1, -1);
            while start <= end {
                let mid = (start + end) >> 1;
                if events[mid as usize][1] < _start {
                    idx = mid;
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
            (1..=k as usize).for_each(|j| dp[i][j] = dp[i-1][j]);
            if idx == -1 {
                dp[i][1] = max(dp[i][1], _val);
            } else {
                for j in 1..=k as usize {
                    dp[i][j] = max(dp[i][j], dp[idx as usize][j-1] + _val);    
                }
            }
        }
        (1..=k as usize).map(|i| dp[events.len() - 1][i]).max().unwrap()
    }
}
