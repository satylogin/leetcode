const MOD: i32 = 1e9 as i32 + 7;

fn prefix_function(s: &Vec<usize>) -> Vec<usize> {
    let mut pi: Vec<usize> = vec![0; s.len()];
    for i in 1..s.len() {
        let mut j: usize = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

fn contains(s: Vec<char>, t: Vec<char>) -> bool {
    for i in 0..(t.len() - s.len() + 1) {
        if s[..] == t[i..=(i + s.len() - 1)] {
            return true;
        }
    }
    false
}

struct GoodString {
    n: usize,
    evil: Vec<usize>,
    pi: Vec<usize>,
    s: Vec<usize>,
    mem: Vec<Vec<Vec<i32>>>
}

impl GoodString {
    fn new(n: usize, evil: String, s: String) -> Self {
        let evil: Vec<usize> = evil.chars().map(|c| (c as u8 - 'a' as u8) as usize).collect();
        let pi: Vec<usize> = prefix_function(&evil);
        let s: Vec<usize> = s.chars().map(|c| (c as u8 - 'a' as u8) as usize).collect();
        let mem: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; evil.len()]; 26]; n + 1];

        GoodString {
            n,
            evil,
            pi,
            s,
            mem,
        }
    }

    fn shift(&self, e_pos: usize, n_ch: usize) -> usize {
        if e_pos == 0 {
            return 0;
        }
        let mut j: usize = self.pi[e_pos - 1];
        while j > 0 && self.evil[j] != n_ch {
            j = self.pi[j - 1];
        }
        j + (n_ch == self.evil[j]) as usize
    }

    fn get(&mut self, pos: usize, ch: usize, e_pos: usize) -> i32 {
        if pos == self.n {
            return 1;
        }
        if self.mem[pos][ch][e_pos] != -1 {
            return self.mem[pos][ch][e_pos];
        }
        let mut ans: i32 = 0;
        for n_ch in 0..26 {
            if e_pos < self.evil.len() && n_ch == self.evil[e_pos] {
                if e_pos < self.evil.len() - 1 {
                    ans = (ans + self.get(pos + 1, n_ch, e_pos + 1)) % MOD;
                }
            } else {
                let new_e_pos = self.shift(e_pos, n_ch);
                ans = (ans + self.get(pos + 1, n_ch, new_e_pos)) % MOD;
            }
        }
        self.mem[pos][ch][e_pos] = ans;
        ans
    }

    fn solve(&mut self) -> i32 {
        let mut ans: i32 = 0; 
        let mut e_pos: usize = 0;
        self.mem_reset();

        for i in 0..self.s.len() {
            if e_pos == self.evil.len() {
                break;
            }
            for n_ch in 0..self.s[i] {
                if e_pos < self.evil.len() && n_ch == self.evil[e_pos] {
                    if e_pos < self.evil.len() - 1 {
                        ans = (ans + self.get(i + 1, n_ch, e_pos + 1)) % MOD;
                    }
                } else {
                    let new_e_pos = self.shift(e_pos, n_ch);
                    ans = (ans + self.get(i + 1, n_ch, new_e_pos)) % MOD;
                }
            }
            if self.s[i] == self.evil[e_pos] {
                e_pos += 1;
            } else {
                e_pos = self.shift(e_pos, self.s[i]);
            }
        }

        ans
    }
    
    fn mem_reset(&mut self) {
        for i in 0..=self.n {
            for j in 0..26 {
                for k in 0..self.evil.len() {
                    self.mem[i][j][k] = -1;
                }
            }
        }
    }
}


impl Solution {
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let mut good_s1 = GoodString::new(n as usize, evil.clone(), s1);
        let mut good_s2 = GoodString::new(n as usize, evil.clone(), s2.clone());
        let evil_last: i32 = !contains(evil.chars().collect(), s2.chars().collect()) as i32;
        (good_s2.solve() - good_s1.solve() + evil_last + MOD) % MOD
    }
}
