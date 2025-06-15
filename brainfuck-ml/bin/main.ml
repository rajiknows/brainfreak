
let build_loop_table program =
    let table = Hashtbl.create 16 in
    let stack = Stack.create () in
    let len = String.length program in
    for i = 0 to len - 1 do
        match program.[i] with
        | '[' -> Stack.push i stack
        | ']' ->
            let j = Stack.pop stack in
            Hashtbl.add table i j;
            Hashtbl.add table j i
        | _ -> ()
    done;
    table

let () =
    let program =
        "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
    in
    let loop_table = build_loop_table program in
    let tape = Array.make 30000 0 in
    let cell = ref 0 in
    let ip = ref 0 in
    let prog_len = String.length program in
    let input_buffer = ref "" in
    let input_index = ref 0 in

    while !ip < prog_len do
        let c = program.[!ip] in
        begin match c with
            | '>' -> incr cell
            | '<' -> decr cell
            | '+' -> tape.(!cell) <- (tape.(!cell) + 1) mod 256
            | '-' -> tape.(!cell) <- (tape.(!cell) - 1 + 256) mod 256
            | '.' -> print_char (Char.chr tape.(!cell))
            | ',' ->
                if !input_index >= String.length !input_buffer then (
                    flush stdout;
                    input_buffer := read_line ();
                    input_index := 0;
                );
                tape.(!cell) <- Char.code !input_buffer.[!input_index];
                incr input_index
            | '[' ->
                if tape.(!cell) = 0 then
                ip := Hashtbl.find loop_table !ip
            | ']' ->
                if tape.(!cell) <> 0 then
                ip := Hashtbl.find loop_table !ip
            | _ -> ()
            end;
        incr ip
    done
