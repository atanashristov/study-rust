# learn-to-code-with-rust

Code and notes from studying [Learn to Code with Rust: Master Rust: The Safe, Fast, and Modern Programming Language](https://www.packtpub.com/en-us/product/learn-to-code-with-rust-9781837024155)

## Getting Started

### Installation

- Visual Studio

Install Visual Studio Community 2022 and specifically "Desktop development with C++".

- Rust Compiler

Then install Rust from [Rust web site](https://www.rust-lang.org/).
Navigate to [Getting Started](https://www.rust-lang.org/learn/get-started) page and download `rustup-init.exe`.
Note: At the time the version was [1.85.0](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html).

Run `rustup-init.exe` and you will see this message. It will install rust and other supplementary tools as the Cargo package management, rust-up, etc.

```sh
Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\atanas.hristov\.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  C:\Users\atanas.hristov\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\atanas.hristov\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
>

```

Proceed with "1". Once ready, run the Rust compiler `rustc --version` to confirm Rust is installed:

```sh
rustc --version
rustc 1.85.0 (4d91de4e4 2025-02-17)
```

- VS Code

Next install [VS Code](https://code.visualstudio.com/).

Check the age about [VS Code Rust support](https://code.visualstudio.com/docs/languages/rust).

Install the "rust-analyzer" VS Code extension. From the extension screen, make sure "auto update" is selected.

Optionally install theme and icon-set. The author uses "Winter is Coming Theme" and "Material icon theme".

### `rustup` and Documentation

To begin working with Rust, we are running a command line tool `rustup`:

```sh
The Rust toolchain installer

Usage: rustup [OPTIONS] [+toolchain] [COMMAND]

Commands:
  show         Show the active and installed toolchains or profiles
  update       Update Rust toolchains and rustup
  check        Check for updates to Rust toolchains and rustup
  default      Set the default toolchain
  toolchain    Modify or query the installed toolchains
  target       Modify a toolchain's supported targets
  component    Modify a toolchain's installed components
  override     Modify toolchain overrides for directories
  run          Run a command with an environment configured for a given toolchain
  which        Display which binary will be run for a given command
  doc          Open the documentation for the current toolchain
  self         Modify the rustup installation
  set          Alter rustup settings
  completions  Generate tab-completion scripts for your shell
  help         Print this message or the help of the given subcommand(s)
```

We can run `rustup update` to update the Rust tooling.

We can run `rustup doc` to open the local Rust documentation html page. The documentation comes with two books:

- The Rust Programming Language
- Rust By Example

### Hello World

Create new project with `cargo`. The name of the project follows snake_case convention:

```sh
cargo new hello_world
 Creating binary (application) `hello_world` package
```

It creates _binary application package_. They are two types of packages (crates) with Rust:

- binary crate: standalone application
- library crate: to incorporate in another project

This is the result project:

```sh
  src
    |-main.rs
        fn main() {
            println!("Hello, world!");
        }
  Cargo.toml
        [package]
        name = "hello_world"
        version = "0.1.0"
        edition = "2024"

        [dependencies]
```

In `src` it keeps the core. in `target` generates the compiled code.

The `Cargo.toml` is the configuration of the project. This is the [Cargo Manifest Documentation Page](https://doc.rust-lang.org/cargo/reference/manifest.html).

The `Cargo.lock` is generated upon compilation. It keeps track of the dependencies. You should _always include in the source control_.

The `main` function prints "Hello, World". The indentation is 4 spaces. The macro `println!` ends with exclamation and it is is a procedure that somebody provides. Technically not a function, but for now we can think of it like if it is a function.

The string is surrounded by double quotes. The lines (commands) are terminated by semicolon.

We can run the code directly from VS Code.

We can also compile and run from the command line.

```sh
cd src
rustc main.rs
cd ..
.\target\debug\hello_world.exe
```

By default it creates binary for the local computer and processor architecture.

Formatting the code is don in several ways:

- VS code extension takes care
- `rustfmt main.rs` to format one file
- `cargo fmt` formats all the files

### `cargo` commands

Run `cargo build` or `cargo b` to build all the code including all libraries. By default it produces debug mode build:

```sh
cargo build
   Compiling hello_world v0.1.0 (C:\Users\...\hello_world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.55s
```

To compile in executable mode run `cargo build -release` or `cargo b -r` short:

```sh
cargo build -r
   Compiling hello_world v0.1.0 (C:\Users\...\hello_world)
    Finished `release` profile [optimized] target(s) in 0.18s
```

To clean the builds run `cargo clean`.

And finally we can run `cargo run` that compiles and runs the program. Or to run it quietly `cargo r --quiet` or even `cargo r -q`.

To analyze the code wun `cargo check`. It will check for code check for syntax or logical code violations.

### Comments

We can write line comments starting with  `// line comment`. Shortcut to toggle line comment is _Ctrl+/_ (_Cmd+/_ omn the Mac).

We can write block comments with `/* block comment */`.

## Variables and Mutability

Code is under [about_me](./code/about_me) directory.

### Variables

By default variables are immutable and we have positional and inline string interpolation:

```rs
fn main() {
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
}
```

Rust infers an `i32` for integer and an `f64` for a float.

While Rust will infer the variable type from the initialization, we can explicitly annotate it:

```rs
let mile_race_length: i32 = 1600;
let some_string: &str = "Some string";
```

### Error Codes

Every error has a numeric code prefixed with "E":

```sh
error[E0282]: type annotations needed
```

You can get more info for an error with `rustc --explain`:

```sh
rustc --explain E0282

The compiler could not infer a type and asked for a type annotation.
...
```

Google "Rust error codes" for more information about the error codes.

### Variables Shadowing

We can use the same name of variable but with a different type. The previous version becomes unusable (overshadowed) after the shadowing. This is mainly used to convert values and reuse the variable name:

```rs
    let grams_of_protein = "100.345";
    let grams_of_protein = 100.345;
    let grams_of_protein = 100;
    println!("{grams_of_protein} grams of protein.");
```

### Variable Scope, Nested Scope

The variables exist within a scope between curly brackets `{...}`:

```rs
    let coffee_price = 12.34;

    {// nested scope
        let coffee_price = 6.23;
        println!("Coffee price is {coffee_price}");
    }

    println!("Coffee price is {coffee_price}");
```

This code prints:

```sh
Coffee price is 6.23
Coffee price is 12.34
```

### Constants

Compile time initialized and permanently immutable.

Can be declared outside a function. Can be accessed from any function thereof.

While the type of the variables can be inferred, we have to explicitly assign type.

```rs
const TAX_RATE: f64 = 7.25;

fn main() {
    println!("The tax rate is {TAX_RATE}");
```

### Type Aliases

We can provide additional context what that type represents.

```rs
type Meters = i32;

fn main() {

    let mile_race_length: i32 = 1600;

    let two_mile_race_length: Meters = 3200;
```

### Compiler Directives

Metadata that customizes how the compiler works (different configuration sets).

Directive above a line applies to the line.

Directive above a function or a block applies to the function or the block.

For example we can allow unused variables:

```rs
    #[allow(unused_variables)]
    let mile_race_length: i32 = 1600;
    ...

    #[allow(unused_variables)]
    {
        let mile_race_length: i32 = 1600;
        let two_mile_race_length: Meters = 3200;
    }
```

Note: we can also just prefix with underscore the variable name.

Directive at beginning of a file applies to the entire file. In this case we also add an exclamation point after the hash tag:

```rs
#![allow(unused_variables)]
```

## Project solution
