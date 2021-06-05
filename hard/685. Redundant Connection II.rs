pub struct Solution;

struct DSU {
    parent: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        DSU {
            parent: (0..n + 1).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);
        if px != py {
            self.parent[px] = py;
            return true;
        }
        false
    }
}

fn culprit(edges: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut indeg = vec![0; edges.len() + 1];
    let mut sink = -1;
    for edge in edges.iter() {
        indeg[edge[1] as usize] += 1;
        if indeg[edge[1] as usize] == 2 {
            sink = edge[1];
        }
    }
    let mut sources = vec![];
    for edge in edges.iter() {
        if edge[1] == sink {
            sources.push(edge[0]);
        }
    }
    let mut dsu = DSU::new(edges.len());
    for edge in edges.iter() {
        if sources.contains(&edge[0]) && edge[1] == sink {
            continue;
        }
        if !dsu.unite(edge[0] as usize, edge[1] as usize) {
            return vec![edge[0], edge[1]];
        }
    }
    if !dsu.unite(sources[0] as usize, sink as usize) {
        vec![sources[0], sink]
    } else {
        vec![sources[1], sink]
    }
}

impl Solution {
    pub fn find_redundant_directed_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        culprit(&edges)
    }
}

#[test]
fn test() {
    let e1 = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    let e2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]];
    let e3 = vec![vec![2, 1], vec![3, 1], vec![4, 2], vec![1, 4]];
    let e4 = vec![vec![4, 2], vec![1, 5], vec![5, 2], vec![5, 3], vec![2, 4]];
    for (ip, exp) in vec![
        (e1, vec![2, 3]),
        (e2, vec![4, 1]),
        (e3, vec![2, 1]),
        (e4, vec![4, 2]),
    ] {
        assert_eq!(Solution::find_redundant_directed_connection(ip), exp);
    }
}
