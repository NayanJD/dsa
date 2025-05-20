package twosum

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestTwoSum(t *testing.T) {
	tests := []struct {
		nums     []int
		target   int
		expected []int
	}{
		{
			nums:     []int{2, 7, 11, 15},
			target:   9,
			expected: []int{0, 1},
		},
	}

	for _, test := range tests {
		actual := twoSum(test.nums, test.target)

		assert.Equal(t, actual, test.expected)
	}

}
