#![feature(address_space)]
//~^ WARN the feature `address_space` is incomplete and may not be safe to use and/or cause compiler crashes [incomplete_features]

#[address_space(0)] //~ ERROR attribute should be applied to a static or foreign static
const CONST_FOO: usize = 42;

#[address_space(0, 1, 2)] //~ ERROR incorrect number of arguments to `#[address_space]`
static BAD_FOO: usize = 42;

#[address_space(0)]
static FOO: usize = 42;

#[address_space(0)]
static mut MUT_FOO: usize = 42;

extern "C" {
    #[address_space(fake_space)] //~ ERROR invalid address space value format in `address_space`
    static BAD_EXTERN_FOO: usize;

    #[address_space(0)]
    static EXTERN_FOO: usize;
}

#[address_space(65_536)] //~ ERROR address space value in `address_space` is too large: `65536`
static TOO_BIG: usize = 7;

#[address_space(0)] //~ ERROR attribute should be applied to a static or foreign static
fn main() {
    #[address_space(0)] //~ ERROR attribute should be applied to a static or foreign static
    let _foo = 8;
}
