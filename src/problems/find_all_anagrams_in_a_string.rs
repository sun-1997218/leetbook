
/*
window sliding
get ascii num : c - b'a'
String to bytes: s.as_bytes() get whole String as bytes
bytes[0] - b'a' get ascii num

 */
pub struct Solution;
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut cnt_p = [0; 26]; 
        let mut cnt_s = [0; 26];
        for c in p.bytes() {
            cnt_p[(c - b'a') as usize] += 1; 
        }

        let mut ans = vec![];
        let s = s.as_bytes();
        for (right, &c) in s.iter().enumerate() {
            cnt_s[(c - b'a') as usize] += 1; 

            if right + 1 >= p.len() {
                let left = right + 1 - p.len();
                if cnt_s == cnt_p {
                    ans.push(left as i32);
                }
                cnt_s[(s[left] - b'a') as usize] -= 1; 
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_anagrams_basic() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let mut result = Solution::find_anagrams(s, p);
        result.sort();
        assert_eq!(result, vec![0, 6]);
    }

    #[test]
    fn test_find_anagrams_no_match() {
        let s = "abcdefg".to_string();
        let p = "hij".to_string();
        let result = Solution::find_anagrams(s, p);
        assert!(result.is_empty());
    }

}
