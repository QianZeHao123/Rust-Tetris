'''
ALU: Algebraic Logic Unit
This module is a part of the combination logic circuit.
It performs arithmetic and logical operations on two 16-bit inputs.
The ALU has two 16-bit inputs, A and B, and a 6-bit control input (zx, nx, zy, ny, f, no).
And the ALU has one 16-bit output, and two 1-bit outputs (zr, ng).


ALU (Arithmetic Logic Unit):
Computes out = one of the following functions:
               0, 1, -1,
               x, y, !x, !y, -x, -y,
               x + 1, y + 1, x - 1, y - 1,
               x + y, x - y, y - x,
               x & y, x | y
on the 16-bit inputs x, y,
according to the input bits zx, nx, zy, ny, f, no.
In addition, computes the two output bits:
    if (out == 0) zr = 1, else zr = 0
    if (out < 0)  ng = 1, else ng = 0
Implementation: Manipulates the x and y inputs
and operates on the resulting values, as follows:
    if (zx == 1) sets x = 0        // 16-bit constant
    if (nx == 1) sets x = !x       // bitwise not
    if (zy == 1) sets y = 0        // 16-bit constant
    if (ny == 1) sets y = !y       // bitwise not
    if (f == 1)  sets out = x + y  // integer 2's complement addition
    if (f == 0)  sets out = x & y  // bitwise and
    if (no == 1) sets out = !out   // bitwise not
'''


def add16(x, y):
    """
    Adds two 16-bit values x and y.

    Parameters:
    x (list): 16-bit input value x (0s and 1s).
    y (list): 16-bit input value y (0s and 1s).

    Returns:
    list: 16-bit output value (sum of x and y).
    """
    # Initialize the output and carry variables
    out = [0] * 16
    carry = 0

    # Iterate over the bits in reverse order
    for i in range(15, -1, -1):
        # Calculate the sum of the current bits and the carry
        s = x[i] + y[i] + carry

        # Update the output bit and carry
        out[i] = s % 2
        carry = s // 2

    return out


def ALU(x, y, zx, nx, zy, ny, f, no):
    """
    Implements the Arithmetic Logic Unit (ALU).

    Parameters:
    x (list): 16-bit input value x (0s and 1s).
    y (list): 16-bit input value y (0s and 1s).
    zx (int): Zero-X input (0 or 1).
    nx (int): Not-X input (0 or 1).
    zy (int): Zero-Y input (0 or 1).
    ny (int): Not-Y input (0 or 1).
    f (int): Function input (0 or 1).
    no (int): Not-Output input (0 or 1).

    Returns:
    tuple: A tuple containing the 16-bit output, and the two output bits (zr, ng).
    """
    # Initialize the output and the 16-bit inputs
    out = [0] * 16
    x_val = x
    y_val = y

    # Apply the zx and nx operations to x
    if zx == 1:
        x_val = [0] * 16
    if nx == 1:
        x_val = [int(not i) for i in x_val]

    # Apply the zy and ny operations to y
    if zy == 1:
        y_val = [0] * 16
    if ny == 1:
        y_val = [int(not i) for i in y_val]

    # Perform the arithmetic operations based on the function input
    if f == 1:
        out = add16(x_val, y_val)
    else:
        out = [x & y for x, y in zip(x_val, y_val)]

    # Apply the no operation to the output
    if no == 1:
        out = [int(not i) for i in out]

    # Compute the zr and ng output bits
    zr = 1 if all(i == 0 for i in out) else 0
    ng = out[0]

    return out, zr, ng


def test_ALU():
    test_cases = [
        # Add your test cases here
        {"x": [0] * 16, "y": [1] * 16, "zx": 1, "nx": 0, "zy": 1, "ny": 0, "f": 1, "no": 0, "expected_out": [0] * 16, "expected_zr": 1, "expected_ng": 0},
        {"x": [0] * 16, "y": [1] * 16, "zx": 1, "nx": 1, "zy": 1, "ny": 1, "f": 1, "no": 1, "expected_out": [0] * 15 + [1], "expected_zr": 0, "expected_ng": 0},
        {"x": [0] * 16, "y": [1] * 16, "zx": 1, "nx": 1, "zy": 0, "ny": 0, "f": 1, "no": 0, "expected_out": [1] * 16, "expected_zr": 0, "expected_ng": 1},
        {"x": [0] * 16, "y": [1] * 16, "zx": 0, "nx": 0, "zy": 1, "ny": 1, "f": 0, "no": 0, "expected_out": [0] * 16, "expected_zr": 1, "expected_ng": 0},
        {"x": [0] * 16, "y": [1] * 16, "zx": 1, "nx": 1, "zy": 0, "ny": 0, "f": 0, "no": 0, "expected_out": [1] * 16, "expected_zr": 0, "expected_ng": 1},
        # Additional test cases omitted for brevity
    ]

    for idx, case in enumerate(test_cases):
        result_out, result_zr, result_ng = ALU(case["x"], case["y"], case["zx"], case["nx"], case["zy"], case["ny"], case["f"], case["no"])
        
        # Check if the output matches the expected values
        if result_out == case["expected_out"] and result_zr == case["expected_zr"] and result_ng == case["expected_ng"]:
            print(f"Test case {idx + 1}: Passed")
        else:
            print(f"Test case {idx + 1}: Failed")
            print(f"Expected out: {case['expected_out']}, zr: {case['expected_zr']}, ng: {case['expected_ng']}")
            print(f"Received out: {result_out}, zr: {result_zr}, ng: {result_ng}")

if __name__ == "__main__":
    test_ALU()
