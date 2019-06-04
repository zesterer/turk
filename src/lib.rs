use std::ops::Not;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Player {
    Agent,
    Opponent,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Agent => Player::Opponent,
            Player::Opponent => Player::Agent,
        }
    }
}

pub trait GameState: Clone {
    type Move: Clone;

    fn next_player(&self) -> Player;
    fn apply_move(&mut self, mov: Self::Move);
    fn get_moves(&self) -> Vec<Self::Move>;
    fn eval_score(&self) -> i32;

    fn solve_depth(&self, depth: usize) -> Option<Self::Move> {
        min(
            self,
            if let Player::Agent = self.next_player() { 1 } else { -1 },
            depth,
        ).1
    }
}

fn min<G: GameState>(game: &G, flip: i32, depth: usize) -> (i32, Option<G::Move>) {
    if depth == 0 {
        (game.eval_score(), None)
    } else {
        game.get_moves()
            .into_iter()
            .map(|mov| {
                let mut this = game.clone();
                this.apply_move(mov.clone());
                (flip * min(&this, -1, depth - 1).0, Some(mov))
            })
            .min_by(|(a, _), (b, _)| a.cmp(b))
            .unwrap_or_else(|| (game.eval_score(), None))
    }
}
