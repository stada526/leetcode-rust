use std::collections::HashSet;

struct Solution {}

impl Solution {
    fn dfs(
        heights: &Vec<Vec<i32>>,
        row: i32,
        col: i32,
        visited: &mut HashSet<(i32, i32)>,
        prev: i32,
    ) {
        if row < 0 || heights.len() as i32 - 1 < row || col < 0 || heights[0].len() as i32 - 1 < col
        {
            return;
        }
        let row_idx = row as usize;
        let col_idx = col as usize;
        if visited.contains(&(row, col)) {
            return;
        }
        if prev > heights[row_idx][col_idx] {
            return;
        }

        visited.insert((row, col));

        let diffs = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for diff in diffs {
            Self::dfs(
                heights,
                row + diff.0,
                col + diff.1,
                visited,
                heights[row_idx][col_idx],
            )
        }
    }

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut pacific_set = HashSet::new();
        let mut atlantic_set = HashSet::new();
        for row in 0..heights.len() {
            Self::dfs(&heights, row as i32, 0, &mut pacific_set, i32::MIN);
            Self::dfs(
                &heights,
                row as i32,
                heights[0].len() as i32 - 1,
                &mut atlantic_set,
                i32::MIN,
            )
        }
        for col in 0..heights[0].len() {
            Self::dfs(&heights, 0, col as i32, &mut pacific_set, i32::MIN);
            Self::dfs(
                &heights,
                heights.len() as i32 - 1,
                col as i32,
                &mut atlantic_set,
                i32::MIN,
            );
        }

        let mut ret = vec![];
        for pair in pacific_set {
            if atlantic_set.contains(&pair) {
                ret.push(vec![pair.0, pair.1])
            }
        }
        return ret;
    }
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;

    use super::*;

    #[test]
    fn test() {
        let heights = vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ];
        let expected = vec![
            vec![0, 4],
            vec![1, 3],
            vec![1, 4],
            vec![2, 2],
            vec![3, 0],
            vec![3, 1],
            vec![4, 0],
        ];
        let res = Solution::pacific_atlantic(heights);
        assert_eq_unordered!(res, expected);
    }
}
