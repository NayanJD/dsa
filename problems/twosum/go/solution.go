package twosum

import (
	"github.com/emirpasic/gods/maps/hashmap"
)

func twoSum(nums []int, target int) []int {
	result := []int{0, 0}

	table := hashmap.New()

	for i, num := range nums {
		if index, ok := table.Get(target - num); ok {
			result[0] = index.(int)
			result[1] = i
			break
		} else {
			table.Put(num, i)
		}
	}

	return result
}
