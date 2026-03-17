# LMC Simulator

## Specs

Ram consists of a 128 size array of i16 values

## Instructions

| Instruction   | Description                                                           |
| ------------- | --------------------------------------------------------------------- |
| Add ...       | Adds value in the accumulator                                         |
| Sub ...       | Subtracts value in the accumulator                                    |
| Lda ...       | Load from memory address to accumulator                               |
| Sta ...       | Store accumulator value at memory adress                              |
| INP           | Gets user input and stores in the accumulator                         |
| OUT           | Prints the value in the acumulator                                    |
| BRP ...       | Branches to instruction number if value in accumulator is positive    |
| BRZ ...       | Branches to instruction number if value in accumulator is zero        |
| BRA ...       | Always branches to instruction number                                 |
| HLT           | Stops the program                                                     |


## Addressing Modes

| Mode          | Description                                                   |   
| ------------- | ------------------------------------------------------------- |
| Immediate (#) | uses value provided                                           |
| Direct    (&) | uses value in memory address                                  |
| Indirect  (@) | uses value in memory address to point to used memory adress   |
| Indexed (TODO)| TODO!                                                         |

