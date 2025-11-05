#include <check.h>
#include "merge.h"

START_TEST(test_merge_0088) {
  int result[6] = {1, 2, 3, 3, 5, 7};
  int m = 3;
  int n = 3;
  int nums1[6] = {2, 3, 7, 0, 0, 0};
  int nums2[3] = {1, 3, 5};
  merge(nums1, n + m, m, nums2, n, n);
  for (int i = 0; i < n + m; i++) {
    ck_assert_msg(nums1[i] == result[i], "nums1[%d] is %d", i, nums1[i]);
  }
}
END_TEST

START_TEST(test_merge_0088_m1) {
  int result[5] = {1, 2, 3, 4, 5};
  int m = 1;
  int n = 4;
  int nums1[5] = {4, 0, 0, 0, 0};
  int nums2[4] = {1, 2, 3, 5};
  merge(nums1, n + m, m, nums2, n, n);
  for (int i = 0; i < n + m; i++) {
    ck_assert_msg(nums1[i] == result[i], "nums1[%d] is %d", i, nums1[i]);
  }
}
END_TEST

START_TEST(test_swapElems) {
  int elem1 = 5;
  int elem2 = 6;
  swapElems(&elem1, &elem2);
  ck_assert(elem1 == 6);
  ck_assert(elem2 == 5);
}
END_TEST

START_TEST(test_merge_m0) {
  int m = 0;
  int n = 1;
  int nums1[1] = {0};
  int nums2[1] = {1};
  merge(nums1, m + n, m, nums2, n, n);
  ck_assert(nums1[0] == 1);
}
END_TEST

START_TEST(test_merge_n0) {
  int m = 1;
  int n = 0;
  int nums1[1] = {3};
  int nums2[0] = {};
  merge(nums1, m, m, nums2, n, n);
  ck_assert(nums1[0] == 3);
}
END_TEST

Suite* make_suite_merge_0088(void) {
  Suite* s;
  TCase* tc_core;

  s = suite_create("0088_merge");

  tc_core = tcase_create("Core");

  tcase_add_test(tc_core, test_swapElems);
  tcase_add_test(tc_core, test_merge_m0);
  tcase_add_test(tc_core, test_merge_n0);
  tcase_add_test(tc_core, test_merge_0088_m1);
  tcase_add_test(tc_core, test_merge_0088);

  suite_add_tcase(s, tc_core);

  return s;
}
