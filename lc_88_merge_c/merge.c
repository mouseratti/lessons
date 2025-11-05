#include "merge.h"

void swapElems(int* elem1, int* elem2) {
  *elem1 = *elem1 ^ *elem2;
  *elem2 = *elem1 ^ *elem2;
  *elem1 = *elem1 ^ *elem2;
}

void merge(int* nums1, int nums1Size, int m, int* nums2, int nums2Size, int n) {
  /** if 2nd array is empty*/
  if (0 == n) {
    return;
  }
  if (0 == m) {
    for (int k = 0; k < n; k++) {
      nums1[k] = nums2[k];
    }
    return;
  }

  int* ptr1 = nums1 + m - 1;  // pointer to last non-zero element in nums1
  short nums1_exhausted = 0;

  int* ptr2 = nums2 + n - 1;  // pointer to last element in nums2
  short nums2_exhausted = 0;

  for (int* i = nums1 + m + n - 1; i >= nums1; i--) {
    if (nums2_exhausted || (nums1_exhausted < 1 && *ptr1 > *ptr2)) {
      *i = *ptr1;
      if (nums1 == ptr1) {
        nums1_exhausted = 1;
      } else {
        ptr1 -= 1;
      }
    } else {
      *i = *ptr2;
      if (nums2 == ptr2) {
        nums2_exhausted = 1;
      } else {
        ptr2 -= 1;
      }
    }
  }
}
