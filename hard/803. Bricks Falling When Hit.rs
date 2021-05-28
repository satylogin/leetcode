struct DSU {
    parent: Vec<usize>,
    is_stable: Vec<bool>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize, m: usize) -> Self {
        DSU {
            parent: (0..n*m).collect(),
            is_stable: vec![false; n * m],
            size: vec![1; n * m],
        }
    }
    
    fn find(&self, x: usize) -> usize {
        if x == self.parent[x] {
            x
        } else {
            self.find(self.parent[x])
        }
    }
    
    fn unite(&mut self, x: usize, y: usize) {
        let mut px = self.find(x);
        let mut py = self.find(y);
        if px == py {
            return;
        }
        if self.size[px] < self.size[py] {
            std::mem::swap(&mut px, &mut py);
        }
        self.parent[py] = px;
        self.size[px] += self.size[py];
        self.is_stable[px] |= self.is_stable[py];
    }
}

fn id(x: i32, y: i32, m: usize) -> usize {
    x as usize * m + y as usize
}

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let n: usize = grid.len();
        let m: usize = grid[0].len();
        
        let mut end = grid.clone();
        for i in 0..hits.len() {
            end[hits[i][0] as usize][hits[i][1] as usize] = 0;   
        }
        
        let mut dsu = DSU::new(n, m);
        for i in 0..n as i32 {
            for j in 0..m as i32 {
                if end[i as usize][j as usize] == 0 {
                    continue;
                }
                if i == 0 {
                    dsu.is_stable[id(i, j, m)] = true;
                }
                for (x, y) in vec![(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                        if end[x as usize][y as usize] == 1 {
                            dsu.unite(id(x, y, m), id(i, j, m));
                        }
                    }
                }
            }
        }

        let mut falls: Vec<usize> = vec![];
        for i in (0..hits.len()).rev() {
            let x = hits[i][0];
            let y = hits[i][1];
            if grid[x as usize][y as usize] == 0 {
                falls.push(0);
                continue;
            }

            let mut state = vec![];

            if x == 0 {
                dsu.is_stable[id(x, y, m)] = true;
            }
            
            for (a, b) in vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if a >= 0 && a < n as i32 && b >= 0 && b < m as i32 {
                    if end[a as usize][b as usize] == 0 {
                        continue;
                    }

                    let ps = dsu.find(id(x, y, m));
                    let pn = dsu.find(id(a, b, m));

                    if ps == pn {
                        continue;
                    }
                    
                    let sz = dsu.size[pn];
                    let st = dsu.is_stable[pn];

                    state.push((sz, st));
                    dsu.unite(ps, pn);
                }
            }
            end[x as usize][y as usize] = 1;

            let mut cnt_falls = 0;
            if dsu.is_stable[dsu.find(id(x, y, m))] {
                for (sz, st) in state {
                    if !st {
                        cnt_falls += sz;
                    }
                }
            }
            falls.push(cnt_falls);
        }
        falls.reverse();
        falls.into_iter().map(|x| x as i32).collect()
    }
}
