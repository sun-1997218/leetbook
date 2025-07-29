pub struct Solution;

impl Solution {
    pub fn solve(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];
        let mut ans = vec![1; n];

        // 计算左侧乘积
        for i in 1..n {
            left[i] = left[i-1] * nums[i-1];
        }
        // 计算右侧乘积
        for i in rev() {
            right[i] = right[i+1] * nums[i+1];
        }
        // 结果
        for i in 0..n {
            ans[i] = left[i] * right[i];
        }
        ans
    }
}