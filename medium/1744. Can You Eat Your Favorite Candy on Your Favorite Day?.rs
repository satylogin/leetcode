impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n: usize = candies_count.len();
        let mut pref_sum: Vec<i64> = vec![0; n];
        pref_sum[0] = candies_count[0] as i64;
        for i in 1..n {
            pref_sum[i] = pref_sum[i-1] + candies_count[i] as i64;
        }
        let mut ans: Vec<bool> = vec![];
        for query in queries {
            let typ = query[0] as i64;
            let day = query[1] as i64;
            let cap = query[2] as i64;
            let to_eat = if typ == 0 {0} else {pref_sum[typ as usize - 1]};
            ans.push((day + 1) * cap > to_eat && (day + 1) <= (pref_sum[typ as usize]));
        }
        ans
    }
}
