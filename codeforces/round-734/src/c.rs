
/* 
You're given 200,000 words whose combined length is 400,000.

Each word consists only of: a,b,c,d, or e. There may be duplicate words.
A story is a sequence of words that are not necessarily different. A story
is "interesting" if there is some letter (called the "interesting letter")
that occurs more than all other letters in the story COMBINED.

Output the maximum number of words you can use to make an interesting story.

The first observation is that there are only 5 letters. Thus, there are only
5 possiblities for which letter occurs more than all the others. This is an 
opportunity to brute force. Let's explore this direction.

Let us suppose we are told in advance *which* letter occurs more than all the 
other letters in the largest interesting story. Call this magic letter "X"
How can we compute the size of the story?

Let use rank all the words we were given by how often X occurs compared to 
all the other letters. For example, we would say that the word: "aaabc" has 
a "power" of 1 with respect to `a` because `a` occurs 3 times, and there are 
2 other letters. 3-2=1.

Given a story that is interesting w.r.t X, adding a word that is "powerful" w.r.t X
will always give you a longer story that is interesting w.r.t X. We can use this 
to argue that any optimal interesting story must include all words that are powerful
w.r.t X. Assume for the sake of contradiction we have a solution w.r.t X: S, that
does not include all powerful words. We can increase the size of S by adding any 
word that is powerful w.r.t. X. Therefore, S was not optimal.

Because "not including all powerful words" implies "your solution is not optimal", then
by the contrapositive we know "your solution is optimal" implies "it includes all powerful words".

Given a story, adding a word that has a power of 0 w.r.t X will not change whether the
story is interesting or not. So, given an interesting story w.r.t X, it is strictly better
to include words that are "neutral" (power 0) w.r.t X.

Lastly, adding a word with a negative power w.r.t X *may* change whether the story is 
interesting, depending on the composition of the word. Out of all words with negative 
power w.r.t X, it is optimal to greedily choose those with the largest power first.
Because we are only concerned that X occurs more than all other letters COMBINED, there
is no reason to take anything besides the word with the largest power.
So, we continue adding words wih negative power until the story would no longer be interesting.

This algorithm for a known X requires the following steps:
Let n := number of words
Let l := combined length of all words

1. Compute relative powers :: O(l)
2. Sort :: O(n log n)
3. Select words :: O(n)

To brute force this for all 5 letters would be:
5 letters times:
    3.6 million operations per letter (give by nlogn when n is 200,000)
        18 million operations

With a time limit of 4 seconds, I feel "just ok" about the performance.
For each letter, simply assume that it's the interesting letter. Then compute 
a solution. For a given interesting letter X, there may be NO words with positive
power w.r.t X. In this case, the answer is 0. If all letters produce 0, then the 
overall answer is 0. Otherwise, we output the maximum.

Jordan from the future: This solution works and finishes in 108ms. So it's not "fast code"
but I suspect that this solution is what the authors intended considering how fast it is.
*/

pub fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let n = read!(usize);
        let mut words = Vec::with_capacity(n);
        for _ in 0..n {
            let word = read_str!();
            words.push(word);
        }
        let mut solutions = [0, 0, 0, 0, 0];

        for c in 'a'..='e' {
            // the word, its power w.r.t c, "the counts"
            // counts[0] is how many times `c` appears
            // counts[1] is how many time a letter besides `c` appears (len - counts[0])
            let mut words_with_power: Vec<(&String, i32, [i32; 2])> = words.iter().map(|w| {
                let (power, counts) =  compute_relative_power(c,w);
                (w, power, counts)
            }).collect();

            // Sort in descending order of power
            words_with_power.sort_unstable_by(|a, b| b.1.cmp(&a.1));

            // If we don't have single powerful word, this `c` cannot produce
            // an interesting story
            if words_with_power[0].1 <= 0 {
                continue;
            }

            let mut story_size = 0;
            // # of c, # of everything else
            let mut story_counts = [0, 0];
            for (_w, p, c) in words_with_power {
                if p >= 0 {
                   story_size += 1;
                   story_counts[0] += c[0];
                   story_counts[1] += c[1];
                } else {
                    // Suppose we added this word to the story...
                    let hypothetical_counts = [
                       story_counts[0] + c[0],
                       story_counts[1] + c[1]
                    ];
                    // If we'd still have an interesting story w.r.t `c`, then add it.
                    if hypothetical_counts[0] > hypothetical_counts[1] {
                       story_size += 1;
                       story_counts[0] += c[0];
                       story_counts[1] += c[1];
                    }
                }
            }
            let c_idx = (c.to_digit(16).unwrap() - 10) as usize;

            solutions[c_idx] = story_size;
        }

        // reduce returns None if the iterator is empty. But that's impossible
        // so we can safely unwrap.
        let best_solution = solutions.iter().reduce(|acc, x| {
            if x > acc { x } else { acc }
        }).unwrap();

        println!("{}", best_solution);
    }
}

fn compute_relative_power(l: char, word: &str) -> (i32, [i32; 2]) {
    // a, b, c, d, e

    let frequency_of_l = word.chars().fold(0, |acc, c| {
        if c == l {
            acc + 1
        } else {
            acc
        }
    });

    let others = word.len() as i32 - frequency_of_l;

    (frequency_of_l - others, [frequency_of_l, others])
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

