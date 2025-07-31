pub struct Solution;

impl Solution{
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
     let mut m = matrix.len();
     let mut n =matrix[0].len()-1;
     let mut i = 0;

     while(i<m && n>=0){
        if(target > matrix[i][n]){
                i+=1;
        }
        else if(target < matrix[i][n]){
                n-=1;

        }
        else {
          return  true;
        }
     }
      false;
    }
}