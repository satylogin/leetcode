use std::collections::BinaryHeap;
use std::cmp::max;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (heights.len() as i32, heights[0].len() as i32);
        let mut cost: Vec<Vec<i32>> = vec![vec![std::i32::MAX; m as usize]; n as usize];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; m as usize]; n as usize];
        let mut queue: BinaryHeap<(i32, (i32, i32))> = BinaryHeap::new();

        queue.push((0, (0, 0)));
        cost[0][0] = 0;
        while let Some((c, (x, y))) = queue.pop() {
            if visited[x as usize][y as usize] {
                continue;
            }
            visited[x as usize][y as usize] = true;
            for (i, j) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (a, b) = (x + i, y + j);
                if a >= 0 && a < n && b >= 0 && b < m {
                    let (a, b) = (a as usize, b as usize);
                    let _c = max(-c, (heights[x as usize][y as usize] - heights[a][b]).abs());
                    if !visited[a][b] & (_c < cost[a][b]) {
                        cost[a][b] = _c;
                        queue.push((-_c, (a as i32, b as i32)));
                    }
                }
            }
        }

        cost[n as usize - 1][m as usize - 1]
    }
}
