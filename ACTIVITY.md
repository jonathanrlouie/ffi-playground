# Workshop Activity

## Calling C from Rust
For this exercise, we will be creating Rust bindings for a C library.

Navigate to the `playground/rust_call_c` folder. In the `src` directory of this folder are 4 files: `hello_world.h`, `hello_world.c`, `main.rs`, and `solution.rs`.

Open `main.rs` and add the missing extern block(s) and type definition(s) corresponding to `hello_world.h`.
You should not need to modify the `main` function at all.
You can build and run the program using `cargo run`.

When you feel satisfied with your solution, you may read `solution.rs` to check your answer.

## Calling Rust from C
For this exercise, we will be writing Rust functions that will be exposed to C and then generating C bindings for them.

Navigate to the `playground/c_call_rust` folder.

The `main.c` file calls several Rust functions that have not been defined and implemented.
Open `src/lib.rs` and add the missing Rust function definitions for `pass_int`, `contains_rust`, `get_rust_string`, and `free_rust_string`.
You may execute `run.sh` to build the Rust library, generate the C bindings, and compile and run the C code.

To verify that your program does not leak memory, ensure you have Valgrind installed and run `./run.sh v`.

When you feel satisfied with your solution, you may read `solution.rs` to check your answer.
