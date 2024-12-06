package main

import (
	"bufio"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

type rule struct {
	first int
	last  int
}

func (r rule) checkUpdate(pages []int) bool {
	lastFound := false

	for i := 0; i < len(pages); i++ {
		page := pages[i]

		if page == r.last {
			lastFound = true
		}

		if page == r.first {
			if lastFound {
				return false
			} else {
				return true
			}
		}
	}

	return true
}

func main() {
	file, err := os.Open("../input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	rules := []rule{}
	updates := [][]int{}

	rulesDone := false

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			rulesDone = true
			continue
		}

		if rulesDone {
			pagesString := strings.Split(line, ",")

			update := []int{}

			for _, s := range pagesString {
				page, _ := strconv.Atoi(s)
				update = append(update, page)
			}

			updates = append(updates, update)
		} else {
			split := strings.Split(line, "|")
			first, _ := strconv.Atoi(split[0])
			last, _ := strconv.Atoi(split[1])

			rules = append(rules, rule{
				first: first,
				last:  last,
			})
		}
	}

	incorrect := [][]int{}
	for _, update := range updates {
		correct := true

		for _, rule := range rules {
			correct = correct && rule.checkUpdate(update)
		}

		if !correct {
			incorrect = append(incorrect, update)
		}
	}

	for _, update := range incorrect {
		slices.SortStableFunc(update, func(x int, y int) int {
			// x < y neg
			for _, rule := range rules {
				if rule.first == x && rule.last == y {
					return -1
				}

				if rule.last == x && rule.first == y {
					return 1
				}
			}

			return 0
		})
	}

	middlePages := 0
	for _, update := range incorrect {
		middlePages += update[len(update)/2]
	}

	println(middlePages)
}
