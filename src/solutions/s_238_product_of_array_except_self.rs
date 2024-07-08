struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut forward_products = vec![1; nums.len()];
        let mut backward_products = vec![1; nums.len()];
        for i in 1..nums.len() {
            forward_products[i] = forward_products[i - 1] * nums[i - 1];
            let offset = nums.len() - 1 - i;
            backward_products[offset] = backward_products[offset + 1] * nums[offset + 1];
        }

        let mut res = vec![];
        for i in 0..forward_products.len() {
            res.push(forward_products[i] * backward_products[i])
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_only_positive_integer() {
        let input = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        let res = Solution::product_except_self(input);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_zero_and_negative() {
        let input = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];
        let res = Solution::product_except_self(input);
        assert_eq!(res, expected);
    }
}
