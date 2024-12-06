package main

import (
	"bufio"
	"log"
	"os"
)

func main() {
	grid := [][]rune{}

	file, err := os.Open("../input.txt")

	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := []rune(scanner.Text())
		grid = append(grid, line)
	}

	x_max := len(grid[0])
	y_max := len(grid)

	numFound := 0

	for x := 1; x < (x_max - 1); x++ {
		for y := 1; y < (y_max - 1); y++ {
			if grid[y][x] != 'A' {
				continue
			}

			if isXmas(x, y, grid) {
				numFound += 1
			}
		}
	}

	println(numFound)
}

func isXmas(x int, y int, grid [][]rune) bool {
	a := (grid[y-1][x-1] == 'M' && grid[y+1][x+1] == 'S') || (grid[y-1][x-1] == 'S' && grid[y+1][x+1] == 'M')
	b := (grid[y-1][x+1] == 'M' && grid[y+1][x-1] == 'S') || (grid[y-1][x+1] == 'S' && grid[y+1][x-1] == 'M')
	return a && b
}
