use std::cmp::Ordering;

#[derive(Debug)]
enum Event {
    Start(u64),
    End(u64),
}

fn timestamp(t: &Event) -> u64 {
    match t {
        Event::Start(s) => *s,
        Event::End(e) => *e,
    }
}

enum State {
    Idle,
    Working,
}

pub fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let n = read!(usize);
        let starts = read_vec!(u64);
        let ends = read_vec!(u64);

        let tagged_starts = starts.iter().map(|&s| Event::Start(s));
        let tagged_ends = ends.iter().map(|&e| Event::End(e));

        let mut events: Vec<Event> = tagged_starts.chain(tagged_ends).collect();
        events.sort_by_key(timestamp);

        let mut last_start_time = 0;
        let mut state = State::Idle;
        let mut waiting_tasks = 0;
        let mut durations = Vec::new();

        for event in events {
            match state {
                State::Idle =>
                    match event {
                        Event::Start(s) => {
                            state = State::Working;
                            last_start_time = s; 
                        },
                        Event::End(_) => unreachable!("Ending a task while Idle is impossible!")
                    }
                State::Working =>
                    match event {
                        Event::Start(_) => {
                            waiting_tasks += 1;
                        },
                        Event::End(e) => {
                            durations.push(e - last_start_time);

                            match waiting_tasks.cmp(&0) {
                                Ordering::Equal => state = State::Idle,
                                // Continue working, but pull a pending tasks.
                                Ordering::Greater => {
                                    waiting_tasks -= 1;
                                    // Treat this as a task start
                                    last_start_time = e;
                                },
                                Ordering::Less => unreachable!("Having a negative number of waiting tasks is impossible!")
                            }
                        }
                    }
            }
            // End `match state`
        }
        let mut ans = String::with_capacity(durations.len() * 2);
        for &d in durations.iter() {
            // The sample output has a trailing space on CF
            ans.push_str(&format!("{} ", d)); 
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

