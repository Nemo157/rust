#![allow(dead_code)]
#![deny(no_mangle)]

struct Bar;

#[allow(no_mangle)]
mod allowed_no_mangle {
    #[no_mangle] fn allowed() {}
}

macro_rules! no_mangle_in_macro {
    () => {
        #[no_mangle] fn foo() {} //~ ERROR: declaration of a `no_mangle` function
        #[no_mangle] static FOO: u32 = 5; //~ ERROR: declaration of a `no_mangle` static
    }
}

#[no_mangle] fn foo() {} //~ ERROR: declaration of a `no_mangle` function
#[no_mangle] static FOO: u32 = 5; //~ ERROR: declaration of a `no_mangle` static

fn main() {
    no_mangle_in_macro!();
}
