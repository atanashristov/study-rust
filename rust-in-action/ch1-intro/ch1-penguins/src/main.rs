// Shows basic processing of CSV data.

fn main() {
    // Escapes the trailing newline character
    let penguin_data = "\
  common name,length (cm)
  Little penguin,33
  Yellow-eyed penguin,65
  Fiordland penguin,60
  Invalid,data
  ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            // Skips header row and lines with only whitespace
            continue;
        }

        // `Vec`: a collection type that can expand dynamically
        // `_`: instructs Rust to infer the type of the elements
        let fields: Vec<_> = record // Starts with a line of text
            .split(',') // Splits record into fields
            .map(|field| field.trim()) // Trims whitespace of each field
            .collect(); // Builds a collection of fields

        if cfg!(debug_assertions) {
            // cfg! checks configuration at compile time
            // `{:?}`: tells Rust to use a default representation
            eprintln!("debug: {:?} -> {:?}", record, fields); // eprintln! prints to standard error (stderr)
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            // Attempts to parse field as a floating- point number
            // `{}`: tells Rust to use a programmer-defined method to represent the value
            println!("{}, {}cm", name, length); // println! prints to standard out (stdout)
        }
    }
}
