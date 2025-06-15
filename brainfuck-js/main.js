function brainfuck(program) {
    const loop_table = new Map();
    const loop_stack = [];

    for (let i = 0; i < program.length; i++) {
        if (program[i] === "[") {
            loop_stack.push(i);
        } else if (program[i] === "]") {
            const start = loop_stack.pop();
            loop_table.set(start, i);
            loop_table.set(i, start);
        }
    }

    let ip = 0;
    const tape = new Array(30000).fill(0);
    let cell_index = 0;
    let input_buffer = "";
    let input_index = 0;

    while (ip < program.length) {
        switch (program[ip]) {
            case ">":
                cell_index++;
                if (cell_index >= tape.length) tape.push(0);
                break;
            case "<":
                if (cell_index > 0) cell_index--;
                break;
            case "+":
                tape[cell_index] = (tape[cell_index] + 1) % 256;
                break;
            case "-":
                tape[cell_index] = (tape[cell_index] - 1 + 256) % 256;
                break;
            case ".":
                process.stdout.write(String.fromCharCode(tape[cell_index]));
                break;
            case ",":
                if (input_index >= input_buffer.length) {
                    input_buffer = prompt("Input: ") || "";
                    input_index = 0;
                }
                if (input_index < input_buffer.length) {
                    tape[cell_index] = input_buffer[input_index++].charCodeAt(
                        0,
                    );
                }
                break;
            case "[":
                if (tape[cell_index] === 0) {
                    ip = loop_table.get(ip);
                }
                break;
            case "]":
                if (tape[cell_index] !== 0) {
                    ip = loop_table.get(ip);
                }
                break;
        }
        ip++;
    }
}

function main() {
    console.log("========= Get your brains freaked ==========");
    const program =
        "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    brainfuck(program);
}

main();
