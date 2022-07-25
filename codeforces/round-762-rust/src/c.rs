pub fn main() {
    /*
     * "Wrong addition" (denoted by +?), is a form of addition where you take two numbers of the same length
     * (the shorter number is padded with leading zeros, as necessary) and you simply add each
     * pair of digits together and add it to the leftmost side of the result:
     *
     *   17236
     *   03465
     * -------
     * 1106911
     *
     * For the problem, we're given *one* of the numbers `a` and then another number `s`.
     * Our goal is to find a `b` such that a +? b=s, or output that no such `b` exists.
     *
     * Wrong addition is pairwise, so we only ever need to consider two digits from the sum at a
     * time.
     * The algorithm is basically:
     * 1. Take the rightmost pair of digits (si,bi) from s and b.
     * 2. Is si >= bi? Add si-bi to the leftmost side of the answer. And pop si and bi off their
     *    respective numbers.
     * 3. Is si < bi? Grab the digit to the left of si, sj. If there is no such digit, return a 0.
     *    Normalize leading 0s so that `0x = x` for any single digit `x`. Call this two digit
     *    number s'.
     * 4. Is s' - bi a positive, single digit number? Add it to the answer and pop s' and bi from
     *    their numbers. If s' - bi cannot be found, output -1.
     *
     * Repeat this until `s` has no digits remaining.
     * */

    let t = read!(usize);
    'case: for _ in 0..t {
        let input = read_str!();
        let mut numbers: Vec<Vec<i32>> = input
            .split_whitespace()
            .map(|number| {
                let mut digit_stack: Vec<i32> = Vec::new();

                for c in number.chars() {
                    digit_stack.push(
                        c.to_digit(10)
                            .expect("Could not turn digit into i32")
                            .try_into()
                            .unwrap(),
                    );
                }

                digit_stack
            })
            .collect();

        let (a, s) = match &mut numbers[..] {
            [f, s, ..] => (f, s),
            _ => unreachable!(),
        };

        // Obvious case, you can't add ANYTHING to `a` (using wrong addition)
        // and get LESS digits.
        if a.len() > s.len() {
            println!("-1");
            continue;
        }

        let mut ans_digits = Vec::new();
        while !s.is_empty() {
            // The loop only runs if s is non-empty, so this wont panic.
            let si = s.pop().expect("Should not run out of digits in s!");
            let ai = a.pop().unwrap_or(0);

            if s.len() < a.len() {
                println!("-1");
                continue 'case;
            }

            if si - ai >= 0 {
                ans_digits.push(si - ai);
            } else {
                // See if we can make two digits from s instead
                let sj = s.pop().unwrap_or(0);
                let s_prime = normalize(sj, si);

                if s_prime - ai > 0 && s_prime - ai < 10 {
                    ans_digits.push(s_prime - ai);
                } else {
                    println!("-1");
                    continue 'case;
                }
            }
        }

        ans_digits.reverse();
        let answer = ans_digits
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<String>>()
            .join("");

        // Remember that the output answer could be really large (10^18)
        // And 2^32 (~4 billion) only has ~10 digits, so clearly we need a long.
        let without_leading_zero = answer.parse::<u64>().unwrap().to_string();
        println!("{}", without_leading_zero);
    }
}

fn normalize(left: i32, right: i32) -> i32 {
    match left {
        0 => right,
        _ => (10 * left) + right,
    }
}

/**
Reads a full line of input and parses it into the value of the specified type.
Any type that implements FromStr can be used: https://doc.rust-lang.org/stable/std/str/trait.FromStr.html
```rs
let my_number = read!(u32);
let my_float = read!(f64);
```
*/
#[allow(unused_macros)]
macro_rules! read {
    ($type:ty) => {{
        let mut inner = String::new();
        std::io::stdin()
            .read_line(&mut inner)
            .expect("read! was unable to read from stdin");
        inner
            .trim()
            .parse::<$type>()
            .expect("read! was unable to parse into the desired type")
    }};
}

/**
Reads a full line of input and returns it as a `String`.
*/
#[allow(unused_macros)]
macro_rules! read_str {
    () => {{
        let mut inner = String::new();
        std::io::stdin()
            .read_line(&mut inner)
            .expect("read_str! was unable to read from stdin");
        inner.trim().to_owned()
    }};
}

/**
Reads a line of input, splits it on whitespace, and parses each value into the specified type.
The values are returned in a Vec.
```rs
let mut some_numbers: Vec<i64> = read_vec!(i64);
```
*/
#[allow(unused_macros)]
macro_rules! read_vec {
    ($type:ty) => {{
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>()
    }};
}

/**
Reads a line of input, splits it on whitespace, and parses each value into the specified type.
The values are then matched into a 2-element tuple using slice matching.
```rs
let (a,b) = read_pair!(usize);
```
*/
#[allow(unused_macros)]
macro_rules! read_pair {
    ($type:ty) => {{
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let v = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
        let (a, b) = match &v[..] {
            &[f, s, ..] => (f, s),
            _ => unreachable!(),
        };
        (a, b)
    }};
}

pub(crate) use read;
pub(crate) use read_pair;
pub(crate) use read_str;
pub(crate) use read_vec;
