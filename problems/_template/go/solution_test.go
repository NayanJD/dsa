package problem

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestTwoSum(t *testing.T) {
	tests := []struct {
		num      int
		expected int
	}{
		{
			num:      9,
			expected: 9,
		},
	}

	for i, test := range tests {

		t.Run(fmt.Sprintf("Test #%d", i), func(t *testing.T) {
			actual := problem(test.nums, test.target)

			assert.Equal(t, actual, test.expected)

		})
	}

}
