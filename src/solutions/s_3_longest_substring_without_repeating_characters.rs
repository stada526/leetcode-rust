use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut char_map = HashMap::new();

        let mut left_ptr = 0;

        let mut max = 0;

        let chars = s.chars().collect::<Vec<char>>();
        for right_ptr in 0..s.len() {
            let char = chars[right_ptr];

            if char_map.contains_key(&char) {
                let next_left_ptr = *char_map.get(&char).unwrap() + 1;
                while left_ptr != next_left_ptr {
                    let ith_char = chars[left_ptr];
                    char_map.remove(&ith_char);
                    left_ptr += 1;
                }
            }

            let num_words = right_ptr - left_ptr + 1;
            max = max.max(num_words);

            char_map.insert(char, right_ptr);
        }

        return max as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abcabcbb() {
        let input = "abcabcbb".to_string();
        let res = Solution::length_of_longest_substring(input);
        assert_eq!(res, 3);
    }

    #[test]
    fn test_abba() {
        let input = "abba".to_string();
        let res = Solution::length_of_longest_substring(input);
        assert_eq!(res, 2);
    }
}
