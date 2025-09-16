package problem

import (
	"fmt"
	"github.com/stretchr/testify/assert"
	"testing"
)

func TestProblem(t *testing.T) {
	tests := []struct {
		n      int
		count int
	}{
		{
			n:     10,
			count: 4,
		},
    {
			n:     0,
			count: 0,
		},
    {
			n:     1,
			count: 0,
		},
	}

	for i, test := range tests {
		t.Run(fmt.Sprintf("Test #%d", i), func(t *testing.T) {
			actual := countPrimes(test.n)

			assert.Equal(t, test.count, actual)
		})
	}

}
