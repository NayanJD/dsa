#include <algorithm>
#include <catch2/catch_test_macros.hpp>
#include <stdio.h>

using namespace std;

class Solution {
public:
  int maxArea(vector<int> &height) {
    int largest{-1};

    int i = 0, j = height.size() - 1;

    while (i < j) {
      largest = max(largest, (j - i) * min(height[i], height[j]));

      if (height[i] == height[j]) {
        i++;
        j--;
      } else if (height[i] < height[j]) {
        i++;
      } else {
        j--;
      }
    }

    return largest;
  }
};

TEST_CASE("Test Case#1", "problem") {
  Solution s;

  vector<int> heights = {1, 8, 6, 2, 5, 4, 8, 3, 7};

  REQUIRE(s.maxArea(heights) == 49);
}
