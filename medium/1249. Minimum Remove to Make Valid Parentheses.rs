use std::collections::vec_deque::VecDeque;
use std::collections::HashSet;

impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let s: Vec<char> = s.chars().collect();
        let mut to_remove: HashSet<usize> = HashSet::new();
        let mut start: VecDeque<usize> = VecDeque::new();
        for i in 0..s.len() {
            if s[i] == '(' {
                start.push_back(i);
            } else if s[i] == ')' {
                if start.is_empty() {
                    to_remove.insert(i);
                }
                start.pop_front();
            }
        }
        to_remove.extend(start.into_iter());
        (0..s.len()).filter(|i| !to_remove.contains(i)).map(|i| s[i]).collect()
    }
}
