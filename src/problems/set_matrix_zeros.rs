pub struct Solution;

impl Solution {
    pub fn solve(matrix:& mut Vec<Vec<i32>> ) {
        let mut rowflag = 0;
        let mut cloflag = 0;
        let m = matrix.len();
        let n =matrix[0].len();

        for i in 0 ..m {
            if(matrix[i][0]==0) {
                rowflag =1;
            }
        }
        for i in 0 ..n {
            if(matrix[0][i]==0) {
                cloflag =1;
            }
        }

        for i in 1..m{
            for j in 1..n {
                if(matrix[i][j] ==0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }
        for i in 0..m{
            if(matrix[i][0]==0){
                for j in 1..n{
                    matrix[i][j]=0;
                }
            }
        }

        for j in 0..n{
            if(matrix[0][j]==0){
                for i in 1..n{
                    matrix[i][j]=0;
                }
            }
        }
        if rowflag ==1 {
            for i in 0..n{
                matrix[0][i]=0;
            }
        }

        if cloflag ==1 {
            for i in 0..m{
                matrix[i][0]=0;
            }
        }
    }
}


#[cfg(test)]
pub mod test{
    use super::*;
    #[test]
    pub fn test1 (){
        let mut ma = vec![
            vec![1,1,1],
            vec![1,0,1],
            vec![1,1,1]
        ];
        Solution::solve(&mut ma);
        assert_eq!(ma,vec![
            vec![1,0,1],    
            vec![0,0,0],
            vec![1,0,1]
        ]);
    }
}