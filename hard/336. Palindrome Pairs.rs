use std::collections::HashSet;

#[derive(Default)]
struct Node {
    is_leaf: bool,
    pos: usize,
    childs: Vec<Node>,
    exists: bool,
}

fn char_to_index(c: char) -> usize {
    (c as u8 - 'a' as u8) as usize
}

fn is_palin(word: &Vec<char>, mut start: usize, mut end: usize) -> bool {
    while start < end {
        if word[start] != word[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}

impl Node {
    fn insert(&mut self, word: &Vec<char>, pos: usize, index: usize) {
        self.exists = true;
        if self.childs.len() == 0 {
            self.childs = (0..26).map(|_| Node::default()).collect();
        }

        if index == word.len() {
            self.is_leaf = true;
            self.pos = pos;
        } else {
            let c_index = char_to_index(word[index]);
            self.childs[c_index].insert(word, pos, index + 1);
        }
    }

    // works on non empty strings
    fn palindromic_pair_pos(&self, word: &Vec<char>, index: usize) -> Vec<usize> {
        let mut ans: Vec<usize> = vec![];

        if index != word.len() {
            let c_index = char_to_index(word[index]);
            if self.childs[c_index].exists {
                ans = self.childs[c_index].palindromic_pair_pos(word, index + 1);
            }
        }

        if self.is_leaf && is_palin(word, index, word.len() - 1) {
            ans.push(self.pos);
        }

        ans
    }
}

struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: Node::default(),
        }
    }

    fn insert(&mut self, word: &Vec<char>, pos: usize) {
        if word.len() == 0 {
            self.root.is_leaf = true;
            self.root.pos = pos;
        } else {
            self.root.insert(word, pos, 0);
        }
    }

    fn palindromic_pair_pos(&self, word: &Vec<char>) -> Vec<usize> {
        if word.len() == 0 {
            return vec![];
        }
        self.root.palindromic_pair_pos(word, 0)
    }
}


impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie = Trie::new();
        let mut pairs: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..words.len() {
            trie.insert(&(words[i].chars().rev().collect()), i);
        }
        for i in 0..words.len() {
            let positions: Vec<usize> = trie.palindromic_pair_pos(
                &(words[i].chars().collect())
            );
            positions.into_iter().filter(|pos| *pos != i).for_each(|pos| {
                pairs.insert((i, pos));
            });
        }

        let mut trie = Trie::new();
        for i in 0..words.len() {
            trie.insert(&&(words[i].chars().collect()), i);
        }
        for i in 0..words.len() {
            let positions: Vec<usize> = trie.palindromic_pair_pos(
                &(words[i].chars().rev().collect())
            );
            positions.into_iter().filter(|pos| *pos != i).for_each(|pos| {
                pairs.insert((pos, i));
            });
        }

        pairs.into_iter().map(|(x, y)| vec![x as i32, y as i32]).collect()    
    }
}
