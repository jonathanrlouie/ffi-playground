#!/usr/bin/env bash

cargo build
cbindgen --config cbindgen.toml --crate cffi --output cffi.h --lang c
gcc -g main.c -o main -lcffi -L./target/debug
