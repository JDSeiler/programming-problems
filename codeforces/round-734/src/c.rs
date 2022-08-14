
/* 
You're given 200,000 words whose combined length is 400,000.

Each word consists only of: a,b,c,d, or e. There may be duplicate words.
A story is a sequence of words that are not necessarily different. A story
is "interesting" if there is some letter (called the "interesting letter")
that occurs more than all other letters in the story.

Output the maximum number of words you can use to make an interesting story.

The first observation is that there are only 5 letters. Thus, there are only
5 possiblities for which letter occurs more than all the others. This is an 
opportunity to brute force. Let's explore this direction.

Let us suppose we are told in advance *which* letter occurs more than all the 
other letters in the largest interesting story. Call this magic letter "X"
How can we compute the size of the story?

Let use rank all the words we were given by how often X occurs compared to the 
most common letter out of all the OTHER letters in the word. For example:
`a` has a power of 1 in "aaabbc" because `a` occurs 3 times while `b` occurs twice.
`b` is the most common out of all the other letters in the word, so we do 3-2=1.

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
I do not have a proof for this, so this is the most tenuous part of my solution.
Check if adding the word would make the story uninteresting. If that is the case, discard it,
otherwise, add it to the story. 

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

        for c in 'a'..='a' {
            let mut words_with_power: Vec::<(&String, i32, [i32; 5])> = words.iter().map(|w| {
                // If we're going to compute the letter frequency for each word we 
                // may as well store it so we don't have to redo it later.
                let (power, counts) = compute_relative_power(c, w);
                (w, power, counts)
            }).collect();

            // Sort by the power value in descending order.
            words_with_power.sort_unstable_by(|a, b| (b.1).cmp(&a.1));

            // We are guaranteed to get at least 1 word as input, so this is safe.
            // If the best word we have doesn't have a positive power, we cannot
            // make an interesting story where `c` is the interesting letter.
            if words_with_power[0].1 <= 0 {
                continue;
            } 

            let mut story_size = 0;
            let mut story_letter_frequency = [0, 0, 0, 0, 0];
            for (_word, power, counts) in words_with_power.iter() {
                if *power >= 0 {
                    story_size += 1;
                    update_counts_inplace(&mut story_letter_frequency, counts); 
                } else {
                    let mut hypothetical_counts = update_counts(&story_letter_frequency, counts);
                    
                    let c_idx = (c.to_digit(16).unwrap() - 10) as usize;
                    let c_count = hypothetical_counts[c_idx];

                    hypothetical_counts.swap(c_idx, 4);
                    let [w,x,y,z,_] = hypothetical_counts;
                    let story_power = c_count - w.max(x).max(y).max(z);
                    if story_power > 0 {
                        story_size += 1;
                        update_counts_inplace(&mut story_letter_frequency, counts); 
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

        println!("{:#?}", solutions);

        println!("{}", best_solution);
    }
}

fn compute_relative_power(l: char, word: &str) -> (i32, [i32; 5]) {
    // a, b, c, d, e
    let mut counts = [0, 0, 0, 0, 0];

    for c in word.chars() {
        let idx = (c.to_digit(16).unwrap() - 10) as usize;
        counts[idx] += 1;
    }

    let l_idx = (l.to_digit(16).unwrap() - 10) as usize;
    let l_count = counts[l_idx];

    counts.swap(l_idx, 4);
    let [w,x,y,z,_] = counts;
    (l_count - w.max(x).max(y).max(z), counts)
}

fn update_counts(acc: &[i32; 5], some_counts: &[i32; 5]) -> [i32; 5] {
    let mut new_counts = [0, 0, 0, 0, 0];
    for i in 0..4 {
        new_counts[i] = acc[i] + some_counts[i];
    }
    new_counts
}

fn update_counts_inplace(acc: &mut [i32; 5], some_counts: &[i32; 5]) {
    for i in 0..4 {
        acc[i] += some_counts[i];
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

