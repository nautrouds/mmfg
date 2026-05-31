//go:build arm64

TEXT ·Procyield(SB),7,$0
    MOVL n+0(FP), R0
again:
    YIELD
    SUB $1, R0
    CBNZ R0, again
    RET
