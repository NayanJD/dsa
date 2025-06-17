package problem

import (
	"github.com/emirpasic/gods/maps/hashmap"
)

func lengthOfLongestSubstring(s string) int {

	if len(s) == 0 {
		return 0
	}

	maxLength := 1
	i := 0
	j := 1

	seen := hashmap.New()
	seen.Put(s[i], 0)

	for j < len(s) {
		if idx, found := seen.Get(s[j]); found {
			for i <= idx.(int) {
				seen.Remove(s[i])
				i++
			}
		}

    seen.Put(s[j], j)

		maxLength = max(maxLength, j-i+1)

		j++
	}

	return maxLength

}
