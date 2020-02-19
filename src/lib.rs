use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Player {
    One,
    Two,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

impl Into<usize> for Player {
    fn into(self) -> usize {
        match self {
            Player::One => 1,
            Player::Two => 2,
        }
    }
}

pub trait GameState: Clone {
    type Move: Clone;

    fn next_player(&self) -> Player;
    fn apply_move(&mut self, mov: Self::Move);
    fn for_each_move<F: FnMut(Self::Move)>(&self, f: F);
    fn eval_score(&self, player: Player) -> i32;

    fn can_move(&self) -> bool {
        let mut can_move = false;
        self.for_each_move(|_| can_move = true);
        can_move
    }

    fn solve(&self, depth: usize) -> Option<Self::Move> {
        minimax(self, self.next_player(), depth).1
    }
}

fn minimax<G: GameState>(game: &G, player: Player, depth: usize) -> (i32, Option<G::Move>) {
    if depth == 0 {
        (game.eval_score(player), None)
    } else {
        let mut max_mov = None;

        let min = game.next_player() != player;
        game.for_each_move(|mov| {
            let mut game = game.clone();
            game.apply_move(mov.clone());
            let score = minimax(&game, player, depth - 1).0;
            if max_mov
                .as_ref()
                .map(|(s, _)| (score > *s) ^ min)
                .unwrap_or(true)
            {
                max_mov = Some((score, mov));
            }
        });

        max_mov
            .map(|(score, mov)| (score, Some(mov)))
            .unwrap_or_else(|| (game.eval_score(player), None))
    }
}
