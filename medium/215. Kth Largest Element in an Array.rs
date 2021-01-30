impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        nums[nums.len() - k as usize]
    }
}
