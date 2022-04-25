// revisions: mir thir
// [thir]compile-flags: -Z thir-unsafeck

#![feature(const_extern_fn)]

const unsafe extern "C" fn foo() -> usize { 5 }

fn main() {
    let a: [u8; foo()];
    //~^ ERROR call to unsafe function `foo` is unsafe and requires unsafe function or block
    foo();
    //[mir]~^ ERROR call to unsafe function `foo` is unsafe and requires unsafe function or block
}
