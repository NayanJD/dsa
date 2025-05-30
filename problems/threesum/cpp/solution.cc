#include <algorithm>
#include <catch2/catch_test_macros.hpp>
#include <catch2/matchers/catch_matchers_vector.hpp>
#include <stdio.h>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
public:
  vector<vector<int>> threeSum(vector<int> &nums) {
    unordered_map<int, int> table;
    unordered_set<int> uTable;

    int target{0};
    vector<vector<int>> result = {};

    sort(nums.begin(), nums.end());

    int prev = 1e5 + 1;
    for (int i = 0; i < nums.size(); i++) {
      if (nums[i] == prev) {
        continue;
      }

      target = -nums[i];
      for (int j = i + 1; j < nums.size(); j++) {
        if (table.find(nums[j]) != table.end() &&
            uTable.find(nums[j]) == uTable.end()) {
          result.push_back({nums[i], nums[table[nums[j]]], nums[j]});
          uTable.insert(nums[j]);
        } else {
          table[target - nums[j]] = j;
        }
      }

      table.clear();
      uTable.clear();

      prev = nums[i];
    }

    return result;
  }
};

TEST_CASE("Test Case#1", "problem") {
  Solution s;

  vector<int> nums = {-1, 0, 1, 2, -1, -4};
  vector<vector<int>> expectedResult = {{-1, -1, 2}, {-1, 0, 1}};
  vector<vector<int>> result = s.threeSum(nums);

  REQUIRE_THAT(result, Catch::Matchers::UnorderedEquals(expectedResult));
}

TEST_CASE("Test Case#2", "problem") {
  Solution s;

  vector<int> nums = {0, 1, 1};
  vector<vector<int>> result = {};
  REQUIRE(s.threeSum(nums) == result);
}

TEST_CASE("Test Case#3", "problem") {
  Solution s;

  vector<int> nums = {0, 0, 0};
  vector<vector<int>> result = {{0, 0, 0}};
  REQUIRE(s.threeSum(nums) == result);
}
