#include <catch2/catch_test_macros.hpp>
#include <cmath>
#include <stdio.h>

using namespace std;

class Solution {
public:
  int countPrimes(int n) {

    if (n == 0 || n == 1) {
      return 0;
    }

    vector<int> sieve(n, 1);
    sieve[1] = 0;

    // To test for primes we only need to iterate its factors upto and equal to
    // sqrt(n)
    for (int i = 1; i <= sqrt(n); i++) {
      if (sieve[i] == 1) {
        // We can start checking from i^2 because for the values till < i^2
        // has already been checked by primes encountered earlier
        for (int poweredValue = i * i; poweredValue < n; poweredValue += i) {
          sieve[poweredValue] = 0;
        }
      }
    }

    int primeCount{0};
    for (int i = 1; i < n; i++) {
      if (sieve[i] == 1) { // not necessary but added for readability
        primeCount++;
      }
    }

    return primeCount;
  }
};

TEST_CASE("Test Case#1", "problem") {
  Solution s;

  int n = 10;

  REQUIRE(s.countPrimes(n) == 4);
}

TEST_CASE("Test Case#2", "problem") {
  Solution s;

  int n = 0;

  REQUIRE(s.countPrimes(n) == 0);
}

TEST_CASE("Test Case#3", "problem") {
  Solution s;

  int n = 1;

  REQUIRE(s.countPrimes(n) == 0);
}
