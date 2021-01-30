impl Solution {
    fn get(mask: usize, i: usize, n: usize) -> i32 {
        if i == n {
            return 1;
        }
        let mut ans: i32 = 0;
        for j in 0..n {
            if (mask >> j) & 1 == 0 {
                if ((j + 1) % (i + 1) == 0) || ((i + 1) % (j + 1) == 0) {
                    ans += Solution::get(mask | (1<<j), i + 1, n);
                }
            }
        }
        ans
    }

    pub fn count_arrangement(n: i32) -> i32 {
        Solution::get(0, 0, n as usize)
    }
}
