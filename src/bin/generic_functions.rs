use std::time::Duration;

// T is a type variable
// To require that type T must support addition, include a trait bound alongside the type variable in the function's definition
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let total = add(2, 3);
    println!("{total}");

    let durations = add(Duration::new(5, 0), Duration::new(10, 0));
    println!("{durations:?}"); // because std::time::Duration does not implement the std::fmt::Display trait, can fall back to requesting std::fmt::Debug
}
