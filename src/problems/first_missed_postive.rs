impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        for i in 0..n {
       
            while 1 <= nums[i] && nums[i] as usize <= n && nums[i] != nums[(nums[i] - 1) as usize] {
        
                let j = (nums[i] - 1) as usize;
                nums.swap(i, j);
            }
        }
      
        for i in 0..n {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as _;
            }
        }

       
        (n + 1) as _
    }
}