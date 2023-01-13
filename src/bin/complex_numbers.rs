// Using the num crate to do operations on complex numbers
use num::complex::Complex;  // use keyword brings the Complex type into local scope.

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };  // every rust type has a literal syntax.
    let b = Complex::new(11.1, 22.2); // Most types implement a new() static method.
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
