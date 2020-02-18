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

pub trait GameState: Clone {
    type Move: Clone;

    fn next_player(&self) -> Player;
    fn apply_move(&mut self, mov: Self::Move);
    fn for_each_move<F: FnMut(Self::Move)>(&self, f: F);
    fn eval_score(&self) -> i32;

    fn solve_depth(&self, player: Player, depth: usize) -> Option<Self::Move> {
        min(
            self,
            if self.next_player() == player { -1 } else { 1 },
            depth,
        )
        .1
    }
}

fn min<G: GameState>(game: &G, flip: i32, depth: usize) -> (i32, Option<G::Move>) {
    if depth == 0 {
        (game.eval_score(), None)
    } else {
        let mut min_mov = None;

        game.for_each_move(|mov| {
            let mut this = game.clone();
            this.apply_move(mov.clone());
            let cost = flip * min(&this, -1, depth - 1).0;
            if min_mov
                .as_ref()
                .map(|(_, c): &(G::Move, i32)| cost > *c)
                .unwrap_or(true)
            {
                min_mov = Some((mov, cost));
            }
        });

        (game.eval_score(), min_mov.map(|(mov, _)| mov))
    }
}
