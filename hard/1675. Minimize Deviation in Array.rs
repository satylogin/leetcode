use std::collections::BinaryHeap;
use std::cmp::min;

impl Solution {
    pub fn minimum_deviation(mut nums: Vec<i32>) -> i32 {
        let mut mn: i32 = std::i32::MAX;
        let mut pq: BinaryHeap<i32> = BinaryHeap::new();
        for mut x in nums {
            if x & 1 == 1 {
                x <<= 1;
            }
            mn = min(mn, x);
            pq.push(x);
        }
        let mut ans: i32 = std::i32::MAX;
        loop {
            let mut x = pq.pop().unwrap();
            ans = min(ans, x - mn);
            if x & 1 == 0 {
                x /= 2;
                mn = min(x, mn);
                pq.push(x);
            } else {
                break;
            }
        }
        ans
    }
}
