use std::collections::HashMap;

struct FreqStack {
    stacks: HashMap<usize, Vec<i32>>,
    freq: HashMap<i32, usize>,
    greatest_freq: usize
}

impl FreqStack {

    fn new() -> Self {
        FreqStack {
            stacks: HashMap::new(),
            freq: HashMap::new(),
            greatest_freq: 0
        }    
    }
    
    fn push(&mut self, x: i32) {
        *self.freq.entry(x).or_insert(0) += 1;
        self.stacks.entry(self.freq[&x]).or_insert(vec![]).push(x);
        if self.freq[&x] > self.greatest_freq {
            self.greatest_freq = self.freq[&x];
        }
    }
    
    fn pop(&mut self) -> i32 {
        let x = self.stacks.get_mut(&self.greatest_freq).unwrap().pop().unwrap();
        *self.freq.get_mut(&x).unwrap() -= 1;
        if self.stacks[&self.greatest_freq].is_empty() {
            self.greatest_freq -= 1;
        }
        x
    }
}
