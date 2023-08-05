use bet::Bet;
use deck::{Deck, Hand, Bets, Cycler};
use lucas::Lucas;
use rand::thread_rng;

mod deck;
mod lucas;
mod bet;

fn main() {

    // create a full Deck
    let mut rng = thread_rng();
    let mut mydeck = Deck::default(); 
    mydeck.fill();

    // shuffle
    mydeck.shuffle(&mut rng);

    let mut hands = vec![Hand::default()];

    // therefore 0th and 1st indices are big and small blind (always)
    // once 2 cards have been dealt
    // skip to 1st indix and start betting
    let mut bets = Bets::default();

    bets.push(2);
    bets.push(1);
    for _ in 2..hands.len() {
        bets.push(0); // this is just setting up for bets for the first round
    }

    let mut current_player = Cycler::default(); 
    current_player.set_max_index(hands.len());

    for _ in 0..2 {
        current_player.reset();
        for i in &mut current_player {
            hands[i].push(mydeck.draw_card());
        }
    }

    let mut players: Vec<Box<dyn Bet>> = Vec::new();

    current_player.reset();
    for i in &mut current_player {
        players[i].bet();
    }
    
    
    // this is where your functions come in
    // array of lambda functions, where each variable in the array is a
    // reference to the function for that player
}
