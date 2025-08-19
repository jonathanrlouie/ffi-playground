# ffi-playground
A small playground Docker environment for testing C FFI in Rust. Includes Valgrind.

# Building the Docker image
The provided Docker image mounts the `playground` directory into the container, so you can easily drop any existing project into this environment for testing with Valgrind by placing it in the `playground` directory. Two existing sample apps, `c_call_rust` and `rust_call_c`, are provided in this directory as well.

To build the Docker image, run the following command:
```
docker build -t ffi-sandbox - < Dockerfile
```

# Running the Docker container
A shell script is provided for running the Docker container. This script also will remove the container when finished. To use this script, run:
```
./start_ffi_playground.sh
```

# `c_call_rust` sample app

## Building the sample app
A script is provided for building the sample app. This script will build the Rust library, generate C bindings for it, and then build the C sample app.

From the `playground/c_call_rust` directory, run:
```
./build.sh
```

## Running the app
From the `playground/c_call_rust` directory, run:
```
./run.sh
```
Or, to run it with Valgrind:
```
./run.sh v
```

# `rust_call_c` sample app

## Building the sample app
The sample app can be built by simply running `cargo build`. The build uses a custom `build.rs` script to compile the C code using the [`cc` crate](https://crates.io/crates/cc).

## Running the app
The sample app can be built by simply running `cargo run`.

Note that when running this sample app with Valgrind, using `cargo run` may result in strange errors. If you want to run this app through Valgrind, use the raw executable in the `target/debug` folder instead.

# Exiting the container
Once you are finished using the environment, you can simply exit the container by using the `exit` command.

# For Nix users
If you have flakes enabled, you can use `nix develop` to start a devShell environment with Rust, rust-bindgen, cbindgen, and Valgrind installed.

Otherwise, you can use `nix shell` instead to start the devShell environment.
