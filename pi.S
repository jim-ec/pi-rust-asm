.global _pi
.text

# Registers:
#   x0: Number of iterations
#   x1: Loop counter
#   d0: Running sum of fractions and return value
#   d4: Constant 4.0

_pi:
    mov x1, #0           // Initialize k to 0
    fmov d0, #0.0        // Initialize sum to 0.0
    fmov d4, #4.0        // Initialize constant c to 4.0

.loop:
    cmp x1, x0           // Compare loop counter (k) with the limit
    bge .end             // Exit loop if k >= rdi (exclusive max)

    # Compute 2k + 1 -> x2, convert to double -> d1
    add x2, x1, x1       // x2 = 2k
    add x2, x2, #1       // x2 = 2k + 1
    scvtf d1, x2         // Convert integer (2k + 1) to double in d1

    fmov d8, d4          // Move the constant 4.0 into d8
    fdiv d8, d4, d1      // Divide 4.0 by (2k + 1), result in d8

    # Check if add or subtract
    and x3, x1, #1       // Test if k is odd or even
    cbnz x3, .subtraction // Branch to subtraction if k is odd

    fadd d0, d0, d8      // Add d8 to sum
    b .loop_end

.subtraction:
    fsub d0, d0, d8      // Subtract d8 from sum

.loop_end:
    add x1, x1, #1       // Increment loop counter k
    b .loop

.end:
    ret
