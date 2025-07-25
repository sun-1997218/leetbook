pub struct Solution;


/*dp */
impl Solution{
    pub fn get_subarray(nums: Vec<i32>) ->i32{
        let mut result = i32::MIN;
        let mut current_sum = 0;
        for &num in nums.iter() {
            if current_sum <= 0 {
                current_sum = num;
            } else {
                current_sum += num;
            }
            result = result.max(current_sum);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_subarray() {
        assert_eq!(Solution::get_subarray(vec![-2,1,-3,4,-1,2,1,-5,4]), 6); // 4,-1,2,1
        assert_eq!(Solution::get_subarray(vec![1]), 1);
        assert_eq!(Solution::get_subarray(vec![5,4,-1,7,8]), 23);
        assert_eq!(Solution::get_subarray(vec![-1,-2,-3,-4]), -1);
        assert_eq!(Solution::get_subarray(vec![-2, -1]), -1);
        assert_eq!(Solution::get_subarray(vec![2, -10, 3, 4]), 7);
    }
}