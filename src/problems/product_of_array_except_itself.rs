pub struct Solution;

impl Solution {
    pub fn solve(nums: Vec<i32>) -> Vec<i32>{
        let mut left = Vec::with_capacity(nums.len() as usize,1);
        let mut right = Vec::with_capacity(nums.len() as usize,1);
        let len = nums.len();
        for i in 0..len{
            if(i==0){
                left[i] == nums[i];
                right[len-i] = nums[len-i];
            }
            else {
                
            }
        }

    }
}