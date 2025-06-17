#include <catch2/catch_test_macros.hpp>
#include <string.h>
#include <unordered_map>
#include <vector>

using namespace std;

class Solution {
public:
  int lengthOfLongestSubstring(string s) {
    unordered_map<char, int> seen;

    int i = 0, j = 1;

    int maxLength{1}, lastSeenAt{0};

    if (s.size() == 0) {
      return 0;
    }

    seen[s[0]] = 0;

    while (j < s.size()) {
      if (seen.find(s[j]) != seen.end()) {
        lastSeenAt = seen[s[j]];
        while (i <= lastSeenAt) {
          seen.erase(seen.find(s[i]));
          i++;
        }
      }

      seen[s[j]] = j;

      maxLength = max(maxLength, j - i + 1);
      
      /*UNSCOPED_INFO("j: " << j << " ,i: " << i << " ,maxLength: " << maxLength << "\n");*/

      j++;
    }


    return maxLength;
  }
};

TEST_CASE("Test Case#1", "problem") {
  Solution s;

  string input = "abcabcbb";

  REQUIRE(s.lengthOfLongestSubstring(input) == 3);
}

TEST_CASE("Test Case#2", "problem") {
  Solution s;

  string input = "bbbbb";

  REQUIRE(s.lengthOfLongestSubstring(input) == 1);
}

TEST_CASE("Test Case#3", "problem") {
  Solution s;

  string input = "pwwkew";

  REQUIRE(s.lengthOfLongestSubstring(input) == 3);
}

TEST_CASE("Test Case#4", "problem") {
  Solution s;

  string input = "";

  REQUIRE(s.lengthOfLongestSubstring(input) == 0);
}

TEST_CASE("Test Case#5", "problem") {
  Solution s;

  string input = "abcdefghij";

  REQUIRE(s.lengthOfLongestSubstring(input) == 10);
}

TEST_CASE("Test Case#6", "problem") {
  Solution s;

  string input = " ";

  REQUIRE(s.lengthOfLongestSubstring(input) == 1);
}
