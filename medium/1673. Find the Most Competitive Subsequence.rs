use std::collections::BTreeSet;

impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k: usize = k as usize;
        let mut elem: BTreeSet<(i32, usize)> = BTreeSet::new();
        for i in 0..(nums.len() - k) {
            elem.insert((nums[i], i));
        }
        let mut ans: Vec<i32> = Vec::with_capacity(k);
        let (mut i, mut j): (usize, usize) = (0, nums.len() - k);
        for _ in 0..k {
            elem.insert((nums[j], j));
            let x: (i32, usize) = elem.iter().next().unwrap().clone();
            ans.push(x.0);
            while i <= x.1 {
                elem.remove(&(nums[i], i));
                i += 1;
            }
            j += 1;
        }
        ans
    }
}
