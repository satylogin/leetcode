use std::collections::{BTreeMap, BTreeSet};

impl Solution {
    fn freq_vec(word: String) -> Vec<i32> {
        let mut freq: BTreeMap<char, i32> = BTreeMap::new();
        word.chars().for_each(|c| *freq.entry(c).or_insert(0) += 1);
        let mut word: Vec<i32> = freq.iter().map(|(_, x)| *x).collect();
        word.sort();
        word
    }

    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut ans: bool = word1.chars().collect::<BTreeSet<char>>() == word2.chars().collect::<BTreeSet<char>>();
        ans &= Solution::freq_vec(word1) == Solution::freq_vec(word2);
        ans
    }
}
