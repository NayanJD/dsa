#include <catch2/catch_test_macros.hpp>
#include <stdio.h>

using namespace std;

class Solution {
public:
  int problem(int input) { return input; }
};

TEST_CASE("Test Case#1", "problem") {
  Solution s;
  
  int input = 9;

  REQUIRE(s.problem(input) == 9);
}

