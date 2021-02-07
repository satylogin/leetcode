use std::cmp::min;

impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut v = vec![a, b, c];
        v.sort();
        let mut ans = v[1] - v[0];
        v[1] -= ans;
        v[2] -= ans;
        let x = min(v[2] / 2, v[0]);
        ans += x + x;
        v[0] -= x;
        v[1] -= x;
        ans += min(v[0], v[1]);
        ans
    }
}
