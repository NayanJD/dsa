package problem

func maxArea(heights []int) int {
  largest := -1

  i := 0
  j := len(heights) - 1

  for i < j {
    largest = max(largest, (j - i) * min(heights[i], heights[j]))
    
    if heights[i] == heights[j] {
      i++;
      j--;
    } else if heights[i] < heights[j] {
      i++;
    } else {
      j--;
    }
  }

  return largest
}
