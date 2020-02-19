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
        if self.eval_score(Player::One) == 0 {
            (0..3)
                .map(|x| (0..3).map(move |y| (x, y)))
                .flatten()
                .filter(|(x, y)| self.board[*x][*y].is_none())
                .for_each(f);
        }
    }

    fn eval_score(&self, player: Player) -> i32 {
        (|| {
            for i in 0..3 {
                // Columns
                match (self.board[i][0], self.board[i][1], self.board[i][2]) {
                    (Some(x), Some(y), Some(z)) if x == y && y == z => return Some(x),
                    _ => {}
                }
                // Rows
                match (self.board[0][i], self.board[1][i], self.board[2][i]) {
                    (Some(x), Some(y), Some(z)) if x == y && y == z => return Some(x),
                    _ => {}
                }
            }

            // Diagonals
            match (self.board[0][0], self.board[1][1], self.board[2][2]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => return Some(x),
                _ => {}
            }
            match (self.board[2][0], self.board[1][1], self.board[0][2]) {
                (Some(x), Some(y), Some(z)) if x == y && y == z => return Some(x),
                _ => {}
            }

            None
        })()
        .map(|p| if p == player { 1 } else { -1 })
        .unwrap_or(0)
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
        for i in 0..3 {
            writeln!(
                f,
                "|{} {} {}|",
                p2c(self.board[0][i]),
                p2c(self.board[1][i]),
                p2c(self.board[2][i])
            )?;
        }
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
        println!("{}", board);

        if !board.can_move() {
            let p1 = board.eval_score(Player::One);
            println!(
                "You {}!",
                if p1 > 0 {
                    "won"
                } else if p1 < 0 {
                    "lost"
                } else {
                    "drew"
                }
            );
            break;
        }

        if board.next_player() == Player::One {
            let (x, y) = loop {
                let mut buf = String::new();
                println!("Enter a location(i.e: 0 1)");
                std::io::stdin().read_line(&mut buf).unwrap();
                match buf
                    .split_whitespace()
                    .map(|s| s.parse())
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(v) if v.len() == 2 => break (v[0], v[1]),
                    _ => {}
                }
            };
            board.apply_move((x, y));
        } else {
            println!("AI is thinking...");
            if let Some(mov) = board.solve(8) {
                board.apply_move(mov);
            } else {
                break;
            }
        }
    }
}
