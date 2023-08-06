use rand::rngs::ThreadRng;

use crate::deck::{Bets, Card};

pub trait Bet {
    fn bet(
        &mut self, 
        rng: &mut ThreadRng,
        player_index: usize, 
        folds: Vec<bool>, 
        bets: Bets,
        board_cards: Vec<Card>, 
    ) -> Option<usize>;
}
