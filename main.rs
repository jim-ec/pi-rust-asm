use std::os::raw::{*};

extern "C" {
    fn pi(x: c_long) -> c_double;
}


fn main() {
    let p = unsafe { pi(10000000) };
    println!("pi ~= {}", p);
}
