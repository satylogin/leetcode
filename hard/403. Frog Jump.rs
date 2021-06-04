impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut moves: Vec<Vec<i32>> = vec![vec![]; stones.len()];
        moves[0].push(0);
        for i in 0..stones.len() {
            moves[i].sort();
            moves[i].dedup();
            let mut k: usize = 0;
            for j in i + 1..stones.len() {
                while k < moves[i].len() && moves[i][k] + 1 < stones[j] - stones[i] {
                    k += 1;
                }
                if k < moves[i].len() && (moves[i][k] - (stones[j] - stones[i])).abs() <= 1 {
                    moves[j].push(stones[j] - stones[i]);
                }
            }
        }
        moves[stones.len() - 1].len() > 0
    }
}
