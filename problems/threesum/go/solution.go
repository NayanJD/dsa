package threesum

import (
	"slices"

	"github.com/emirpasic/gods/maps/hashmap"
	"github.com/emirpasic/gods/sets/hashset"
)

func threeSum(nums []int) [][]int {
	result := make([][]int, 0)

	table := hashmap.New()
	uTable := hashset.New()

	slices.Sort(nums)

	var prev = int(1e5 + 1)
	for i := range len(nums) {
		if nums[i] == prev {
			continue
		}

		target := -nums[i]

		for j := i + 1; j < len(nums); j++ {
			if idx, found := table.Get(nums[j]); found {
				if !uTable.Contains(nums[j]) {
					result = append(result, []int{nums[i], nums[idx.(int)], nums[j]})
					uTable.Add(nums[j])
				}
			} else {
				table.Put(target-nums[j], j)
			}
		}

		table.Clear()
		uTable.Clear()

		prev = nums[i]
	}

	return result
}
