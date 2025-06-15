def brainfuck(program):
    loop_table = {}
    loop_stack = []

    for ip, instruction in enumerate(program):
        if instruction == '[':
            loop_stack.append(ip)
        elif instruction == ']':
            start = loop_stack.pop()
            loop_table[ip] = start
            loop_table[start] = ip

    ip = 0
    tape = [0] * 30000
    cell_index = 0
    input_buffer = ""
    input_index = 0

    while ip < len(program):
        l = program[ip]
        if l == '>':
            cell_index += 1
            if cell_index >= len(tape):
                tape.append(0)
        elif l == '<':
            if cell_index > 0:
                cell_index -= 1
        elif l == '+':
            tape[cell_index] = (tape[cell_index] + 1) % 256
        elif l == '-':
            tape[cell_index] = (tape[cell_index] - 1) % 256
        elif l == '.':
            print(chr(tape[cell_index]), end="")
        elif l == ',':
            if input_index >= len(input_buffer):
                input_buffer = input()
                input_index = 0
            if input_index < len(input_buffer):
                tape[cell_index] = ord(input_buffer[input_index])
                input_index += 1
        elif l == '[':
            if tape[cell_index] == 0:
                ip = loop_table[ip]
        elif l == ']':
            if tape[cell_index] != 0:
                ip = loop_table[ip]
        ip += 1

if __name__ == "__main__":
    print("===========Get your brains fucked==========")
    program = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
    brainfuck(program)
