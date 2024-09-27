def AND(a, b):
    """
    Computes the logical AND of two binary values.

    Parameters:
    a (int): First binary input (0 or 1).
    b (int): Second binary input (0 or 1).

    Returns:
    int: Result of logical AND operation (1 if both a and b are 1, otherwise 0).
    """
    return int(a and b)


def OR(a, b):
    """
    Computes the logical OR of two binary values.

    Parameters:
    a (int): First binary input (0 or 1).
    b (int): Second binary input (0 or 1).

    Returns:
    int: Result of logical OR operation (1 if either a or b is 1, otherwise 0).
    """
    return int(a or b)


def NOT(a):
    """
    Computes the logical NOT of a binary value.

    Parameters:
    a (int): Binary input (0 or 1).

    Returns:
    int: Result of logical NOT operation (1 if a is 0, otherwise 0).
    """
    return int(not a)


def NAND(a, b):
    """
    Computes the logical NAND of two binary values.

    Parameters:
    a (int): First binary input (0 or 1).
    b (int): Second binary input (0 or 1).

    Returns:
    int: Result of logical NAND operation (0 if both a and b are 1, otherwise 1).
    """
    return int(not (a and b))


def XOR(a, b):
    """
    Computes the logical XOR of two binary values.

    Parameters:
    a (int): First binary input (0 or 1).
    b (int): Second binary input (0 or 1).

    Returns:
    int: Result of logical XOR operation (1 if a and b are different, otherwise 0).
    """
    return int(a != b)


def MUX(a, b, sel):
    """
    Implements a Multiplexor (MUX).

    Parameters:
    a (int): First input (0 or 1).
    b (int): Second input (0 or 1).
    sel (int): Selector input (0 or 1).

    Returns:
    int: Output of the MUX (a if sel is 0, b if sel is 1).
    """
    # Calculate the output based on the selector
    if sel == 0:
        return a
    else:
        return b
    
def DMUX(input, sel):
    """
    Implements a Demultiplexor (DMUX).

    Parameters:
    input (int): Input to be demultiplexed (0 or 1).
    sel (int): Selector input (0 or 1).

    Returns:
    tuple: A tuple containing the two outputs of the DMUX (input if sel is 0, 0 if sel is 1).
    """
    # Calculate the outputs based on the selector
    if sel == 0:
        return input, 0
    else:
        return 0, input


if __name__ == '__main__':
    print('-------------------------------------------------------------')
    print("Basic logic test")
    print('-------------------------------------------------------------')

    print('Test AND')
    print(f'AND(0, 0) = {AND(0, 0)}')
    print(f'AND(0, 1) = {AND(0, 1)}')
    print(f'AND(1, 0) = {AND(1, 0)}')
    print(f'AND(1, 1) = {AND(1, 1)}')
    print('-------------------------------------------------------------')

    print('Test OR')
    print(f'OR(0, 0) = {OR(0, 0)}')
    print(f'OR(0, 1) = {OR(0, 1)}')
    print(f'OR(1, 0) = {OR(1, 0)}')
    print(f'OR(1, 1) = {OR(1, 1)}')
    print('-------------------------------------------------------------')

    print('Test NOT')
    print(f'NOT(0) = {NOT(0)}')
    print(f'NOT(1) = {NOT(1)}')
    print('-------------------------------------------------------------')

    print('Test NAND')
    print(f'NAND(0, 0) = {NAND(0, 0)}')
    print(f'NAND(0, 1) = {NAND(0, 1)}')
    print(f'NAND(1, 0) = {NAND(1, 0)}')
    print(f'NAND(1, 1) = {NAND(1, 1)}')
    print('-------------------------------------------------------------')

    print('Test XOR')
    print(f'XOR(0, 0) = {XOR(0, 0)}')
    print(f'XOR(0, 1) = {XOR(0, 1)}')
    print(f'XOR(1, 0) = {XOR(1, 0)}')
    print(f'XOR(1, 1) = {XOR(1, 1)}')
    print('-------------------------------------------------------------')
    print("Multiplexor Test")
    print(f'MUX(0, 0, 0) = {MUX(0, 0, 0)}')  # Output: 0
    print(f'MUX(0, 1, 0) = {MUX(0, 1, 0)}')  # Output: 0
    print(f'MUX(1, 0, 0) = {MUX(1, 0, 0)}')  # Output: 1
    print(f'MUX(1, 1, 0) = {MUX(1, 1, 0)}')  # Output: 1
    print(f'MUX(0, 0, 1) = {MUX(0, 0, 1)}')  # Output: 0
    print(f'MUX(0, 1, 1) = {MUX(0, 1, 1)}')  # Output: 1
    print(f'MUX(1, 0, 1) = {MUX(1, 0, 1)}')  # Output: 0
    print(f'MUX(1, 1, 1) = {MUX(1, 1, 1)}')  # Output: 1
    print('-------------------------------------------------------------')
    print("Demultiplexor Test")
    print(f'DMUX(0, 0) = {DMUX(0, 0)}')  # Output: (0, 0)
    print(f'DMUX(0, 1) = {DMUX(0, 1)}')  # Output: (0, 0)
    print(f'DMUX(1, 0) = {DMUX(1, 0)}')  # Output: (1, 0)
    print(f'DMUX(1, 1) = {DMUX(1, 1)}')  # Output: (0, 1)
    print('-------------------------------------------------------------')