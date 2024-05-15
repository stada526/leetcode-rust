use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut index_set: HashSet<usize> = HashSet::new();
        let mut res: Vec<Vec<i32>> = vec![];

        for num in 0..nums.len() {
            index_set.insert(num);
        }

        Solution::dfs(&nums, &mut index_set, &mut vec![], &mut res);

        return res;
    }

    fn dfs(
        nums: &Vec<i32>,
        index_set: &mut HashSet<usize>,
        stack: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if index_set.is_empty() {
            res.push(stack.clone());
            return;
        }
        for index in index_set.clone().iter() {
            stack.push(nums[*index]);
            index_set.remove(index);
            Solution::dfs(nums, index_set, stack, res);
            stack.pop();
            index_set.insert(*index);
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 2, 3];
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1],
        ];
        let res = Solution::permute(input);
        assert_eq_unordered!(res, expected);
    }
}
