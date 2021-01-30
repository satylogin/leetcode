use std::collections::BTreeMap;
use std::cmp::min;

impl Solution {
    fn cost3(s: &String) -> usize {
        let mut freq: BTreeMap<char, usize> = BTreeMap::new();
        for c in s.chars() {
            *freq.entry(c).or_insert(0) += 1;
        }
        let mut ans: usize = std::usize::MAX;
        for (_, x) in freq {
            ans = min(ans, s.len() - x);
        }
        ans
    }

    fn cost1(a: &String, b: &String) -> usize {
        let mut freqa: [usize; 26] = [0; 26];
        let mut freqb: [usize; 26] = [0; 26];
        for c in a.chars() {
            freqa[(c as u8 - 'a' as u8)  as usize] += 1;
        }
        for c in b.chars() {
            freqb[(c as u8 - 'a' as u8) as usize] += 1;
        }
        let mut ans: usize = std::usize::MAX;
        for i in 0..25 {
            let mut cost: usize = 0;
            for j in i+1..26 {
                cost += freqa[j];
            }
            for j in 0..=i {
                cost += freqb[j];
            }
            ans = min(ans, cost);
        }
        ans
    }

    pub fn min_characters(a: String, b: String) -> i32 {
        let mut cost = Solution::cost3(&a) + Solution::cost3(&b);
        cost = min(cost, Solution::cost1(&a, &b));
        cost = min(cost, Solution::cost1(&b, &a));
        cost as i32
    }
}
