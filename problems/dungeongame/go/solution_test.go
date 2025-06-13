package problem

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestCalculateMinimumHP(t *testing.T) {
	tests := []struct {
		dungeon  [][]int
		expected int
	}{
		{
			dungeon:  [][]int{{-2, -3, 3}, {-5, -10, 1}, {10, 30, -5}},
			expected: 7,
		},
		{
			dungeon:  [][]int{{0}},
			expected: 1,
		},
		{
			dungeon:  [][]int{{1, -3, 3}, {0, -2, 0}, {-3, -3, -3}},
			expected: 3,
		},
		{
			dungeon:  [][]int{{0, 0}},
			expected: 1,
		},
	}

	for i, test := range tests {

		t.Run(fmt.Sprintf("Test #%d", i), func(t *testing.T) {
			actual := calculateMinimumHP(test.dungeon)

			assert.Equal(t, test.expected, actual)

		})
	}

}
