use std::iter::Enumerate;

use bet::Bet;
use deck::{Deck, Hand, Bets, Cycler, Card};
use lucas::Lucas;
use rand::{thread_rng, rngs::ThreadRng};
use utility::calc_hand_value;

mod deck;
mod lucas;
mod bet;
mod utility;

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
    let mut board_cards: Vec<Card> = vec![];
    let mut money = vec![];

    for _ in &hands {
        money.push(100); 
    }

    bets = dbg!(bets);

    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng, &board_cards, &mut money);
   
    board_cards.push(mydeck.draw_card());
    board_cards.push(mydeck.draw_card());
    board_cards.push(mydeck.draw_card());
    
    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng, &board_cards, &mut money);

    board_cards.push(mydeck.draw_card());

    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng, &board_cards, &mut money);

    board_cards.push(mydeck.draw_card());

    commence_betting(&mut current_player, &mut folds, &mut players, &mut bets, &mut rng, &board_cards, &mut money);


    let mut scores = vec![];
    for hand in &mut hands {
        scores.push(calc_hand_value(&hand.hand, &board_cards)); 
    }

    let mut highest = 0;
    let mut highest_index = 0;
    for (i, score) in scores.iter().enumerate() {
        if *score > highest {
            highest = *score;
            highest_index = i;
        }
    }

    println!("Player {0} won the game with the hand score of {1}", highest_index, highest);
    println!("Their hand was");
    println!("{:#?}", hands[highest_index]);
    println!("The cards in play were");
    println!("{:#?}", board_cards);
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
    board_cards: &Vec<Card>, 
    money: &mut Vec<i32>,
) {
    let mut raise = bet_round(current_player, folds, players, bets, rng, board_cards, money);

    while raise {
        raise = bet_round(current_player, folds, players, bets, rng, board_cards, money);
    }

    modify_money(bets, money);

    bets.increment_turn();

    for _ in 0..players.len() {
        bets.push(0);
    }
}

fn modify_money(bets: &mut Bets, money: &mut Vec<i32>,) {
    for (i, player_money) in money.clone().iter_mut().enumerate() {
        *player_money -= *bets.get_current(i).unwrap() as i32;
    }
}

fn bet_round(
    current_player: &mut Cycler, 
    folds: &mut Vec<bool>,
    players: &mut Vec<Box<dyn Bet>>,
    bets: &mut Bets,
    rng: &mut ThreadRng,
    board_cards: &Vec<Card>,
    money: &Vec<i32>,
) -> bool {
    let mut raise = false;
    current_player.reset();

    for i in current_player {

        if folds[i] {
            continue;
        }

        let maybe_bet = players[i].bet(rng, i, folds.clone(), bets.clone(), board_cards.clone());

        if let Some(mut bet) = maybe_bet {

            let previous_bet = bets.get_max_bet();

            if bet > 10000 {
                bet = 10000;
            }
            if bet as i32 > money[i] {
                bet = money[i] as usize;
                println!("Player {i} goes all in");
            } else {
                if bet < previous_bet {
                    bet = previous_bet;
                    println!("Player {0} called", i);
                }
            }
            if bet > previous_bet {
                raise = true; 
                println!("Player {0} bet {1}", i, &bet); 
            }

            bets.set(i, bet);
            
        } else {
            println!("Player {0} folded", i);
            folds[i] = true;
        }
    }
    return raise;
}
