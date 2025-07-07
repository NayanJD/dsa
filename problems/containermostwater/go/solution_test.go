package problem

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestMaxArea(t *testing.T) {
	tests := []struct {
		heights      []int
		expected int
	}{
		{
			heights:      []int{1,8,6,2,5,4,8,3,7},
			expected: 49,
		},
	}

	for i, test := range tests {

		t.Run(fmt.Sprintf("Test #%d", i), func(t *testing.T) {
			actual := maxArea(test.heights)

			assert.Equal(t, test.expected, actual)

		})
	}

}
