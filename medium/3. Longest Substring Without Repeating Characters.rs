use std::collections::BTreeMap;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut freq: BTreeMap<char, i32> = BTreeMap::new();
        let mut ans: usize = 0;
        while j < s.len() {
            let c = arr[j];
            *freq.entry(c).or_insert(0) += 1;
            while freq[&c] > 1 {
                *freq.get_mut(&arr[i]).unwrap() -= 1;
                i += 1;
            }
            ans = max(ans, j - i + 1);
            j += 1;
        }
        ans as i32
    }
}
