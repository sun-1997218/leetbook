use std::io::{self, BufRead, Write, BufReader, BufWriter};

pub fn solve() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let sum = sum_line(&input);
    writeln!(writer, "{}", sum).unwrap();
}

// 核心算法函数，便于单元测试
pub fn sum_line(input: &str) -> i32 {
    input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_line() {
        assert_eq!(sum_line("1 2 3 4"), 10);
        assert_eq!(sum_line("5 5 5"), 15);
        assert_eq!(sum_line(""), 0);
    }
} 