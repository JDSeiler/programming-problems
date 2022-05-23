
/*
As input you get an array `a` of length `n` and a number `k`.

You get to perform at most `k` operations on the array. An "operation"
consists of setting the j-th bit of any number a_i in the array
to 1. But `j` must be in the range [0, 30]

Output the maximum possible value of a1 AND a2 AND ... AND a_n
after performing your k operations.

n :: [1, 2*10^5]
k :: [0, 10^9]
a_i :: [0, 2^31)

Clearly we cannot do anything linear in K because the bound is so
high. But, I think we can do the following:

We want to construct a 30 bit number, for each place j in our 30-bit
number:

Record:
1. The bitwise AND of the jth place of all a_i \in A
2. How many 0s are in the jth place of all a_i \in A

Then, we will make at most 30 operations (1 per bit). But the cost
of flipping the jth bit is equal to the number of zeros in that
bit across all of `a`. Our budget is k. We perform at most 30
operations because that's how many bits there are to flip. Once
we flip all 30 bits we can't do any better.

What operations do we choose to do? We check each bit of our
answer, starting at 30 and working down. We ask: Can we afford
to do this operation? If so, we do it and update our budget.
Else, we move on to the next bit. We continue until
we've checked every bit.

Our final answer is the result.`
*/


pub fn main() {
    let t = read!(u64);
    for i in 0..t {
        // https://stackoverflow.com/a/29509257
        let (n, k) = read_pair!(usize);
        let a: Vec<u32> = read_vec!(u32);

        // ith place is the ith bit, a[i] = num of zeros
        // 0 == least significant bit, read it right to left
        let mut bits: [u32; 31] = [0; 31];
        for num in a {
            // a..b = [a,b)
            for j in 0..31 {
                // Puts the jth bit in the 0s place and then we AND it with 1.
                let jth_bit = (num >> j) & 1;

                if jth_bit == 0 {
                    bits[j] += 1
                }
            }
        }

        let mut budget = k as u32;
        for j in (0..31).rev() {
            if bits[j] <= budget {
                budget -= bits[j];
                bits[j] = 0;
            }
        }

        let mut ans: u32 = 0;
        for j in 0..31 {
            if bits[j] == 0 {
                ans |= (1 << j);
            }
        }

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
Reads a full line of input and returns it as a `&str`.
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
