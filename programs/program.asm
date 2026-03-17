ADD #127
STA 0
INP
STA 1
// Gets Initial Values

// Loads counter, outputs value and appends it to the back of ram
LDA 1
STA @0
OUT
SUB #1

STA 1
// Decrements the indirect address
LDA 0
SUB #1
STA 0

// Loops Unless the Counter is 0
LDA 1
BRZ 15
BRA 4
HLT