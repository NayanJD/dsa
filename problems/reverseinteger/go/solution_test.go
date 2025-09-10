package problem

import (
	"fmt"
	"math"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestProblem(t *testing.T) {
	tests := []struct {
		num      int
		expected int
	}{
		{
			num:      9,
			expected: 9,
		},
    {
			num:      12,
			expected: 21,
		},
    {
			num:      12345678,
			expected: 87654321,
		},
    {
			num:      1234567892,
			expected: 0,
		},
    {
			num:      math.MaxInt32,
			expected: 0,
		},
    {
			num:      0,
			expected: 0,
		},
    {
			num:      123456789,
			expected: 987654321,
		},
    {
			num:      -123,
			expected: -321,
		},
    {
			num:      math.MinInt32,
			expected: 0,
		},
    {
			num:      1534236469,
			expected: 0,
		},
	}

	for i, test := range tests {

		t.Run(fmt.Sprintf("Test #%d", i), func(t *testing.T) {
			actual := reverse(test.num)

			assert.Equal(t, test.expected, actual)
		})
	}

}
