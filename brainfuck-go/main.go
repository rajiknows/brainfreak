package main

import "fmt"

func brainfuck(program string) {

	loopTable := make(map[int]int)
	loopStack := []int{}

}

func main() {
	fmt.Printf("==========Get your brains fucked==============")
	program := "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>."
	brainfuck(program)

}
