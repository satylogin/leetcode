impl Solution {
    pub fn largest_merge(word1: String, word2: String) -> String {
        let mut word1: Vec<char> = word1.chars().collect();
        let mut word2: Vec<char> = word2.chars().collect();
        let (mut s1, mut s2): (usize, usize) = (0, 0);
        let mut ans = String::new();
        while s1 < word1.len() || s2 < word2.len() {
            let (mut i, mut j) = (s1, s2);
            let mut greater = 0;
            while i < word1.len() && j < word2.len() {
                if word1[i] < word2[j] {
                    greater = 2;
                    break;
                } else if word1[i] > word2[j] {
                    greater = 1;
                    break;
                }
                i += 1;
                j += 1;
            }
            if greater == 0 {
                greater = if (word1.len() - s1) > (word2.len() - s2) {1} else {2};
            }
            if greater == 1 {
                ans.push(word1[s1]);
                s1 += 1;
            } else {
                ans.push(word2[s2]);
                s2 += 1;
            }
        }
        ans
    }
}
