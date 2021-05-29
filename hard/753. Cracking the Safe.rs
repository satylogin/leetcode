use std::collections::HashMap;

fn nodes(n: usize, k: u8) -> Vec<String> {
    let mut nodes: Vec<String> = vec![String::new()];
    for _ in 0..n {
        let mut new_nodes = vec![];
        for node in nodes {
            for i in 0..k {
                let mut n_node: String = node.clone();
                n_node.push(i as char);
                new_nodes.push(n_node);
            }
        }
        nodes = new_nodes;
    }
    nodes
}

fn adj(nodes: Vec<String>, k: u8) -> HashMap<String, Vec<String>> {
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();

    for node in nodes {
        let base = String::from(&node[1..]);
        for j in 0..k {
            let mut next: String = base.clone();
            next.push(j as char);
            adj.entry(node.clone()).or_insert(vec![]).push(next);
        }
    }

    adj
}

fn euler_tour(mut adj: HashMap<String, Vec<String>>, source: String) -> Vec<String> {
    let mut path: Vec<String> = vec![];
    let mut stack: Vec<String> = vec![];

    let mut cur: String = source;
    loop {
        if adj[&cur].is_empty() {
            path.push(cur);
            if stack.is_empty() {
                break;
            }
            cur = stack.pop().unwrap();
        } else {
            let next: String = adj.get_mut(&cur).unwrap().pop().unwrap();
            stack.push(cur);
            cur = next;
        }
    }

    path.reverse();
    path.pop();
    path
}

impl Solution {
    pub fn crack_safe(n: i32, k: i32) -> String {
        if n == 1 {
            let mut ans: String = String::new();
            for i in 0..k as usize {
                ans.push(('0' as u8 + i as u8) as char);
            }
            return ans;
        }
        let nodes: Vec<String> = nodes(n as usize - 1, k as u8);
        let source: String = nodes[0].clone();
        let adj: HashMap<String, Vec<String>> = adj(nodes, k as u8);
        let path: Vec<String> = euler_tour(adj, source.clone());

        let mut ans: String = source
            .chars()
            .map(|c| (c as u8 + '0' as u8) as char)
            .collect();
        for i in 0..path.len() {
            ans.push((path[i].chars().next_back().unwrap() as u8 + '0' as u8) as char);
        }
        ans
    }
}
