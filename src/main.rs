mod problems;

use std::io::{self, BufRead, Write, BufReader, BufWriter};

fn main() {
     let mut nums = vec![1,2,3,4,5,6,7];
     let len = nums.len()-1;
     for i in 0..len{
        println!("{}",nums[i]);
     }

}
