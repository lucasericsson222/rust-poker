use crate::bet;

use bet::Bet;
use rand::{rngs::ThreadRng, Rng};

pub struct Lucas {}

impl Bet for Lucas {
    fn bet(&mut self, rng: &mut ThreadRng) -> Option<usize> {
        let fold = false; 

        if fold {
            return None;
        }

        return Some(rng.gen());
        // if your bet is less than the required bet to call
        // then it will bet all that is necessary to call
        // if that is too much, all in
        //
        // to raise, just put in a higher number than the previous bet
    }    
}
