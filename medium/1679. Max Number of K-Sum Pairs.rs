use std::collections::btree_map::BTreeMap;
use std::cmp::min;

impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut freq: BTreeMap<i32, i32> = BTreeMap::new();
        for x in nums {
            *freq.entry(x).or_insert(0) += 1;
        }
        let mut ans: i32 = 0;
        for x in freq.keys().cloned().collect::<Vec<i32>>() {
            if x == k - x {
                ans += freq.get(&x).unwrap() / 2;
            } else {
                let y = min(
                    freq.get(&x).cloned().unwrap(),
                    freq.get(&(k - x)).cloned().unwrap_or(0)
                );
                *freq.entry(x).or_insert(0) -= y;
                *freq.entry(k - x).or_insert(0) -= y;
                ans += y;
            }
        }
        ans
    }
}
