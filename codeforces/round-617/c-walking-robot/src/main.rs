use std::io::{self, Result};
use std::collections::HashMap;

// X, Y
type RobotLocation = (i64, i64);
type Timestamp = usize;

// Start Vertex, End Vertex, Length (in edges)
type Cycle = (usize, usize, usize);

fn read_int() -> i64 {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf);
    return buf.trim().parse::<i64>().expect("Expected an i64, got an error!");
}

fn read_case() -> String {
    // Technically usize could be much smaller than an i64
    // For this problem it wont be an issue because the bounds are
    // small enough. But you should be careful if this int could be large
    let path_length = read_int() as usize;
    let mut path = String::with_capacity(path_length);

    let stdin = io::stdin();
    stdin.read_line(&mut path);
    return path;
}

fn solve_case() {
    let raw_path = read_case();
    let mut path: HashMap<RobotLocation, Timestamp> = HashMap::new();

    let mut cycles: Vec<Cycle> = Vec::new();

    let mut last_pos: RobotLocation = (0, 0);
    path.insert(last_pos, 0);

    for (path_idx, chr) in raw_path.trim().chars().enumerate() {
        // The thing we're iterating over is the set of EDGES. But we want the index
        // to be the VERTEX number. So adjust by one.
        let current_time = path_idx + 1;

        let (x, y) = last_pos;
        let curr_pos = match chr {
            'U' => {
                (x+1, y)
            },
            'D' => {
                (x-1, y)
            },
            'L' => {
                (x, y-1)
            },
            'R' => {
                (x, y+1)
            },
            _ => panic!("Invalid path char: {}", chr)
        };

        if let Some(last_occurrence) = path.get(&curr_pos) {
            let cycle = (*last_occurrence, current_time, current_time - *last_occurrence);
            cycles.push(cycle);
        }

        path.entry(curr_pos)
            .and_modify(|ts| { *ts = current_time })
            .or_insert(current_time);

        last_pos = curr_pos;
    }

    // Sort cycles by increasing order of length
    cycles.sort_by_key(|cycle| cycle.2);

    if cycles.is_empty() {
        println!("-1");
    } else {
        let shortest_cycle = cycles[0];
        // Adjust what we print because the cycles are encoded as vertices, while the output
        // is concerned with edges!!! If a cycle occurs between vertices 2, 3, 4 (0 indexed)
        // The edges to remove are edges 2 and 3. So to adjust for 1 indexing and such:
        // we add one to the start VERTEX of the path, and then print the end VERTEX of the path
        // which is the same as adding 1 to the ending EDGE of the path.
        println!("{} {}", shortest_cycle.0 + 1, shortest_cycle.1);
    }
}

fn main() {
    let t = read_int();
    for _i in 0..t {
        solve_case();
    }
}
