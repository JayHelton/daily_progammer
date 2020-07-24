package main

import (
	"fmt"
	"strings"
)

func main() {
	fmt.Println(balanced("x"))
}

func balanced(x string) bool {
	isBalanced := true
	m := make(map[string]int)
	characters := strings.Split(x, "")

	for _, c := range characters {
		if val, ok := m[c]; ok {
			m[c] = val + 1
		} else {
			m[c] = 1
		}
	}

	if len(m) == 1 {
		return false
	}
	for _, v := range m {
		for _, vo2 := range m {
			isBalanced = v == vo2

			if !isBalanced {
				break
			}
		}
	}

	return isBalanced
}
