#!/usr/bin/env bash

export LD_LIBRARY_PATH=./target/debug
if [ "$1" == "v" ]; then
    ./build.sh && valgrind ./main
else
    ./build.sh && ./main
fi

