// Listing 1-1: A program that prints Hello, world!

fn main() {
    println!("Hello, world!");
}

/*
ANATOMY OF A RUST PROGRAM
Every Rust program starts with fn main(). The function body is inside {}.
To print to the screen, use println!(). A macro has an exclamation mark !.
End statements with ;. Format code using rustfmt.
*/

/*
COMPILING AND RUNNING ARE SEPARATE STEPS
Rust compiles first, then runs. rustc main.rs creates a binary.
On Linus/macOS: ./main
On Windows: .\main.exe
Dynamic languages need only one step but require an interpreter.
For larger projects -> use Cargo.
*/