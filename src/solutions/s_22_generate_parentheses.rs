struct Solution {}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(
            left_cnt: i32,
            right_cnt: i32,
            stack: &mut Vec<String>,
            res: &mut Vec<String>,
            n: i32,
        ) {
            if left_cnt > n || right_cnt > n || left_cnt < right_cnt {
                return;
            }
            if left_cnt == n && right_cnt == n {
                let string = stack.join("");
                res.push(string);
                return;
            }

            if left_cnt == right_cnt {
                stack.push("(".to_string());
                dfs(left_cnt + 1, right_cnt, stack, res, n);
                stack.pop();
            }

            if left_cnt > right_cnt {
                stack.push("(".to_string());
                dfs(left_cnt + 1, right_cnt, stack, res, n);
                stack.pop();

                stack.push(")".to_string());
                dfs(left_cnt, right_cnt + 1, stack, res, n);
                stack.pop();
            }
        }

        let mut res = Vec::new();
        dfs(0, 0, &mut Vec::new(), &mut res, n);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use assert_unordered::{self, assert_eq_unordered};

    #[test]
    fn test() {
        let input = 3;
        let res = Solution::generate_parenthesis(input);
        let expected: Vec<String> = vec![
            "((()))".to_string(),
            "(()())".to_string(),
            "(())()".to_string(),
            "()(())".to_string(),
            "()()()".to_string(),
        ];
        assert_eq_unordered!(res, expected);
    }
}
