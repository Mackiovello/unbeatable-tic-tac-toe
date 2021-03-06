extern crate itertools;

use board::Board;
use win_condition::itertools::Itertools;

pub fn is_winning_board(board: Board) -> bool {
    is_column_win(board) || is_row_win(board) || is_diagonal_win(board)
}

fn is_diagonal_win(board: Board) -> bool {
    let mut right_diagonal: Vec<char> = Vec::new();
    let mut left_diagonal: Vec<char> = Vec::new();
    for x in 0..3 {
        right_diagonal.push(board.grid[x][x]);
        left_diagonal.push(board.grid[x][2 - x]);
    }

    unique_non_empty_row(&right_diagonal) || unique_non_empty_row(&left_diagonal)
}

fn unique_non_empty_row(row: &[char]) -> bool {
    row.into_iter().unique().count() == 1 && row[0] != '-'
}

fn is_column_win(board: Board) -> bool {
    let transposed = board.transpose();
    is_row_win(transposed)
}

fn is_row_win(board: Board) -> bool {
    board
        .grid
        .iter()
        .filter(|x| unique_non_empty_row(*x))
        .count() > 0
}

#[cfg(test)]
mod win_condition_tests {
    use super::*;

    #[test]
    fn empty_board_is_no_win() {
        let board = Board {
            grid: [['-'; 3]; 3],
        };
        assert!(!is_winning_board(board));
    }

    #[test]
    fn complete_row_is_win() {
        let grid = [['O'; 3], ['-'; 3], ['-'; 3]];
        assert!(is_winning_board(Board { grid }));
    }

    #[test]
    fn diagonal_is_win() {
        let grid = [['O', '-', '-'], ['-', 'O', '-'], ['-', '-', 'O']];
        assert!(is_winning_board(Board { grid }));
    }

    #[test]
    fn complete_column_is_win() {
        let grid = [['O', '-', '-'], ['O', '-', '-'], ['O', '-', '-']];
        assert!(is_winning_board(Board { grid }));
    }

    #[test]
    fn combined_row_is_no_win() {
        let grid = [['O', 'X', 'X'], ['-', '-', '-'], ['-', '-', '-']];
        assert!(!is_winning_board(Board { grid }));
    }

    #[test]
    fn combined_column_is_no_win() {
        let grid = [['O', '-', '-'], ['X', '-', '-'], ['O', '-', '-']];
        assert!(!is_winning_board(Board { grid }));
    }

    #[test]
    fn combined_diagonal_is_no_win() {
        let grid = [['O', '-', '-'], ['-', 'X', '-'], ['-', '-', 'O']];
        assert!(!is_winning_board(Board { grid }));
    }
}
