## hello world script 

-> building a minimal script that prints hello world and compile it using rustc

## hello world cargo 

-> build a minimal "hello world" cargo project initialized with cargo new hello_cargo
First run `cargo new hello_cargo`
Once this is done, we can run `cargo build` to build the project

The commands that we can also use are : 
`cargo run` compiles and run the code
`cargo check` checks the code for errors and warnings without building it

Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

The command `cargo build --release` can be used to build the project with optimizations for release. This will generate a smaller and faster executable, but the build process may take longer.

I am currently at [Chapter 2: Guessing Game Tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number) in the Rust book.
