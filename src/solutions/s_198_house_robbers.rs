struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut pre = 0;
        let mut next = 0;
        for num in nums {
            let money = next.max(pre + num);
            pre = next;
            next = money;
        }
        return pre.max(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![2, 7, 9, 3, 1];
        let res = Solution::rob(input);

        assert_eq!(res, 12);
    }
}
