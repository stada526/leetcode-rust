struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = nums.len() as i32 - 1;
        while left_ptr <= right_ptr {
            let mid_ptr = (right_ptr + left_ptr) / 2;
            match target.cmp(&nums[mid_ptr as usize]) {
                std::cmp::Ordering::Less => {
                    right_ptr = mid_ptr - 1;
                }
                std::cmp::Ordering::Equal => return mid_ptr as i32,
                std::cmp::Ordering::Greater => {
                    left_ptr = mid_ptr + 1;
                }
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        let input = vec![-1, 0, 3, 5, 9, 12];
        let res = Solution::search(input, 9);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_not_exist() {
        let input = vec![-1, 0, 3, 5, 9, 12];
        let res = Solution::search(input, 2);
        assert_eq!(res, -1);
    }

    #[test]
    fn test_single_element() {
        let input = vec![1];
        let res = Solution::search(input, -1);
        assert_eq!(res, -1);
    }
}
