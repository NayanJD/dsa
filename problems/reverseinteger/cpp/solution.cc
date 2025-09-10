#include <catch2/catch_test_macros.hpp>
#include <cmath>
#include <limits>
#include <stdio.h>

using namespace std;

class Solution {
public:
  int reverse(int x) {

    int result{0}, lastDigit{};

    int maxVal = numeric_limits<int>::max();
    int minVal = numeric_limits<int>::min();

    while (x != 0) {
      lastDigit = x % 10;

      if (x >= 0) {
        if ((maxVal - lastDigit) / 10 < result) {
          return 0;
        }
      } else {
        if ((minVal - lastDigit) / 10 > result) {
          return 0;
        }
      }

      result = result * 10 + (x % 10);
      x /= 10;
    }

    return result; 
  }
};

TEST_CASE("Test Case#1", "problem") {
  Solution s;

  int input = 9;

  REQUIRE(s.reverse(input) == 9);
}

TEST_CASE("Test Case#2", "problem") {
  Solution s;

  int input = 12;

  REQUIRE(s.reverse(input) == 21);
}

TEST_CASE("Test Case#3", "problem") {
  Solution s;

  int input = 0;

  REQUIRE(s.reverse(input) == 0);
}

TEST_CASE("Test Case#4", "problem") {
  Solution s;

  int input = 12345678;

  REQUIRE(s.reverse(input) == 87654321);
}

TEST_CASE("Test Case#5", "problem") {
  Solution s;

  int input = 123456789;

  REQUIRE(s.reverse(input) == 987654321);
}

TEST_CASE("Test Case#6", "problem") {
  Solution s;

  int input = 1234567892;

  REQUIRE(s.reverse(input) == 0);
}

TEST_CASE("Test Case#7", "problem") {
  Solution s;

  int input = numeric_limits<int>::max();

  REQUIRE(s.reverse(input) == 0);
}

TEST_CASE("Test Case#8", "problem") {
  Solution s;

  int input = -123;

  REQUIRE(s.reverse(input) == -321);
}

TEST_CASE("Test Case#9", "problem") {
  Solution s;

  int input = numeric_limits<int>::min();

  REQUIRE(s.reverse(input) == 0);
}

TEST_CASE("Test Case#10", "problem") {
  Solution s;

  int input = 1534236469;

  REQUIRE(s.reverse(input) == 0);
}
