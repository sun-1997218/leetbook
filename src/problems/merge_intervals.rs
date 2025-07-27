pub  struct Solution;

impl Solution{
    pub fn merge_intervals(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>{
        let mut ans = Vec::new();
        let mut start =0;
        let mut end = 0;

        for (i, interval) in intervals.iter().enumerate() {
            let l = interval[0];
            let r = interval[1];
            if i == 0 {
                start = l;
                end = r;
            } else if end < l {
                ans.push(vec![start, end]);
                start = l;
                end = r;
            } else if end >= l {
                end = end.max(r);
            }
        }
        ans.push(vec![start, end]); // Add the last interval
        return ans;
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_merge_intervals(){
        assert_eq!(Solution::merge_intervals(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]), vec![vec![1,6],vec![8,10],vec![15,18]]);
        assert_eq!(Solution::merge_intervals(vec![vec![1,4],vec![4,5]]), vec![vec![1,5]]);
    }
}