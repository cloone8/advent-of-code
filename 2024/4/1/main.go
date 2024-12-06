package main

import (
	"bufio"
	"log"
	"os"
)

type checker struct {
	dx int
	dy int

	x    int
	y    int
	grid [][]rune
}

func (c checker) runCheck() bool {
	x := c.x
	y := c.y
	dx := c.dx
	dy := c.dy

	return c.grid[y+dy][x+dx] == 'M' && c.grid[y+(2*dy)][x+(2*dx)] == 'A' && c.grid[y+(3*dy)][x+(3*dx)] == 'S'
}

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

	checkers := []checker{}

	x_max := len(grid[0])
	y_max := len(grid)

	for x := 0; x < x_max; x++ {
		for y := 0; y < y_max; y++ {
			if grid[y][x] != 'X' {
				continue
			}

			checkers = makeCheckers(x, y, grid, checkers)
		}
	}

	numFound := 0

	for _, c := range checkers {
		if c.runCheck() {
			numFound += 1
		}
	}

	println(numFound)
}

func makeCheckers(x int, y int, grid [][]rune, checkers []checker) []checker {
	x_max := len(grid[0])
	y_max := len(grid)
	canLeft := x >= 3
	canRight := x < (x_max - 3)
	canUp := y >= 3
	canDown := y < (y_max - 3)

	newCheckers := checkers

	if canUp {
		newCheckers = append(newCheckers, checker{
			dx:   0,
			dy:   -1,
			x:    x,
			y:    y,
			grid: grid,
		})

		if canRight {
			newCheckers = append(newCheckers, checker{
				dx:   1,
				dy:   -1,
				x:    x,
				y:    y,
				grid: grid,
			})
		}

		if canLeft {
			newCheckers = append(newCheckers, checker{
				dx:   -1,
				dy:   -1,
				x:    x,
				y:    y,
				grid: grid,
			})
		}
	}

	if canDown {
		newCheckers = append(newCheckers, checker{
			dx:   0,
			dy:   1,
			x:    x,
			y:    y,
			grid: grid,
		})
		if canRight {
			newCheckers = append(newCheckers, checker{
				dx:   1,
				dy:   1,
				x:    x,
				y:    y,
				grid: grid,
			})
		}

		if canLeft {
			newCheckers = append(newCheckers, checker{
				dx:   -1,
				dy:   1,
				x:    x,
				y:    y,
				grid: grid,
			})
		}
	}

	if canRight {
		newCheckers = append(newCheckers, checker{
			dx:   1,
			dy:   0,
			x:    x,
			y:    y,
			grid: grid,
		})
	}

	if canLeft {
		newCheckers = append(newCheckers, checker{
			dx:   -1,
			dy:   0,
			x:    x,
			y:    y,
			grid: grid,
		})
	}

	return newCheckers
}
