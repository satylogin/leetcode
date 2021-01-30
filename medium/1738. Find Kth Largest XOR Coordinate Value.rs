impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n: usize = matrix.len();
        let m: usize = matrix[0].len();
        let mut coord: Vec<Vec<i32>> = vec![vec![0; m]; n];
        coord[0][0] = matrix[0][0];
        let mut ans: Vec<i32> = Vec::with_capacity(n * m);
        ans.push(matrix[0][0]);
        for i in 0..n {
            for j in 0..m {
                if i == 0 && j == 0 {
                    continue;
                } else if i == 0 {
                    coord[i][j] = coord[i][j-1] ^ matrix[i][j];
                } else if j == 0 {
                    coord[i][j] = coord[i-1][j] ^ matrix[i][j];
                } else {
                    coord[i][j] = coord[i-1][j] ^ coord[i][j-1] ^ coord[i-1][j-1] ^ matrix[i][j];
                }
                ans.push(coord[i][j]);
            }
        }
        ans.sort();
        ans.reverse();
        ans[k as usize - 1]
    }
}
