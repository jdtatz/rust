#[address_space(0)] //~ ERROR the `#[address_space]` attribute is an experimental feature
static FOO: usize = 42;

extern "C" {
    #[address_space(0)] //~ ERROR the `#[address_space]` attribute is an experimental feature
    static EXTERN_FOO: usize;
}

#[address_space(65_536)] //~ ERROR the `#[address_space]` attribute is an experimental feature
                         //~^ ERROR address space value in `address_space` is too large: `65536`
static TOO_BIG: usize = 7;

fn main() {}
