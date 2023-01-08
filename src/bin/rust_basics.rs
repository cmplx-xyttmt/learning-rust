fn main() {
    let a = 10; // Types can be inferred by the compiler
    let b: i32 = 20; // or declared by the programmer when creating variables
    let c = 30i32; // Numeric types can include a type annotation in their literal form.
    let d = 30_i32; // Numbers can include underscores, which are intended to increase readability and have no functional impact.
    let e = add(add(a, b), add(c, d));

    println!("(a + b) + (c + d) = {}", e);

    println!("\n=========================\n");
    println!("Result of basic_number_ops function:");
    basic_number_ops();

    println!("\n=========================\n");
    println!("Result of bases function:");
    bases();

    println!("\n=========================\n");
    println!("Result of number_comparisons function:");
    number_comparisons();

    println!("\n=========================\n");
    println!("Result of comparing_floats function:");
    comparing_floats();
}

fn add(i: i32, j: i32) -> i32 { // Type declarations are required when defining functions
    i + j // functions return the last expression's result so that return is not required. Note: adding a semicolon changes the semantics, making the function return unit
}

fn basic_number_ops() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{twenty} + {twenty_one} + {twenty_two} = {addition}");

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2)); // Numbers have methods

    // Creates an array of numbers, which must all be the same type, by surrounding those with square brackets
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
        143.0067
    ];

    println!("{forty_twos:.02?}");  // Use question mark to pretty-print an array and .02 to round to 2 decimal places.
}

fn bases() {
    // Rust has support for numeric literals that allow you to define integers in base 2 (binary), base 8 (octal) and base 16 (hexadecimal)
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;

    println!("base 10: {three} {thirty} {three_hundred}");
    println!("base 2: {three:b} {thirty:b} {three_hundred:b}");
    println!("base 8: {three:o} {thirty:o} {three_hundred:o}");
    println!("base 16: {three:x} {thirty:x} {three_hundred:x}");
}

fn number_comparisons() {
    let a: i32 = 10;
    let b: u16 = 100;

    if a < (b as i32) {  // Need to cast operand to compare
        println!("Ten is less than one hundred");
    }

    // can also use the TryInto trait.
    let b_: i32 = b.try_into().unwrap();

    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

fn comparing_floats() {
    // Comparing equality of floating point numbers isn't recommended (because of how floats are represented on computers)
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("  0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("        0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("  0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("        0.3: {:x}", (xyz.2).to_bits());
    println!();

    let f32_equal = abc.0 + abc.1 == abc.2;
    let f64_equal = xyz.0 + xyz.1 == xyz.2;
    println!("f32 equal?: {f32_equal}");
    println!("f64 equal?: {f64_equal}");

    // It's recommended to use "tolerances" to allow comparisons between floating-point values.
    let result: f32 = 0.1 + 0.1;
    let desired: f32 = 0.2;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f32::EPSILON);
}
