use std::io::{self, Read};

fn read_int() -> u32 {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf);
    return buf.trim().parse::<u32>().expect("Expected a u32, got an error!");
}

fn read_case() -> Vec<i32> {
    let _case_size = read_int();

    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf);
    return buf.trim().split_ascii_whitespace()
        .map(|elem| elem.parse::<i32>().expect("Expected a u32, got an error!"))
        .collect()
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

fn max(s: &[i32]) -> (i32, usize) {
    let mut max: i32 = -1;
    let mut max_loc: usize = 0;
    for (idx, v) in s.iter().enumerate() {
        if *v > max {
            max = *v;
            max_loc = idx;
        }
    }
    return (max, max_loc);
}

fn tree_from_slice(s: &[i32]) -> Option<Box<Node>> {
    return if s.is_empty() {
        None
    } else {
        let (m, m_loc) = max(s);
        let parts = s.split_at(m_loc);
        let new_node: Node = Node {
            value: m,
            left: tree_from_slice(parts.0),
            right: tree_from_slice(&parts.1[1..])
        };
        Some(Box::new(new_node))
    }
}

fn find_depth(q: &i32, current: &Option<Box<Node>>, count: &i32) -> Option<i32> {
    return if let Some(ref node) = current {
        if *q == node.value {
            Some(count.clone())
        } else {
            let l = find_depth(q, &node.left, &(count + 1));
            let r = find_depth(q, &node.right, &(count + 1));
            if l.is_some() {
                l
            } else if r.is_some() {
                r
            } else {
                None
            }
        }
    } else {
        None
    }
}

fn main() {
    let t = read_int();
    for _i in 0..t {
        let case = read_case();
        let tree = tree_from_slice(case.as_slice());
        for v in case.iter() {
            let depth = find_depth(v, &tree, &0);
            print!("{} ", depth.unwrap_or(-1));
        }
    }
}
