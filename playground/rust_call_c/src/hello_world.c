#include <stdio.h>
#include "hello_world.h"

void hello_world() {
    printf("Hello, World!\n");
    fflush(stdout);
}

int take_foo(Foo *foo) {
    return (*foo).bar;
}
