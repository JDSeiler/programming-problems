use std::io::{self, Read};

fn read_int() -> u32 {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf);
    return buf.trim().parse::<u32>().expect("Expected a u32, got an error!");
}

fn read_case() -> Vec<u32> {
    let _case_size = read_int();

    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf);
    return buf.trim().split_ascii_whitespace()
        .map(|elem| elem.parse::<u32>().expect("Expected a u32, got an error!"))
        .collect()
}

fn solve_case(case: &Vec<u32>) -> u32 {
    let mut count: u32 = 0;
    case.windows(2).for_each(|slice| {
        if let [l, r] = slice {
            let big = *l.max(r);
            let mut small = *l.min(r);

            while big as f64 / small as f64 > 2.0 {
                small = small * 2;
                count = count + 1;
            }
        }
    });
    return count;
}

fn main() {
    let t = read_int();
    for _i in 0..t {
        let case = read_case();
        let ans = solve_case(&case);
        println!("{}", ans);
    }
}
