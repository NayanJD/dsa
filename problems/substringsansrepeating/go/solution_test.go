package problem

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestLengthOfLongestSubstring(t *testing.T) {
	tests := []struct {
		str      string
		expected int
	}{
		{
			str:      "abcabcbb",
			expected: 3,
		},
		{
			str:      "bbbbb",
			expected: 1,
		},
		{
			str:      "pwwkew",
			expected: 3,
		},
		{
			str:      "",
			expected: 0,
		},
		{
			str:      "abcdefghij",
			expected: 10,
		},
		{
			str:      " ",
			expected: 1,
		},
	}

	for i, test := range tests {

		t.Run(fmt.Sprintf("Test #%d", i), func(t *testing.T) {
			actual := lengthOfLongestSubstring(test.str)

			assert.Equal(t, test.expected, actual)

		})
	}

}
