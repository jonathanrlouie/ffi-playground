#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

bool pass_int(uint32_t _i);

/**
 * Note: Since the caller can pass an arbitrary *const c_char, we need to
 * add safety docs here and mark the function as unsafe. The Rust FFI Omnibus
 * does not do this, but it really should, since it would be UB if a Rust
 * function called this with an invalid pointer without using an unsafe block.
 *
 * # Panics
 *
 * This function will panic if `s` is null or not a valid UTF-8 string.
 *
 * # Safety
 *
 * It is undefined behaviour to call this function for any non-null pointer `s` that cannot be converted safely to a `CStr`.
 * See the safety docs for [CStr::from_ptr](https://doc.rust-lang.org/src/core/ffi/c_str.rs.html#263)
 */
bool contains_rust(const char *s);

char *get_rust_string(void);

/**
 * ## Safety
 *
 * This should only ever be called with a pointer earlier obtained by calling `get_rust_string`.
 */
void free_rust_string(char *s);
