pub struct Solution;

impl Solution{
    pub fn solve(&mut matrix : Vec<Vec<i32>>){
        let n = matrix.len();
        for i in 0..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }


        for row in matrix {
            row.reverse();
        }

    }
}