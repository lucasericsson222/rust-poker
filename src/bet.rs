use rand::rngs::ThreadRng;

pub trait Bet {
    fn bet(&mut self, rng: &mut ThreadRng) -> Option<usize>;
}
