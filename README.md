# Rust-Tetris
From Nand to Tetris -- Rust Implementation

* Hack Machine
* Machine Language
* Assembler

## 1.1 Hardware Description Language Input and Output

### 1.1.1 Combinational Logic

#### Nand Gate

| Input A | Input B | Output (A NAND B) |
|---------|---------|-------------------|
| 0       | 0       | 1                 |
| 0       | 1       | 1                 |
| 1       | 0       | 1                 |
| 1       | 1       | 0                 |

#### Not Gate

```
output = nand(input, input)
```

#### Or Gate

```
output = nand(not(inputA), not(inputB))
```

#### And Gate

```
output = not(nand(inputA, inputB))
```

#### Mux Gate

| Input A | Input B | sel | Output |
|---|---|-----|-----|
| 0 | 0 | 0   | 0   |
| 0 | 0 | 1   | 0   |
| 0 | 1 | 0   | 0   |
| 0 | 1 | 1   | 1   |
| 1 | 0 | 0   | 1   |
| 1 | 0 | 1   | 0   |
| 1 | 1 | 0   | 1   |
| 1 | 1 | 1   | 1   |

```
output = or(and(a, not(sel)), and(b, sel))
```
#### DMux Gate

| Input | sel | Output A | Output B |
|----|-----|---|---|
| 0  | 0   | 0 | 0 |
| 0  | 1   | 0 | 0 |
| 1  | 0   | 1 | 0 |
| 1  | 1   | 0 | 1 |

```
OutputA = and(not(sel), in)
OutputB = and(sel, in)
```

#### Or16, And16 and Not16

```
Or16: output[0..15] = not(inputA[0..15], inputB[0..15])
And16: output[0..15] = not(inputA[0..15], inputB[0..15])
Not16: output[0..15] = not(inputA[0..15], inputB[0..15])
```

#### Or8Way

```
Or(
    Or(Or(Input1, Input2), Or(Input3, Input4)),
    Or(Or(Input5, Input6), Or(Input7, Input8))
   )
```

#### Mux16

```
Output[0..15] = Mux(inputA[0..15], inputB[0..15], sel)
```

#### Mux4Way16, Mux8Way16

Mux4Way16 Ouput is equal to:
* InputA if sel = 00
* InputB if sel = 01
* InputC if sel = 10
* InputD if sel = 11

Mux8Way16 Ouput is equal to:
* InputA if sel = 000
* InputB if sel = 001
* InputC if sel = 010
* InputD if sel = 011
* InputE if sel = 100
* InputF if sel = 101
* InputG if sel = 110
* InputH if sel = 111

#### Dmux4Way, DMux8Way

DMux4Way Output(Output[0..3]) is equal to:
* [in, 0, 0, 0] if sel = 00
* [0, in, 0, 0] if sel = 01
* [0, 0, in, 0] if sel = 10
* [0, 0, 0, in] if sel = 11

DMux8Way Output(Output[0..7]) is equal to:
* [in, 0, 0, 0, 0, 0, 0, 0] if sel = 000
* [0, in, 0, 0, 0, 0, 0, 0] if sel = 001
* [0, 0, in, 0, 0, 0, 0, 0] if sel = 010
* [0, 0, 0, in, 0, 0, 0, 0] if sel = 011
* [0, 0, 0, 0, in, 0, 0, 0] if sel = 100
* [0, 0, 0, 0, 0, in, 0, 0] if sel = 101
* [0, 0, 0, 0, 0, 0, in, 0] if sel = 110
* [0, 0, 0, 0, 0, 0, 0, in] if sel = 111

### 1.1.2 Adder and ALU

### 1.1.3 Sequential Logic

### 1.1.4 CPU and Memory

### 1.1.5 Computer