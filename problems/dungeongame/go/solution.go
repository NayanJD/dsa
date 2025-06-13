package problem

import (
	"fmt"
	"math"
	"slices"
)

func calculateMinimumHP(dungeon [][]int) int {
	n := len(dungeon)
	m := len(dungeon[0])

	curr := slices.Repeat([]int{math.MaxInt}, m)
	up := slices.Repeat([]int{0}, m)

	curr[m-1] = max(1, 1-dungeon[n-1][m-1])
	for i := n - 1; i >= 0; i-- {
		for j := m - 1; j >= 0; j-- {
			if j != m-1 {
				curr[j] = min(curr[j], max(1, curr[j+1]-dungeon[i][j]))
			}

			if i != 0 {
				up[j] = max(1, curr[j]-dungeon[i-1][j])
			}
		}

		for j := range m {
			fmt.Printf("%v, ", curr[j])
		}
		fmt.Println()

		if i != 0 {
			for j := range m {
				curr[j] = up[j]
			}
		}

	}

	return curr[0]
}
