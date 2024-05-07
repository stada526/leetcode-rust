use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars = s.chars().collect::<Vec<char>>();
        let mut char_map = HashMap::<char, i32>::new();

        let mut res = 0;
        let mut left_index = 0;

        for right_index in 0..chars.len() {
            let char = chars[right_index];
            let val = *char_map.get(&char).unwrap_or(&0) + 1;
            char_map.insert(char, val);
            if Solution::is_valid(&char_map, k) {
                res = res.max(right_index - left_index + 1);
                continue;
            }
            while !Solution::is_valid(&char_map, k) {
                let left_char = chars[left_index];
                char_map.insert(left_char, *char_map.get(&left_char).unwrap() - 1);
                left_index += 1;
            }
        }

        return res as i32;
    }

    fn is_valid(char_map: &HashMap<char, i32>, k: i32) -> bool {
        let mut total = 0;
        let mut max = 0;
        for (key, value) in char_map {
            total += value;
            max = max.max(*value);
        }

        let diff = total - max;
        return diff <= k;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "AABABBA".to_string();
        let k = 1;

        let res = Solution::character_replacement(input, k);

        assert_eq!(res, 4);
    }
}
