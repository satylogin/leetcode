impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        let mut i: i32 = 0;
        let mut j: i32 = people.len() as i32 - 1;
        let mut ans = 0;
        people.sort();
        while i <= j {
            if i < j && people[i as usize] + people[j as usize] <= limit {
                i += 1;
            }
            j -= 1;
            ans += 1;
        }
        ans
    }
}
