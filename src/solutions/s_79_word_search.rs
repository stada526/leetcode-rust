use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        // Approach: Depth First Search
        //
        // Contraints:
        //  - m == board.length
        //  - n = board[i].length
        //  - 1 <= m, n <= 6
        //  - 1 <= word.length <= 15
        //
        // Time Complexity: O(m * n * word.length)
        //  - Maximum board size is 36 (m * n)
        //
        let word_vec = word.chars().collect::<Vec<char>>();
        for row_index in 0..board.len() {
            for col_index in 0..board[0].len() {
                if Self::dfs(
                    row_index as i32,
                    col_index as i32,
                    &board,
                    0,
                    &word_vec,
                    &mut HashSet::new(),
                ) {
                    return true;
                }
            }
        }

        return false;
    }

    fn dfs(
        row_index: i32,
        col_index: i32,
        board: &Vec<Vec<char>>,
        word_index: usize,
        word_vec: &Vec<char>,
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if row_index < 0
            || board.len() as i32 - 1 < row_index
            || col_index < 0
            || board[0].len() as i32 - 1 < col_index
        {
            return false;
        }

        let row_index_usize = row_index as usize;
        let col_index_usize = col_index as usize;

        if visited.contains(&(row_index_usize, col_index_usize)) {
            return false;
        }

        if board[row_index_usize][col_index_usize] != word_vec[word_index] {
            return false;
        }

        visited.insert((row_index_usize, col_index_usize));

        if word_index == word_vec.len() - 1 {
            return true;
        }
        let mut res = vec![];
        for (row, col) in [
            (row_index + 1, col_index),
            (row_index - 1, col_index),
            (row_index, col_index + 1),
            (row_index, col_index - 1),
        ] {
            res.push(Self::dfs(
                row,
                col,
                board,
                word_index + 1,
                word_vec,
                visited,
            ));
        }
        visited.remove(&(row_index_usize, col_index_usize));
        return res.iter().any(|x| *x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_true() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let word_vec = word.chars().collect::<Vec<char>>();
        let res = Solution::dfs(0, 0, &board, 0, &word_vec, &mut HashSet::new());
        assert!(res);
    }

    #[test]
    fn test_dfs_true_2() {
        let board = vec![vec!['a', 'b'], vec!['c', 'd']];
        let word = "cdba".to_string();
        let word_vec = word.chars().collect::<Vec<char>>();
        let res = Solution::dfs(1, 0, &board, 0, &word_vec, &mut HashSet::new());
        assert!(res);
    }

    #[test]
    fn test_dfs_true_3() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'E', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCESEEEFS";
        let word_vec = word.chars().collect::<Vec<char>>();
        let res = Solution::dfs(0, 0, &board, 0, &word_vec, &mut HashSet::new());
        assert!(res);
    }

    #[test]
    fn test_dfs_false() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let word_vec = word.chars().collect::<Vec<char>>();
        let res = Solution::dfs(1, 0, &board, 0, &word_vec, &mut HashSet::new());
        assert!(!res);
    }

    #[test]
    fn test_exist_true() {
        let board: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCCED".to_string();
        let res = Solution::exist(board, word);
        assert!(res);
    }

    #[test]
    fn test_exist_false() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];
        let word = "ABCB".to_string();
        let res = Solution::exist(board, word);
        assert!(!res);
    }
}
