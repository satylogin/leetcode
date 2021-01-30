use std::cmp::min;

impl Solution {
    pub fn get_smallest_string(n: i32, mut k: i32) -> String {
        let mut num: Vec<u8> = vec!['a' as u8; n as usize];
        k -= n;
        for i in (0..num.len()).rev() {
            let x = min(25, k);
            if x == 0 {
                break;
            }
            k -= x;
            num[i] = num[i] + x as u8;
        }
        num.iter().map(|&x| x as char).collect::<String>()
    }
}
