package main

import (
	"bufio"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type mul struct {
	x int
	y int
}

func (m mul) Do() int {
	return m.x * m.y
}

func parseMul(s string) mul {
	noMul := strings.ReplaceAll(strings.ReplaceAll(s, "mul(", ""), ")", "")

	nums := strings.Split(noMul, ",")

	x, _ := strconv.Atoi(nums[0])
	y, _ := strconv.Atoi(nums[1])

	return mul{
		x: x,
		y: y,
	}
}

func main() {
	file, err := os.Open("../input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	muls := []mul{}

	r, _ := regexp.Compile(`mul\(\d{1,3},\d{1,3}\)`)

	for scanner.Scan() {
		matches := r.FindAllString(scanner.Text(), -1)

		if matches == nil {
			continue
		}

		for i := range matches {
			muls = append(muls, parseMul(matches[i]))
		}
	}

	sum := 0

	for i := range muls {
		sum += muls[i].Do()
	}

	println(sum)
}
