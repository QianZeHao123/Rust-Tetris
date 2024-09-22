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
