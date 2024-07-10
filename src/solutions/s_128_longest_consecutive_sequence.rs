use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut num_set = HashSet::<i32>::new();
        for num in nums.iter() {
            num_set.insert(*num);
        }

        let mut res = 0;

        for i in 0..nums.len() {
            if Self::is_first_in_sequence(&num_set, nums[i]) {
                let mut cnt = 0;
                let mut num = nums[i];
                while num_set.contains(&num) {
                    cnt += 1;
                    num += 1
                }
                res = res.max(cnt);
            }
        }

        return res;
    }

    fn is_first_in_sequence(num_set: &HashSet<i32>, num: i32) -> bool {
        return !num_set.contains(&(num - 1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        let res = Solution::longest_consecutive(nums);
        assert_eq!(res, 4)
    }
}
