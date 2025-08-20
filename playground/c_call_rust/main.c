#include <stdint.h>
#include <assert.h>
#include <string.h>
#include <stdio.h>
#include "c_call_rust.h"

int main() {
    uint32_t i = 5;
    bool b = pass_int(5);
    assert(b);

    char *str = (char*) malloc(10*sizeof(char));
    strcpy(str, "rustacean");
    assert(contains_rust(str));
    // We allocated the string on the C side, so we should use the
    // same allocator to free it.
    free(str);

    // Rust allocated this string, so Rust should free it
    char *rust_string = get_rust_string();
    assert(contains_rust(rust_string));
    free_rust_string(rust_string);

    printf("Success!");

    return 0;
}

