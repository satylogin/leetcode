impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let n: usize = s.len();
        let mut valid_len: Vec<i32> = vec![0; n];
        let mut ans: i32 = 0;
        for i in 1..s.len() {
            if s[i] == '(' {
                continue;
            }
            let idx: i32 = i as i32 - valid_len[i - 1] - 1;
            if idx >= 0 && s[idx as usize] == '(' {
                valid_len[i] = valid_len[i - 1] + 2;
                if idx > 0 {
                    valid_len[i] += valid_len[idx as usize - 1];
                }
            }
            ans = std::cmp::max(ans, valid_len[i]);
        }
        ans
    }
}
