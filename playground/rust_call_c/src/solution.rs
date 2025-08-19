unsafe extern "C" {
    // We can't pass any values to hello_world, so we can't pass any invalid values.
    // Therefore, we can mark this as safe.
    // Note that rust-bindgen won't do this automatically for you!
    pub safe fn hello_world();
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub bar: ::std::os::raw::c_int,
}

// Some extra info printed by rust-bindgen.
// 
// This might be a bit hard to read. Essentially, this is a comment telling 
// you the size and alignment of Foo, as well as the offset of its fields.
// The actual value of this constant isn't usable for anything.
//
// Here's what the first line is equivalent to:
// ```
// let arr: [&str; 1] = ["Size of Foo"];
// let index = ::std::mem::size_of::<Foo>() - 4usize;    // This evaluates to 0
// arr[index];    // This ends in a semi-colon, so we actually just get `()`
// ```
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of Foo"][::std::mem::size_of::<Foo>() - 4usize];
    ["Alignment of Foo"][::std::mem::align_of::<Foo>() - 4usize];
    ["Offset of field: Foo::bar"][::std::mem::offset_of!(Foo, bar) - 0usize];
};

unsafe extern "C" {
    // We could hypothetically pass in an invalid Foo pointer.
    // We don't need to explicitly mark this unsafe (it's implied by the block).
    pub fn take_foo(foo: *mut Foo) -> ::std::os::raw::c_int;
}

fn main() {
    hello_world();
    let mut foo: Foo = Foo { bar: 2 };
    let foo_ptr: *mut Foo = &mut foo;
    assert_eq!(2, unsafe { take_foo(foo_ptr) });

    let mut another_foo: Foo = Foo { bar: 7 };
    let another_foo_ptr: *mut Foo = &mut another_foo;
    assert_eq!(7, unsafe { take_foo(another_foo_ptr) });

    println!("Success!");
}
