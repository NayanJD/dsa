package problem

import (
	"math"
	"slices"
)

func countPrimes(n int) int {

	if n == 0 || n == 1 {
		return 0
	}

	var sieve = slices.Repeat([]int{1}, n)
	sieve[1] = 0

	for i := 1; float64(i) < math.Sqrt(float64(n)); i++ {
		if sieve[i] == 1 {
			for poweredValue := i*i; poweredValue < n; poweredValue += i{
				sieve[poweredValue] = 0
			}
		}
	}

	primeCount := 0
	for i := 1; i < n; i++ {
		if sieve[i] == 1 {
			primeCount++
		}
	}

	return primeCount
}
