use bet::Bet;
use deck::{Deck, Hand, Bets, Cycler};
use lucas::Lucas;
use rand::{thread_rng, rngs::ThreadRng};

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

    bets.push(1);
    bets.push(2);
    for _ in 2..hands.len() {
        bets.push(0); // this is just setting up for bets for the first round
    }

    let mut current_player = Cycler::default(); 
    current_player.set_max_index(hands.len());

    deal_card(&mut current_player, &mut hands, &mut mydeck);
    deal_card(&mut current_player, &mut hands, &mut mydeck);

    let mut players: Vec<Box<dyn Bet>> = Vec::new();

    players.push(Box::new(Lucas {}));

    let mut folds = vec![false];

    bets = dbg!(bets);

    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng);
   
    deal_card(&mut current_player, &mut hands, &mut mydeck);

    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng);

    deal_card(&mut current_player, &mut hands, &mut mydeck);

    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng);

    deal_card(&mut current_player, &mut hands, &mut mydeck);

    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng);
}

fn deal_card(
    current_player: &mut Cycler, 
    hands: &mut Vec<Hand>,
    mydeck: &mut Deck,
) {
    current_player.reset();
    for i in current_player {
        hands[i].push(mydeck.draw_card());
    }
    for hand in hands {
        println!("----------");
        dbg!(hand);
    }
}

fn commence_betting(
    current_player: &mut Cycler, 
    folds: &mut Vec<bool>,
    players: &mut Vec<Box<dyn Bet>>,
    bets: &mut Bets,
    rng: &mut ThreadRng,
) {
    let mut raise = bet_round(current_player, folds, players, bets, rng);

    while raise {
        raise = bet_round(current_player, folds, players, bets, rng);
    }

    bets.increment_turn();
    for _ in 0..players.len() {
        bets.push(0);
    }
}

fn bet_round(
    current_player: &mut Cycler, 
    folds: &mut Vec<bool>,
    players: &mut Vec<Box<dyn Bet>>,
    bets: &mut Bets,
    rng: &mut ThreadRng,
) -> bool {
    let mut raise = false;
    current_player.reset();
    for i in current_player {

        if folds[i] {
            continue;
        }

        let maybe_bet = players[i].bet(rng);

        if let Some(mut bet) = maybe_bet {

            let previous_bet = bets.get_max_bet();

            if bet > previous_bet {
                raise = true; 
                println!("Player {0} bet {1}", i, &bet); 
            }
            if bet < previous_bet {
                bet = previous_bet;
                println!("Player {0} called", i);
            }

            bets.set(i, bet.clone());
            
        } else {
            println!("Player {0} folded", i);
            folds[i] = true;
        }
    }
    return raise;
}

fn raise() {
    panic!();
}
