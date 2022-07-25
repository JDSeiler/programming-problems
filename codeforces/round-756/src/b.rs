
/**
 * There are two groups, their sizes are given as `a` and `b`.
 * We want to make teams of 4. Each team of 4 must
 * contain at least one member from each group.
 * */

// Tried to get away with a cheap trick and it didn't work :( 
// Skipped and moved on to C

pub fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let (a, b) = read_pair!(u64);
        // Head off easy cases at the gate:
        // If either value is 0, we're cooked. We can make no teams
        if a == 0 || b == 0 {
            println!("0");
            continue;
        }

        /*
        If the values are equal, there is no reason we should have teams of:
        1 "a" and 3 "b" or vice versa. We have the same amount of both resources
        so the choice we make is irrelevant. So, we can make teams by evenly
        splitting between "a" and "b", that is, each team has 2 "a" and 2 "b".
        Therefore, the total number of teams is floor( (a+b) / 4 ).

        Alternately, it's floor( a / 2 ) if that's more intuitive.
        They are algebraically equivalent.
        */
        if a == b {
            let ans = (a+b) / 4;
            println!("{}", ans);
            continue;
        }

        /*
        In all other cases, we should be as sparing as possible with the resource 
        that we have the least of. It never makes sense to use more than one of the 
        minority, because IF there is a sufficient surplus of the majority, you could've 
        made a team with that person you just consumed. But if you use them up, then
        that possibility is erased.

        So, assume we are given (a, b) = n, infinity. Clearly we can make `n` teams.
        But, since each team requires 3 people, we *could* run out of `b`.

        So either:
        1. We run out of our minority first
        2. We use some of our minority but there isn't enough majority left
        to another team.
        */
        let (smaller_group, larger_group) = (a.min(b), a.max(b));
        let ans = smaller_group.min(larger_group / 3);
        println!("{}", ans);
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

