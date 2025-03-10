// Iterated over after being altered midway through
// Error: move occurs because `letters` has type `Vec<&str>`, which does not implement the `Copy` trait.
fn main() {
    let mut letters = vec![
        // Creates a mutable vector letters
        "a", "b", "c",
    ];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone()); // Copies each letter and appends it to the end of letters
    }
}
