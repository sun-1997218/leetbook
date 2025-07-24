use std::collections::HashMap;

pub struct Solution;

impl Solution{
    pub fn find_subarray(vec:Vec<i32>,k:i32)-> i32{
        let mut result =0;
        let mut sum = 0;
        let mut hash:HashMap<i32, usize> = HashMap::new();
        hash.insert(0,1);
        for i in 0..vec.len(){
            sum+=vec[i];
            if let Some(count) = hash.get(&(sum-k)){
                result+=count;
            }
            *hash.entry(sum).or_insert(0)+=1;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_subarray_basic() {
        let vec = vec![1, 2, 3];
        let k = 3;
        let result = Solution::find_subarray(vec, k);
        assert_eq!(result, 2); // [1,2], [3]
    }

    #[test]
    fn test_find_subarray_negative() {
        let vec = vec![1, -1, 0];
        let k = 0;
        let result = Solution::find_subarray(vec, k);
        assert_eq!(result, 3); // [1,-1], [0], [1,-1,0]
    }

    #[test]
    fn test_find_subarray_no_match() {
        let vec = vec![0,0,0];
        let k = 0;
        let result = Solution::find_subarray(vec, k);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_find_subarray_match() {
        let vec = vec![1];
        let k = 0;
        let result = Solution::find_subarray(vec, k);
        assert_eq!(result, 0);
    }
}