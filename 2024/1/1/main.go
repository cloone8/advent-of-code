package main

import (
	"bufio"
	"log"
	"os"
	"sort"
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

	sort.Ints(list1)
	sort.Ints(list2)

	sum := 0
	for i := 0; i < len(list1); i++ {
		distance := list1[i] - list2[i]
		if distance < 0 {
			distance = -distance
		}

		sum += distance
	}

	println(sum)
}
