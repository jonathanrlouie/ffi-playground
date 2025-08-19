// Exercise:
// Add the required extern blocks and type definitions to make main()
// run successfully.
//
// Try it by hand first, then try generating it with rust-bindgen.
//
// Compare your solution with solution.rs. Do you notice any differences?

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
