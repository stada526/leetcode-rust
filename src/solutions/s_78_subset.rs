struct Solution {}

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        Solution::backtrack(0, &mut res, &mut vec![], &nums);
        return res;
    }

    fn backtrack(index: usize, res: &mut Vec<Vec<i32>>, stack: &mut Vec<i32>, nums: &Vec<i32>) {
        if index == nums.len() {
            res.push(stack.clone());
            return;
        }
        stack.push(nums[index]);
        Solution::backtrack(index + 1, res, stack, nums);
        stack.pop();

        Solution::backtrack(index + 1, res, stack, nums);
    }
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 2, 3];
        let expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        let res = Solution::subsets(input);
        assert_eq_unordered!(res, expected);
    }
}
