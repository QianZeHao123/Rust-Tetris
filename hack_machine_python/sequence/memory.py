class FlipFlop:
    def __init__(self):
        self.state = 0  # Initial state of the flip-flop is 0

    def clock(self, d):
        """
        This method simulates a clock cycle.
        d: The input to the flip-flop (0 or 1).
        """
        output = self.state  # The output is the current state
        self.state = d       # Update the state to the input
        return output


class OneBitRegister:
    def __init__(self):
        self.flip_flop = FlipFlop()  # A 1-bit register has a single flip-flop

    def update(self, d, load):
        """
        This method simulates a clock cycle.
        d: The input to the register (0 or 1).
        load: The load input to the register (0 or 1).
        """
        if load == 1:
            # when load is 1, update the state
            output = self.flip_flop.clock(d)
        else:
            output = self.flip_flop.clock(
                self.flip_flop.state)  # when load is 0, keep the state
        return output


class Register:
    def __init__(self):
        # Initialize 16 one-bit flip-flops to store each bit
        # self.bits = [FlipFlop() for _ in range(16)]
        self.bits = [OneBitRegister() for _ in range(16)]

    def update(self, data, load):
        """
        Update the state of the 16-bit register.
        data: A list of 16 bits (0s and 1s) representing the input data.
        load: The load signal (0 or 1); when 1, the register updates with the input data.
        """
        output = []
        for i in range(16):
            # Update each OneBitRegister with corresponding bit from data
            output.append(self.bits[i].update(data[i], load))
        return output

class RAM8:
    def __init__(self):
        # Initialize 8 registers to store 16-bit data each
        self.registers = [Register() for _ in range(8)]

    def update(self, data, address, load):
        """Update or retrieve the state of the RAM8.
        data: A list of 16 bits (0s and 1s) representing the input data.
        address: A list of 3 bits (0s and 1s) representing the register address (0 to 7).
        load: The load signal (0 or 1); when 1, the addressed register updates with the input data.
        """
        # Convert the 3-bit binary address to a decimal index
        address_index = address[0] * 4 + address[1] * 2 + address[2]
        
        output = []
        for i in range(8):
            if i == address_index:
                # Update only the selected register based on the load signal
                output = self.registers[i].update(data, load)
            else:
                # For non-selected registers, just retrieve their current state
                self.registers[i].update(self.registers[i].update([0]*16, 0), 0)
        
        return output

class RAM64:
    def __init__(self):
        # Initialize 8 RAM8 components
        self.ram8s = [RAM8() for _ in range(8)]

    def update(self, data, address, load):
        """Update or retrieve the state of the RAM64.
        data: A list of 16 bits (0s and 1s) representing the input data.
        address: A list of 6 bits (0s and 1s) representing the register address (0 to 63).
        load: The load signal (0 or 1); when 1, the addressed register updates with the input data.
        """
        # Split the 6-bit address into two parts
        ram8_index = address[0] * 4 + address[1] * 2 + address[2]  # Select which RAM8 (3 bits)
        inner_address = address[3:]  # Select the register within RAM8 (3 bits)

        output = []
        for i in range(8):
            if i == ram8_index:
                output = self.ram8s[i].update(data, inner_address, load)
            else:
                self.ram8s[i].update([0]*16, inner_address, 0)  # Ensure non-selected RAM8 stays unchanged
        
        return output
    
class RAM512:
    def __init__(self):
        # Initialize 8 RAM64 components
        self.ram64s = [RAM64() for _ in range(8)]

    def update(self, data, address, load):
        """Update or retrieve the state of the RAM512.
        data: A list of 16 bits (0s and 1s) representing the input data.
        address: A list of 9 bits (0s and 1s) representing the register address (0 to 511).
        load: The load signal (0 or 1); when 1, the addressed register updates with the input data.
        """
        # Split the 9-bit address into two parts
        ram64_index = address[0] * 4 + address[1] * 2 + address[2]  # Select which RAM64 (3 bits)
        inner_address = address[3:]  # Select the register within RAM64 (6 bits)

        output = []
        for i in range(8):
            if i == ram64_index:
                output = self.ram64s[i].update(data, inner_address, load)
            else:
                self.ram64s[i].update([0]*16, inner_address, 0)  # Ensure non-selected RAM64 stays unchanged
        
        return output

class RAM4K:
    def __init__(self):
        # Initialize 8 RAM512 components
        self.ram512s = [RAM512() for _ in range(8)]

    def update(self, data, address, load):
        """Update or retrieve the state of the RAM4K.
        data: A list of 16 bits (0s and 1s) representing the input data.
        address: A list of 12 bits (0s and 1s) representing the register address (0 to 4095).
        load: The load signal (0 or 1); when 1, the addressed register updates with the input data.
        """
        # Split the 12-bit address into two parts
        ram512_index = address[0] * 4 + address[1] * 2 + address[2]  # Select which RAM512 (3 bits)
        inner_address = address[3:]  # Select the register within RAM512 (9 bits)

        output = []
        for i in range(8):
            if i == ram512_index:
                output = self.ram512s[i].update(data, inner_address, load)
            else:
                self.ram512s[i].update([0]*16, inner_address, 0)  # Ensure non-selected RAM512 stays unchanged
        
        return output


if __name__ == '__main__':
    # Flip-flop test
    print('-------------------------------------------------------------')
    print("Flip-flop test")
    ff = FlipFlop()
    print(ff.clock(0))  # Output is 0
    print(ff.clock(1))  # Output is 0
    print(ff.clock(1))  # Output is 1
    print(ff.clock(0))  # Output is 1
    print(ff.clock(0))  # Output is 1
    print(ff.clock(0))  # Output is 1
    print(ff.clock(0))  # Output is 1
    # 1-bit register test
    print('-------------------------------------------------------------')
    print("1-bit register test")
    reg = OneBitRegister()
    # Output: 0，the initial state is 0, update the state to 1
    print(reg.update(1, 1))
    print(reg.update(0, 0))  # Output: 1, load is 0, keep the current state
    print(reg.update(0, 1))  # Output: 1, load is 1, update the state to 0
    print(reg.update(1, 0))  # Output: 0, load is 0, keep the current state
    # 16-bit register test
    print('-------------------------------------------------------------')
    print("16-bit register test")
    reg = Register()
    print(reg.update([1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], 1))
    print(reg.update([0]*16, 0))
    print(reg.update([0]*16, 1))
    print(reg.update([1]*16, 1))
    print(reg.update([1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1], 1))
    print(reg.update([0]*16, 0))
    print(reg.update([0]*16, 0))
    # RAM8 test
    print('-------------------------------------------------------------')
    print("RAM8 test")
    ram8 = RAM8()
    print(ram8.update([1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0], 1))  # Store 16 ones at address 0
    print(ram8.update([1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1], [0, 0, 0], 0))  # Store 16 ones at address 0
    print(ram8.update([0]*16, [0, 0, 1], 1))  # Store 16 zeros at address 1
    print(ram8.update([1]*16, [0, 0, 0], 0))  # Should output the previous value stored in address 0 (16 ones)
    print(ram8.update([0]*16, [0, 0, 1], 0))  # Should output the previous value stored in address 1 (16 zeros)
    print(ram8.update([1]*16, [0, 0, 0], 0))  # Should output the previous value stored in address 0 (16 ones)
    print(ram8.update([1]*16, [0, 0, 0], 1))  # Should output the previous value stored in address 0 (16 ones)
    print(ram8.update([1]*16, [0, 0, 0], 0))  # Should output the previous value stored in address 0 (16 ones)
