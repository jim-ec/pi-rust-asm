use std::os::raw::{c_double, c_long};

extern "C" {
    fn pi(x: c_long) -> c_double;
}

fn main() {
    dbg!(unsafe { pi(1000) });
}
