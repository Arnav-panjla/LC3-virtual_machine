; test.asm
.ORIG x3000
AND R0, R0, #0    ; Clear R0
ADD R0, R0, #5    ; R0 = R0 + 5
TRAP x25          ; HALT
.END

