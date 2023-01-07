// use std::thread;
//
// // This shows an example of a data race condition:
// // i.e the inability to determine how a program behaves from run to run due to changing
// // external factors.
//
// // Compiler will give the following error:
// //       closure may outlive the current function, but it borrows `data`, which is owned by the current function
//
// // Rust doesn't allow multiple places in an application to have write access to data.
// fn race_condition() {
//     let mut data = 100;
//
//     thread::spawn(|| { data = 500; });
//     thread::spawn(|| { data = 1000; });
//
//     println!("{}", data);
// }

fn buffer_overflow() {
    let fruit = vec!["peach", "banana", "grapes"];

    let buffer_overflow = fruit[4]; // Rust will cause a crash rather assign an invalid memory location to a variable
    assert_eq!(buffer_overflow, "watermelon");
}

fn iterator_invalidation() {
    let mut letters = vec!["a", "b", "c"];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());  // Rust doesn't allow the letters variable to be modified within the iteration block
    }
}

fn main() {
    // race_condition();

    // buffer_overflow();

    iterator_invalidation();
}
