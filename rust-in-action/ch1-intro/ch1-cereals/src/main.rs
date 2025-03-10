// Attempting to create a dangling pointer

#[derive(Debug)] // Allows println! macro to print the Cereal enum
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let mut grains: Vec<Cereal> = vec![]; // Initialize empty vector
    grains.push(Cereal::Rye); // Add element to vector
    drop(grains); // Delete variable and it's content
    println!("{:?}", grains); // Attempt to access deleted value - "borrow of moved value"
}
