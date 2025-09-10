package problem

import (
	"math"
)

func reverse(input int) int {
  
	var result int = 0
	var lastDigit int

	for input != 0 {
		lastDigit = input % 10

		if(input > 0 && ((math.MaxInt32 - lastDigit) / 10) < result) {
		  return 0
		} else if (input < 0 && ((math.MinInt32 - lastDigit) / 10) > result){
		  return 0
		}

		result = result*10 + lastDigit
    input /= 10
	}
	return result
}
