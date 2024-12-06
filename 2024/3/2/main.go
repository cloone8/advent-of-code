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
	file, err := os.Open("../test.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	r, _ := regexp.Compile(`mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)`)

	sum := 0
	mulEnabled := true

	for scanner.Scan() {
		matches := r.FindAllString(scanner.Text(), -1)

		if matches == nil {
			continue
		}

		for i := range matches {
			match := matches[i]

			if match == "do()" {
				mulEnabled = true
			} else if match == "don't()" {
				mulEnabled = false
			} else if mulEnabled {
				sum += parseMul(match).Do()
			}
		}
	}

	println(sum)
}
