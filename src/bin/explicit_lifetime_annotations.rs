// <'a, 'b> declares two lifetime variables within the scope of add_with_lifetimes
// (spoken as "lifetime a" and "lifetime b")
// i: &'a i32 binds lifetime variable 'a to the lifetime of i. reads as: "parameter i is a reference to an i32 with lifetime a"
// same for j

// Rust's safety checks have a lifetime system that verifies that all attempts to access data are valid.
// All values bound to a given lifetime must live as long as the last access to any value bound to that lifetime.
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j // adds the values referred to by i and j rather than adding the references directly
}

fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b); // no lifetime annotation is required when calling a function
    println!("{res}");
}
