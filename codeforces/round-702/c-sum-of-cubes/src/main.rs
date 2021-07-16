use std::collections::BTreeSet;
use std::io::{self, Read};

fn read_int() -> i128 {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf);
    return buf.trim().parse::<i128>().expect("Expected a u64, got an error!");
}

fn pre_compute_cubes(set: &mut BTreeSet<i128>) {
    for i in 1..10_000 {
        let cube: i128 = i*i*i;
        set.insert(cube);
    }
}

fn main() {
    let mut cubes: BTreeSet<i128> = BTreeSet::new();
    // linear over 10,000 - only done once, no problem
    pre_compute_cubes(&mut cubes);

    let t = read_int();
    // 100 test cases
    for _i in 0..t {
        let x = read_int();
        let mut found_match: bool = false;
        // Linear over 10,000 - no problem
        for a in cubes.iter() {
            let b: i128 = x - a;
            // log(10,000) - no problem
            if cubes.contains(&b) {
                println!("YES");
                found_match = true;
                break;
            }
        }
        if !found_match {
            println!("NO");
        }
    }
}
