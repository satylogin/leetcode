impl Solution {
    fn len(x: i64) -> i32 {
        for i in (0..17).rev() {
            if (x>>i) & 1 == 1 {
                return i + 1;
            }
        }
        0
    }

    pub fn concatenated_binary(n: i32) -> i32 {
        let mut ans: i64 = 0;
        const MOD: i64 = 1e9 as i64 + 7;
        for i in 1..=n as i64 {
            ans = (ans * (1<<Self::len(i)) % MOD + i) % MOD;
        }
        ans as i32
    }
}
