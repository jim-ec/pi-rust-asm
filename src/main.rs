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

            // Compute 2i + 1
            "lsl {term_int}, {i}, 1",
            "add {term_int}, {term_int}, 1",
            "scvtf {term:d}, {term_int}",

            // Compute 4.0 / (2i + 1)
            "fdiv {term:d}, {four:d}, {term:d}",

            // Extract LSB of i
            "and {lsb}, {i}, 1",
            "csel {lsb}, {mask}, xzr, ne",

            // Test if i is even or odd
            "and {lsb}, {i}, 1",
            "cbnz {lsb}, 2f",

            "fadd {sum:d}, {sum:d}, {term:d}",
            "b 3f",

            "2:",
            "fsub {sum:d}, {sum:d}, {term:d}",

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
