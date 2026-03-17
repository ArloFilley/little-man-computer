ADD #127    // Points to back of ram
STA 0       // Stores Address: Address 0
STA 1       // Stores Address for output: Address 2

INP         // Gets Input
STA 2       // Stores Count: Address 1

// Loop Begins Here
LDA 2
STA @0
SUB #1
STA 2

LDA 0
SUB #1
STA 0

LDA 2
BRP 6

// Second Loop
LDA @1
LDA 1
SUB #1
STA 1
LDA @1
BRP 14

HLT         // Stop Program