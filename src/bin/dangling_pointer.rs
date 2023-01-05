#[derive(Debug)]
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; // initializes an empty vector of Cereal
    grains.push(Cereal::Rye);
    drop(grains);  // Deletes grains and its contents
    println!("{:?}", grains); // Attempts to access the deleted value
}
