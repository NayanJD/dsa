#include <algorithm>
#include <catch2/catch_test_macros.hpp>
#include <limits>
#include <queue>
#include <stdio.h>

using namespace std;

class Solution {
public:
  int calculateMinimumHP(vector<vector<int>> &dungeon) {

    int n = dungeon.size();
    int m = dungeon[0].size();

    int result{0};

    vector<int> up(m, 0), curr(m, numeric_limits<int>::max());

    curr[m - 1] = max(1, 1 - dungeon[n - 1][m - 1]);

    for (int i = n - 1; i >= 0; i--) {
      for (int j = m - 1; j >= 0; j--) {
        if (j != m - 1) {
          curr[j] = min(curr[j], max(curr[j + 1] - dungeon[i][j], 1));
        }

        if (i != 0) {
          up[j] = max(curr[j] - dungeon[i - 1][j], 1);
        }
      }

      /*for (int j = 0; j < m; j++) {*/
      /*  printf("%d,", curr[j]);*/
      /*}*/
      /*printf("\n");*/

      if (i != 0) {
        copy(up.begin(), up.end(), curr.begin());
      }
    }

    return curr[0];
  }
};

TEST_CASE("Test Case#1", "problem") {
  Solution s;

  vector<vector<int>> dungeon = {{-2, -3, 3}, {-5, -10, 1}, {10, 30, -5}};
  int output = 7;

  REQUIRE(s.calculateMinimumHP(dungeon) == output);
}

TEST_CASE("Test Case#2", "problem") {
  Solution s;

  vector<vector<int>> dungeon = {{0}};
  int output = 1;

  REQUIRE(s.calculateMinimumHP(dungeon) == output);
}

/**/
/* 1 -3  3*/
/* 0 -2  0*/
/*-3 -3 -3*/
TEST_CASE("Test Case#3", "problem") {
  Solution s;

  vector<vector<int>> dungeon = {{1, -3, 3}, {0, -2, 0}, {-3, -3, -3}};
  int output = 3;

  REQUIRE(s.calculateMinimumHP(dungeon) == output);
}
/**/
TEST_CASE("Test Case#4", "problem") {
  Solution s;

  vector<vector<int>> dungeon = {{0, 0}};
  int output = 1;

  REQUIRE(s.calculateMinimumHP(dungeon) == output);
}
