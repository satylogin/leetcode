class Solution {
public:
    int find(vector<int> &A, vector<int> &B, int x) {
        int start = -1e9;
        int end = 1e9;
        int ans = 0;
        
        while (start <= end) {
            int mid = (start + end) >> 1;
            int a = upper_bound(A.begin(), A.end(), mid) - A.begin();
            int b = upper_bound(B.begin(), B.end(), mid) - B.begin();
            if (a+b >= x) {
                ans = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        
        return ans;
    }
    
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        int n = nums1.size() + nums2.size(), x, y;
        x = find(nums1, nums2, (n+1) / 2);
        if (n&1) y = x;
        else y = find(nums1, nums2, (n+1) / 2 + 1);
        
        return (x + y) / 2.0;
    }
};
