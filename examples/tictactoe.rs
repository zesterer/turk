use std::fmt;
use turk::{GameState, Player};

#[derive(Clone)]
struct TicTacToe {
    board: [[Option<Player>; 3]; 3],
    next_player: Player,
}

impl GameState for TicTacToe {
    type Move = (usize, usize);

    fn next_player(&self) -> Player {
        self.next_player
    }

    fn apply_move(&mut self, (x, y): Self::Move) {
        self.board[x][y] = Some(self.next_player);
        self.next_player = !self.next_player;
    }

    fn for_each_move<F: FnMut(Self::Move)>(&self, f: F) {
        if self.eval_score() == 0 {
            (0..3)
                .map(|x| {
                    (0..3)
                        .filter(move |y| self.board[x][*y].is_none())
                        .map(move |y| (x, y))
                })
                .flatten()
                .for_each(f);
        }
    }

    fn eval_score(&self) -> i32 {
        // Columns
        for i in 0..3 {
            match (self.board[i][0], self.board[i][1], self.board[i][2]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => return 1,
                _ => {}
            }
        }

        // Rows
        for i in 0..3 {
            match (self.board[0][i], self.board[1][i], self.board[2][i]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => return 1,
                _ => {}
            }
        }

        // Diagonals
        match (self.board[0][0], self.board[1][1], self.board[2][2]) {
            (Some(x), Some(y), Some(z)) if x == y && y == z => return 1,
            _ => {}
        }
        match (self.board[2][0], self.board[1][1], self.board[0][2]) {
            (Some(x), Some(y), Some(z)) if x == y && y == z => return 1,
            _ => {}
        }

        0
    }
}

impl fmt::Display for TicTacToe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn p2c(player: Option<Player>) -> char {
            match player {
                Some(Player::One) => 'x',
                Some(Player::Two) => 'o',
                None => ' ',
            }
        }

        writeln!(f, ",-----,")?;
        writeln!(
            f,
            "|{} {} {}|",
            p2c(self.board[0][0]),
            p2c(self.board[1][0]),
            p2c(self.board[2][0])
        )?;
        writeln!(
            f,
            "|{} {} {}|",
            p2c(self.board[0][1]),
            p2c(self.board[1][1]),
            p2c(self.board[2][1])
        )?;
        writeln!(
            f,
            "|{} {} {}|",
            p2c(self.board[0][2]),
            p2c(self.board[1][2]),
            p2c(self.board[2][2])
        )?;
        writeln!(f, "'-----'")?;
        Ok(())
    }
}

fn main() {
    let mut board = TicTacToe {
        board: [[None; 3]; 3],
        next_player: Player::One,
    };

    loop {
        if let Some(mov) = board.solve_depth(Player::One, 8) {
            println!("{}", board);
            board.apply_move(mov);
        } else {
            break;
        }
        if let Some(mov) = board.solve_depth(Player::Two, 8) {
            println!("{}", board);
            board.apply_move(mov);
        } else {
            break;
        }
    }

    println!("{}", board);
}
