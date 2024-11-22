#![allow(unused)]

use std::arch::asm;

#[cfg(target_arch = "aarch64")]
fn pi(n: u64) -> f64 {
    let mut sum: f64 = 0.0;
    unsafe {
        asm!(
            "1:",
            // Exit loop if i >= n
            "cmp {i}, {n}",
            "bge 2f",

            // Compute 2i + 1:
            // - Shift i left by 1 to multiply by 2
            "lsl {term_int}, {i}, 1",
            // - Add 1 to the result
            "add {term_int}, {term_int}, 1",
            // - Convert the result to a floating-point number
            "scvtf {term:d}, {term_int}",

            // Compute 4.0 / (2i + 1)
            "fdiv {term:d}, {four:d}, {term:d}",

            // Alternate between adding and subtracting the term
            // - Extract LSB of i, if not set add the term, otherwise subtract it
            "and {lsb}, {i}, 1",
            // - Move the term into a scalar register
            "fmov {term_int}, {term:d}",
            // - Move the LSB to the sign bit position
            "bfi {term_int}, {lsb}, 63, 1",
            // - Move the term back to a SIMD register
            "fmov {term:d}, {term_int}",
            // - Add the possibly negated term to the sum
            "fadd {sum:d}, {sum:d}, {term:d}",

            // Increment i, jump back
            "add {i}, {i}, 1",
            "b 1b",

            "2:",

            n = in(reg) n,
            i = in(reg) 0_u64,
            lsb = out(reg) _,
            sum = inout(vreg) sum,
            four = in(vreg) 4.0,
            term_int = out(reg) _,
            term = out(vreg) _,
        )
    }
    sum
}

fn main() {
    dbg!(pi(0));
    dbg!(pi(1));
    dbg!(pi(2));
    dbg!(pi(3));
    dbg!(pi(4));
    dbg!(pi(10));
    dbg!(pi(100));
    dbg!(pi(1000));
    dbg!(pi(10000));
    dbg!(pi(100000));
    dbg!(pi(1000000));
    dbg!(pi(10000000));
}
