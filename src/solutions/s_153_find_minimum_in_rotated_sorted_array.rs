struct Solution {}

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left_ptr = 0;
        let mut right_ptr = nums.len() - 1;
        while left_ptr <= right_ptr {
            let mid_ptr = (left_ptr + right_ptr) / 2;
            if nums[left_ptr] <= nums[mid_ptr] && nums[mid_ptr] <= nums[right_ptr] {
                return nums[left_ptr];
            }
            if nums[left_ptr] <= nums[mid_ptr] {
                left_ptr = mid_ptr + 1
            } else {
                right_ptr = mid_ptr
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_element() {
        let input = vec![3, 4, 5, 1, 2];
        let res = Solution::find_min(input);
        assert_eq!(res, 1)
    }

    #[test]
    fn test_single_element() {
        let input = vec![1];
        let res = Solution::find_min(input);
        assert_eq!(res, 1)
    }
}
