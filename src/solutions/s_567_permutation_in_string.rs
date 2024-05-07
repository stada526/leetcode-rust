use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut s1_char_map = HashMap::<char, i32>::new();
        for char in s1.chars() {
            s1_char_map.insert(char, s1_char_map.get(&char).unwrap_or(&0) + 1);
        }

        let s2_chars = s2.chars().collect::<Vec<char>>();
        let mut s2_char_map = HashMap::<char, i32>::new();
        let mut index = 0;
        while index < s2.len() {
            let s2_char = s2_chars[index];
            s2_char_map.insert(s2_char, s2_char_map.get(&s2_char).unwrap_or(&0) + 1);
            if index + 1 < s1.len() {
                index += 1;
                continue;
            }

            if Solution::is_same_hashmap(&s1_char_map, &s2_char_map) {
                return true;
            }

            let target = s2_chars[index + 1 - s1.len()];
            s2_char_map.insert(target, s2_char_map.get(&target).unwrap_or(&0) - 1);

            index += 1;
        }

        return false;
    }

    fn is_same_hashmap(map1: &HashMap<char, i32>, map2: &HashMap<char, i32>) -> bool {
        for (map1_key, map1_value) in map1 {
            let val_option = map2.get(&map1_key);
            match val_option {
                Some(val) => {
                    if *val != *map1_value {
                        return false;
                    }
                }
                None => return false,
            }
        }
        return true;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_true() {
        let s1 = "ab".to_string();
        let s2 = "eidbaooo".to_string();

        let res = Solution::check_inclusion(s1, s2);

        assert!(res);
    }

    #[test]
    fn test_false() {
        let s1 = "ab".to_string();
        let s2 = "eidboaoo".to_string();

        let res = Solution::check_inclusion(s1, s2);

        assert!(!res);
    }
}
