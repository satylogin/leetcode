impl Solution {
    #[inline]
    fn quad_sum(n: i64) -> i64 {
        n * (n + 1) * (2 * n + 1) / 6
    }
    
    fn get_box(mut h: i64) -> i64 {
        if h & 1 == 1 {
            Solution::quad_sum(h) - 4 * Solution::quad_sum(h / 2)
        } else {
            4 * Solution::quad_sum(h / 2)
        }
    }

    pub fn minimum_boxes(n: i32) -> i32 {
        let (mut start, mut end, mut h): (i64, i64, i64) = (0, 1000000, 0);
        while start <= end {
            let mid = (start + end) >> 1;
            if Solution::get_box(mid) <= n as i64 {
                h = mid;
                start = mid + 1;
            } else {
                end = mid - 1;
            }
        }
        let mut ans: i64 = h * (h + 1) / 2;
        let mut boxes: i64 = Solution::get_box(h);
        let mut inc: i64 = 1;
        while boxes < n as i64 {
            ans += 1;
            boxes += inc;
            inc += 1;
        }
        ans as i32
    }
}
