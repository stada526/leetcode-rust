use std::collections::HashSet;

struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
struct Board<'a> {
    board: &'a Vec<Vec<char>>,
}

impl<'a> Board<'a> {
    fn new(board: &'a Vec<Vec<char>>) -> Self {
        Board { board }
    }
    fn row_size(&self) -> i32 {
        return self.board.len() as i32;
    }
    fn col_size(&self) -> i32 {
        return self.board[0].len() as i32;
    }
    fn get_value(&self, coord: &Coord) -> char {
        return self.board[coord.y][coord.x];
    }
    fn next_coords(&self, coord: &Coord) -> Vec<Coord> {
        let mut ret = vec![];
        if 0 <= coord.y as i32 - 1 {
            ret.push(Coord::new(coord.x, coord.y - 1))
        }
        if coord.y as i32 + 1 <= self.row_size() - 1 {
            ret.push(Coord::new(coord.x, coord.y + 1))
        }
        if 0 <= coord.x as i32 - 1 {
            ret.push(Coord::new(coord.x - 1, coord.y))
        }
        if coord.x as i32 + 1 <= self.col_size() - 1 {
            ret.push(Coord::new(coord.x + 1, coord.y))
        }
        return ret;
    }
}

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
        let b = Board::new(&board);
        for row_index in 0..b.row_size() as usize {
            for col_index in 0..b.col_size() as usize {
                if Self::dfs(
                    Coord::new(col_index, row_index),
                    &b,
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
        coord: Coord,
        board: &Board,
        word_index: usize,
        word_vec: &Vec<char>,
        visited: &mut HashSet<(usize, usize)>,
    ) -> bool {
        if visited.contains(&(coord.x, coord.y)) {
            return false;
        }

        if board.get_value(&coord) != word_vec[word_index] {
            return false;
        }

        visited.insert((coord.x, coord.y));

        if word_index == word_vec.len() - 1 {
            return true;
        }

        let mut res: Vec<bool> = vec![];
        for next_coord in board.next_coords(&coord) {
            res.push(Self::dfs(
                next_coord,
                board,
                word_index + 1,
                word_vec,
                visited,
            ));
        }
        visited.remove(&(coord.x, coord.y));
        return res.iter().any(|x| *x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
