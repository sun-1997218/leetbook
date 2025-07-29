pub struct Solution;

impl Solution{
   
        pub fn spiral_order(mut matrix: Vec<Vec<i32>>) -> Vec<i32> {
            const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // 右下左上
            let mut m = matrix.len();
            let mut n = matrix[0].len();
            let size = m * n;
            let mut ans = Vec::with_capacity(size);
            let mut i = 0;
            let mut j = -1; 
            let mut di = 0;
            while ans.len() < size {
                let (dx, dy) = DIRS[di];
                for _ in 0..n { 
                    i += dx;
                    j += dy; 
                    ans.push(matrix[i as usize][j as usize]); 
                }
                di = (di + 1) % 4; 
                (n, m) = (m - 1, n); 
            }
            ans
        }
    }
