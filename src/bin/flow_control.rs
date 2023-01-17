use std::time::{Duration, Instant};  // brings Duration and Instant types from std::time into local scope

fn while_example() {
    // using while to stop iterating once a duration is reached
    let mut count = 0;
    let time_limit = Duration::new(1, 0); // creates a Duration that represents 1 second
    let start = Instant::now();  // Accesses time from the system's clock

    while (Instant::now() - start) < time_limit { // an Instant minus an Instant returns a Duration
        count += 1;
    }

    println!("{count}");  // prints how fast your computer can increment a count in 1 second.

    // Note: Avoid a "while true" loop for a never ending loop.
    // Rust provides a "loop" keyword for this. It continues executing until a break statement is encountered.

    // Can use loop labels to break out of nested loops.
    // A loop label is an identifier prefixed with an apostrophe (') e.g:
    //          'outer: for x in 0..n

}

fn match_example() {
    // let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{item}: {result}");
        }
    }
}

fn main() {
    while_example();
    match_example();
}
