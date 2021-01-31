impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let n = nums.len();
        let (mut l, mut r) = (0, n-1);
        for i in (0..n-1).rev() {
            if nums[i] < nums[i + 1] {
                l = i;
                break;
            }
        }
        for i in (l+1..n).rev() {
            if nums[i] > nums[l] {
                r = i;
                break;
            }
        }
        nums.swap(l, r);
        nums[l+1..n].sort();
    }
}
