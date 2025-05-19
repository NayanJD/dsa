#include <stdio.h>
#include <vector>

using namespace std;

void printVector(vector<int> nums) {
  printf("[");
  for (int i = 0; i < nums.size(); i++) {
    if (i == nums.size() - 1) {
      printf("%d", nums[i]);
    } else {
      printf("%d,", nums[i]);
    }
  }
  printf("]\n");
}
