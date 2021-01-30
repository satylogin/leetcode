use std::cmp::min;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let n: usize = nums.len();
        let mut pref_sum: Vec<i32> = vec![0; n + 1];
        (0..n).for_each(|i| pref_sum[i + 1] = nums[i] + pref_sum[i]);
        let mut ans: usize = n + 1;
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i <= n {
            while j <= n && pref_sum[i] + pref_sum[n] - pref_sum[j] >= x {
                if pref_sum[i] + pref_sum[n] - pref_sum[j] == x {
                    ans = min(ans, i + n - j);
                }
                j += 1;
            }
            i += 1;
        }
        if ans == n + 1 {-1} else {ans as i32}
    }
}
