def read_cmp_file(file_path):
    test_cases = []
    
    with open(file_path, 'r', encoding='utf-8') as file:
        lines = file.readlines()
        
        # Skip the first header line
        for line in lines[1:]:
            line = line.strip()
            if line:  # Skip empty lines
                columns = [col.strip() for col in line.split('|')[1:-1]]
                
                # Convert binary strings to lists of integers
                x = [int(bit) for bit in columns[0]]
                y = [int(bit) for bit in columns[1]]
                zx = int(columns[2])
                nx = int(columns[3])
                zy = int(columns[4])
                ny = int(columns[5])
                f = int(columns[6])
                no = int(columns[7])
                expected_out = [int(bit) for bit in columns[8]]
                expected_zr = int(columns[9])
                expected_ng = int(columns[10])
                
                # Append the formatted dictionary to the test cases list
                test_cases.append({
                    "x": x,
                    "y": y,
                    "zx": zx,
                    "nx": nx,
                    "zy": zy,
                    "ny": ny,
                    "f": f,
                    "no": no,
                    "expected_out": expected_out,
                    "expected_zr": expected_zr,
                    "expected_ng": expected_ng
                })
    
    return test_cases


if __name__ == '__main__':
    # Usage
    file_path = 'ALU.cmp'  # Replace this with the path to your test.cmp file
    data = read_cmp_file(file_path)
    # display_data(header, data)
    print(data[0])