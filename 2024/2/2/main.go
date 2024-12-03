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
		if checkSafe(reports[i], true) {
			numSafe += 1
		}
	}

	println(numSafe)
}

func checkSafe(report []int, canRemove bool) bool {
	if report[0] == report[1] {

		return canRemove && checkSafeSublists(report)
	}

	isDecreasing := report[1] < report[0]

	for i := 1; i < len(report); i++ {
		diff := report[i] - report[i-1]

		if isDecreasing && diff > 0 {
			return canRemove && checkSafeSublists(report)
		} else if !isDecreasing && diff < 0 {
			return canRemove && checkSafeSublists(report)
		}

		if diff < 0 {
			diff = -diff
		}

		if diff < 1 || diff > 3 {
			return canRemove && checkSafeSublists(report)
		}
	}

	return true
}

func checkSafeSublists(report []int) bool {
	for i := 0; i < len(report); i++ {
		newList := []int{}

		for j := 0; j < len(report); j++ {
			if j != i {
				newList = append(newList, report[j])
			}
		}

		if checkSafe(newList, false) {
			return true
		}
	}

	return false
}

func AbsInt(x int) int {
	if x < 0 {
		return -x
	} else {
		return x
	}
}
