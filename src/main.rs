mod players;
mod win_condition;
mod board;

use win_condition::is_winning_grid;
use players::{HumanPlayer, Player};

fn main() {
    let player_one = HumanPlayer { sign: 'O' };
    let player_two = HumanPlayer { sign: 'X' };

    let game = Game::new((player_one, player_two));

    game.play();
}

struct Game<T>
where
    T: Player,
{
    board: board::Board,
    players: (T, T),
    current_player: T,
    is_over: bool,
}

impl<T> Game<T>
where
    T: Player,
{
    fn new(players: (T, T)) -> Game<T> {
        let board = board::Board::new();
        let current_player = players.0;
        let players = players;
        let is_over = false;
        Game {
            board,
            current_player,
            players,
            is_over,
        }
    }

    fn next_turn(&self) -> Self {
        Game {
            board: self.board,
            is_over: is_winning_grid(self.board),
            players: self.players,
            current_player: if self.current_player == self.players.0 {
                self.players.1
            } else {
                self.players.0
            },
        }
    }

    fn play(&self) {
        if self.is_over {
            println!("Game over");
        } else {
            match self.current_player.get_coordinate(self.board) {
                Ok(coordinate) => match self.board
                    .add_value(coordinate, self.current_player.get_sign())
                {
                    Ok(b) => {
                        let new_game = Game {
                            board: b,
                            players: self.players,
                            is_over: self.is_over,
                            current_player: self.current_player,
                        };
                        let new_game = new_game.next_turn();
                        println!("{}", new_game.board);
                        new_game.play();
                    }
                    Err(e) => {
                        println!("{}", e);
                        println!("{}", self.board);
                        self.play();
                    }
                },
                Err(e) => {
                    println!("{}", e);
                    println!("{}", self.board);
                    self.play();
                }
            }
        }
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;
    use board::Board;

    #[test]
    fn add_value_in_empty_field_adds_value() {
        let sign = 'X';
        let board = board::Board::new();
        let player = TestPlayer { sign };
        let result_board = board.add_value((0, 0), player.get_sign()).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_with_custom_sign_uses_sign() {
        let sign = 'A';
        let board = board::Board::new();
        let player = TestPlayer { sign };
        let result_board = board.add_value((0, 0), player.get_sign()).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_outside_of_bounds_is_invalid() {
        let board = board::Board::new();
        let player = TestPlayer { sign: 'X' };
        let result = board.add_value((3, 3), player.get_sign());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn add_value_to_existing_field_is_invalid() {
        let board = board::Board {
            grid: [['X'; 3], ['-'; 3], ['-'; 3]],
        };

        let player = TestPlayer { sign: 'X' };
        let result = board.add_value((0, 0), player.get_sign());
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn play_game_with_game_over_does_not_panic() {
        let players = (TestPlayer { sign: 'O' }, TestPlayer { sign: 'X' });
        let mut game = Game::new(players);
        game.is_over = true;

        game.play();
    }

    #[test]
    fn next_turn_switches_current_player() {
        let players = (TestPlayer { sign: 'O' }, TestPlayer { sign: 'X' });
        let game = Game::new(players);
        let initial_player = game.current_player;

        let game_after_turn = game.next_turn();
        let players_are_same = initial_player == game_after_turn.current_player;
        assert_eq!(players_are_same, false)
    }

    #[derive(PartialEq, Clone, Copy)]
    struct TestPlayer {
        pub sign: char,
    }

    impl Player for TestPlayer {
        fn get_sign(&self) -> char {
            self.sign
        }

        fn get_coordinate(&self, _board: Board) -> Result<(usize, usize), String> {
            Err("Not implemented".to_string())
        }
    }
}
