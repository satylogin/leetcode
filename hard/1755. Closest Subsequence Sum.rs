use std::cmp::min;

impl Solution {
    fn get_sum(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        let mut ans: Vec<i32> = Vec::with_capacity(1<<n);
        for x in 0..(1<<n) {
            let mut s: i32 = 0;
            for i in 0..n {
                if (x>>i) & 1 == 1 {
                    s += nums[i];
                }
            }
            ans.push(s);
        }
        ans
    }

    pub fn min_abs_difference(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len() / 2;
        let nums1: Vec<i32> = (0..n).map(|i| nums[i]).collect();
        let nums2: Vec<i32> = (n..nums.len()).map(|i| nums[i]).collect();

        let sums1 = Self::get_sum(nums1);
        let mut sums2 = Self::get_sum(nums2);
        sums2.sort();

        let mut ans: i32 = std::i32::MAX;
        for i in 0..sums1.len() {
            // <= goal
            let (mut start, mut end): (i32, i32) = (0, sums2.len() as i32 - 1);
            while start <= end {
                let mid = (start + end) >> 1;
                let _sum = sums1[i] + sums2[mid as usize];
                if _sum <= goal {
                    ans = min(ans, goal - _sum);
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
            // >= goal
            let (mut start, mut end): (i32, i32) = (0, sums2.len() as i32 - 1);
            while start <= end {
                let mid = (start + end) >> 1;
                let _sum = sums1[i] + sums2[mid as usize];
                if _sum >= goal {
                    ans = min(ans, _sum - goal);
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            }
        }
        ans
    }
}
