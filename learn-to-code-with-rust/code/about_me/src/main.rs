const TAX_RATE: f64 = 7.25;

type Meters = i32;

fn main() {
    println!("The tax rate is {TAX_RATE}");

    print!("Hi there! ");
    println!("My name is Tony.");
    println!("I live in Frisco TX.");
    println!("I am software engineer.");
    // To format, check code, compile and execute a release build run and clean
    // "cargo fmt && cargo check && cargo b -r && cargo r -rq && cargo clean"

    let apples = 50; // by default declare an immutable variable
    let oranges = 14 + 6;

    let mut fruits = apples + oranges; // declare a mutable variable
    fruits -= 2;

    // Before Rust 1.5: positional arguments are assigned an index of order of appearance:
    println!(
        "Yes, {1} oranges. Like I saids, this year my garden has {} apples and {} oranges. Hey, these are {0} apples and {1} oranges...",
        apples, oranges
    );
    // Rust 1.5 syntax allows direct variable interpolation:
    println!(
        "So, they were {} fruits. but I ate some, so they are only {fruits} now.",
        apples + oranges
    );

    // Variable shadowing
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    let grams_of_protein = 100;
    println!("{grams_of_protein} grams of protein.");

    // Variable scope
    let coffee_price = 12.34;

    {
        // nested scope
        let coffee_price = 6.23;
        println!("Coffee price is {coffee_price}");
    }

    println!("Coffee price is {coffee_price}");

    // Type aliases

    #[allow(unused_variables)]
    let mile_race_length: i32 = 1600;
    let two_mile_race_length: Meters = 3200;

    println!("Two mile race is {two_mile_race_length}");

    #[allow(unused_variables)]
    {
        let mile_race_length: i32 = 1600;
        let two_mile_race_length: Meters = 3200;
    }

    let some_string: &str = "Some string";
}
