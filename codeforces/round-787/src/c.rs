pub fn main() {
    let t = read!(u64);
    for _ in 0..t {
        let answers = read_str!();
        let timeline = format!("1{}0", answers);

        /*
         * Notes because I WILL forget how this works later:
         * A key observation is that once we have a 0, we cannot have a 1 ever again or its a
         * logical contradiction. Furthermore, we can add some "phantom answers" to get a complete
         * timeline. We know that the painting was there before any friends arrived (this is the 1
         * we add) and we know it was stolen by SOMEONE (this is the 0). In a perfect world, the
         * timeline looks something like:
         * 1...10...0
         * And there are two candidates, the people who are on the border between 1 and 0. But,
         * because there are ? answers, it might be ambiguous where that border lies.
         *
         * This code finds the distance between the rightmost 1 and the leftmost 0. That distance
         * directly corresponds to the number of people who COULD be "on the border" and thus
         * suspects. Observe:
         * 111??000
         * ..abcd..
         * The "10" border could be on"
         * ab, bc, or, cd, and the entire answer be consistent. Thus there are 4 suspects.
         * */
        let rightmost_one = timeline.rfind('1').unwrap() as i64;
        let leftmost_zero = timeline.find('0').unwrap() as i64;

        // If it so happens that we end up using either of our phantom number (say the answers
        // start with 0), then we have to do some adjustments to get the right answer.
        let one_adjust = if rightmost_one == 0 { -1 } else { 0 };
        let zero_adjust = if leftmost_zero == timeline.len() as i64 - 1 { -1 } else { 0 };

        println!("{}", leftmost_zero - rightmost_one + 1 + one_adjust + zero_adjust);
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

