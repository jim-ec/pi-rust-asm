.intel_syntax noprefix
.global pi
.text

# Sum of fractions: xmm0
# Loop counter: rcx
#   Test rcx with 0x1.
# 4 constant: xmm4
# k: xmm1 (copy of rcx)
# rdi: exclusive max of k

pi:
    mov rcx, 4
    cvtsi2sd xmm4, rcx  # init c to 4.0
    xor rcx, rcx        # init k to 0
    
.loop:
    cmp rcx, rdi
    jge .end
    
    # Compute 2k + 1 -> xmm1
    lea r8, [rcx * 2 + 1]
    cvtsi2sd xmm1, r8
    
    movsd xmm8, xmm4
    divsd xmm8, xmm1
    
    # Check if add or sub
    test rcx, 1
    jnz .subtraction
    
    addsd xmm0, xmm8
    jmp .loop_end
    
.subtraction:

    subsd xmm0, xmm8

.loop_end:
    inc rcx
    jmp .loop

.end:
    ret
