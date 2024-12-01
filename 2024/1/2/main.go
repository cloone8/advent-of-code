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

	list1 := []int{}
	list2 := []int{}

	for scanner.Scan() {
		words := strings.Fields(scanner.Text())
		l1, _ := strconv.Atoi(words[0])

		l2, _ := strconv.Atoi(words[1])

		list1 = append(list1, l1)
		list2 = append(list2, l2)
	}

	totalSimScore := 0

	for i := 0; i < len(list1); i++ {
		simScore := 0

		for j := 0; j < len(list2); j++ {
			if list1[i] == list2[j] {
				simScore += list1[i]
			}
		}

		totalSimScore += simScore
	}

	println(totalSimScore)
}
