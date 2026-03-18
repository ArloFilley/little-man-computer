# LMC Simulator
## Specs

Memory consists of a 100 size array of i16 values

## Instructions

| Instruction   | Description                                                            |
| ------------- | ---------------------------------------------------------------------- |
| ADD xx/...    | Subtracts value at memory address xx or label ... from the accumulator |
| ADDIMM xx     | Adds the value xx to the accumulator                                   |
| SUB xx/...    | Subtracts value at memory address xx or label ... from the accumulator |
| SUBIMM xx     | Subtracts the vaule xx from the accumulator                            |
| LDA xx/...    | Load from memory address to accumulator                                |
| STA xx/...    | Store accumulator value at memory adress                               |
| INP           | Gets user input and stores in the accumulator                          |
| OUT           | Prints the value in the acumulator                                     |
| BRP ...       | Branches to location ... if value in accumulator is positive           |
| BRZ ...       | Branches to location ... if value in accumulator is zero               |
| BRA ...       | Always branches to location ...                                        |
| HLT           | Halts the program                                                      |
| ... DAT xx    | Creates a memory address named ... optionally storing value xx         |
****