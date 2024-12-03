package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("../input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	reports := [][]int{}

	for scanner.Scan() {
		words := strings.Fields(scanner.Text())
		report := []int{}

		for i := range words {
			word := words[i]

			converted, _ := strconv.Atoi(word)
			report = append(report, converted)
		}

		reports = append(reports, report)
	}

	numSafe := 0

	for i := range reports {
		if checkSafe(reports[i]) {
			numSafe += 1
		}
	}

	println(numSafe)
}

func checkSafe(report []int) bool {
	if report[0] == report[1] {
		return false
	}

	isDecreasing := report[1] < report[0]

	for i := 1; i < len(report); i++ {
		diff := report[i] - report[i-1]

		if isDecreasing && diff > 0 {
			return false
		} else if !isDecreasing && diff < 0 {
			return false
		}

		if diff < 0 {
			diff = -diff
		}

		if diff < 1 || diff > 3 {
			return false
		}
	}

	return true
}
