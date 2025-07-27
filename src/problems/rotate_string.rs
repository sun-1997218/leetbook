pub struct Solution;
impl Solution {

    pub fn rotate_string (nums: &mut Vec<i32>, k: i32){
        let k = k as usize % nums.len(); 
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();


    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_rotate_string(){
        let mut nums = vec![1,2,3,4,5,6,7];
        Solution::rotate_string(&mut nums, 3);
        assert_eq!(nums, vec![5,6,7,1,2,3,4]);
    }
}