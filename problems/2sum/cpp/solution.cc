#include "../../../languages/cpp/lib.h"
#include <catch2/catch_test_macros.hpp>
#include <stdio.h>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {
    vector<int> result = {0, 0};

    unordered_map<int, int> table;

    for (int i = 0; i < nums.size(); i++) {
      if (table.find(target - nums[i]) != table.end()) {
        result[0] = i;
        result[1] = table[target - nums[i]];
        break;
      } else {
        table[nums[i]] = i;
      }
    }

    return result;
  }
};

TEST_CASE("Test Case#1", "twoSums") {
  Solution s;
  vector<int> nums = {2, 7, 11, 15};

  REQUIRE(s.twoSum(nums, 9) == vector<int>{1, 0});
}
