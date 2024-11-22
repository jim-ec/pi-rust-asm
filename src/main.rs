#![allow(unused)]

use std::arch::asm;

#[cfg(target_arch = "aarch64")]
fn pi(n: u64) -> f64 {
    let mut sum: f64 = 0.0;
    unsafe {
        asm!(
            "1:",
            // Loop exit condition
            "cmp {i}, {n}",
            "bge 4f",

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
            // - Extract LSB of i
            "and {lsb}, {i}, 1",
            // - Compare LSB to zero
            "cmp {lsb}, 0",
            // - If LSB is not zero, LSB is set to the sign-bit mask, otherwise it is zero
            "csel {lsb}, {mask}, xzr, ne",
            // - Move LSB to a SIMD register
            "fmov {tmp:d}, {lsb}",
            // - Xor the term with the sign-bit mask or zero to flip the sign bit if necessary
            "eor {term:v}.16b, {term:v}.16b, {tmp:v}.16b",
            // - Add the possibly negated term to the sum
            "fadd {sum:d}, {sum:d}, {term:d}",

            // End of loop, increment i
            "3:",
            "add {i}, {i}, 1",
            "b 1b",

            // End of iteration
            "4:",

            n = in(reg) n,
            i = in(reg) 0_u64,
            lsb = out(reg) _,
            mask = in(reg) 0x8000000000000000_u64,
            sum = inout(vreg) sum,
            tmp = out(vreg) _,
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
