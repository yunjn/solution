#include <iostream>
#include <string>
#include <vector>
using namespace std;

int searchInsert(vector<int>& nums, int target) {
  int len = nums.size();
  for (int i = 0; i < len; i++) {
    if (nums[i] >= target) {
      return i;
    }
  }
  return len;
}

bool winnerOfGame(string colors) {
  int a_cnt = 0, b_cnt = 0, len = colors.size();
  for (int i = 1; i < len - 1; ++i) {
    if (colors[i - 1] == 'A' && colors[i] == 'A' && colors[i + 1] == 'A') {
      a_cnt++;
    } else if (colors[i - 1] == 'B' && colors[i] == 'B' &&
               colors[i + 1] == 'B') {
      b_cnt++;
    }
  }
  return a_cnt > b_cnt;
}

int main() { return 0; }