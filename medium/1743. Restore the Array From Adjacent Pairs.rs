use std::collections::{BTreeSet, BTreeMap, VecDeque};

impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let mut vals: BTreeMap<i32, BTreeSet<i32>> = BTreeMap::new();
        let a = adjacent_pairs[0][0];
        for x in adjacent_pairs {
            vals.entry(x[0]).or_insert(BTreeSet::new()).insert(x[1]);
            vals.entry(x[1]).or_insert(BTreeSet::new()).insert(x[0]);
        }
        let mut ans: VecDeque<i32> = VecDeque::new();
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(a);
        let mut visited: BTreeSet<i32> = BTreeSet::new();
        visited.insert(a);
        while let Some(x) = queue.pop_front() {
            if ans.is_empty() {
                ans.push_back(x);
            } else {
                let mut inserted = false;
                if let Some(a) = ans.pop_back() {
                    ans.push_back(a);
                    if vals[&a].contains(&x) && !inserted {
                        ans.push_back(x);
                        inserted = true;
                    }
                }
                if let Some(a) = ans.pop_front() {
                    ans.push_front(a);
                    if vals[&a].contains(&x) && !inserted {
                        ans.push_front(x);
                        inserted = true;
                    }
                }
            }
            for &y in vals[&x].iter() {
                if !visited.contains(&y) {
                    queue.push_back(y);
                    visited.insert(y);
                }
            }
        }
        ans.into_iter().collect()
    }
}
