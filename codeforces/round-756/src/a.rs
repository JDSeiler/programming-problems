
pub fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let number = read_str!();
        let digits: Vec<u32> = break_up(number);

        // If we start with an even number we don't have to do anything
        if let Some(&d) = digits.last() {
            if d % 2 == 0 {
                println!("0");
                continue;
            }
        }

        // If we start with an even number....
        if digits[0] % 2 == 0 {
            if digits.len() == 1 {
                // A single digit number with 1 even digit is obviously already even...
                println!("0");
            } else {
                // Otherwise we reverse it
                println!("1");
            }
            continue;
        }



        // Everything else
        if digits.iter().any(|&d| d % 2 == 0) {
            // Reverse a substring so that the even digit is at the front. Then reverse the whole
            // number.
            println!("2");
        } else {
            // If there's no even digit at all, clearly the number will never be even no matter how
            // much flipping we do.
            println!("-1");
        }

    }

}

fn break_up(n: String) -> Vec<u32> {
    n.chars().map(|c| c.to_digit(10).unwrap()).collect()
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
    ($type:ty) => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).expect("read! was unable to read from stdin");
            inner.trim().parse::<$type>().expect("read! was unable to parse into the desired type")
        }
    };
}

/**
 Reads a full line of input and returns it as a `String`.
 */
#[allow(unused_macros)]
macro_rules! read_str {
    () => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).expect("read_str! was unable to read from stdin");
            inner.trim().to_owned()
        }
    };
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
    ($type:ty) => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).unwrap();
            inner
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<$type>().unwrap())
                .collect::<Vec<$type>>()
        }
    };
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
    ($type:ty) => {
        {
            let mut inner = String::new();
            std::io::stdin().read_line(&mut inner).unwrap();
            let v = inner
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<$type>().unwrap())
                .collect::<Vec<$type>>();
            let (a, b) = match &v[..] {
                &[f, s, ..] => (f, s),
                _ => unreachable!()
            };
            (a,b)
        }
    };
}

pub(crate) use read;
pub(crate) use read_str;
pub(crate) use read_vec;
pub(crate) use read_pair;

