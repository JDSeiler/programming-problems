use std::io::{self};
use std::collections::VecDeque;

fn read_int() -> i64 {
    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf).expect("Read failed!");
    parse_int(&buf)
}

fn read_case() -> VecDeque<i64> {
    let _case_size = read_int();

    let mut buf = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buf).expect("Read failed!");
    return buf.trim().split_ascii_whitespace()
        .map(|elem| parse_int(elem))
        .collect()
}

fn parse_int(s: &str) -> i64 {
    return s.trim().parse::<i64>().expect("BIG PROBLEM: parse_int croaked!");
}

type GameResult = (i64, i64, i64);
fn simulate(candy: &mut VecDeque<i64>) -> GameResult {
    let mut moves: i64 = 0;
    let mut alice: i64 = 0;
    let mut bob: i64 = 0;

    let mut last_move: i64 = 0;
    while !candy.is_empty() {
        // Alice's Turn
        if let Some(amount) = eat_from(Side::Front, candy, last_move) {
            alice += amount;
            last_move = amount;
            moves += 1;
        }

        // Bob's Turn
        if let Some(amount) = eat_from(Side::Back, candy, last_move) {
            bob += amount;
            last_move = amount;
            moves += 1;
        }
    }

    (moves, alice, bob)
}

type AmountEaten = Option<i64>;
enum Side {
    Front,
    Back
}
fn eat_from(side: Side, candy: &mut VecDeque<i64>, last_move: i64) -> AmountEaten {
    let mut amount_eaten: i64 = 0;
    while amount_eaten <= last_move {
        let next = match side {
            Side::Front => candy.pop_front(),
            Side::Back => candy.pop_back()
        };

        match next {
            Some(candy) => amount_eaten += candy,
            None => break
        }
    }

    if amount_eaten == 0 {
        None
    } else {
        Some(amount_eaten)
    }
}

fn main() {
    let t = read_int();
    for _i in 0..t {
        let mut case = read_case();
        let ans: GameResult = simulate(&mut case);
        println!("{} {} {}", ans.0, ans.1, ans.2);
    }
}
