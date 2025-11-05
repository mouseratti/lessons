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

    /** if first array is empty*/
    if (0 == m) {
        for (int k = 0; k < n; k++) {
            nums1[k] = nums2[k];
        }
        return;
    }

}