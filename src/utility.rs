use std::{collections::HashMap, cmp::max_by};

use crate::deck::Card;

pub fn calc_hand_value(
    player_cards: &Vec<Card>,
    board_cards: &Vec<Card>,
) -> i32 {
    let mut hand = player_cards.clone();
    hand.append(&mut board_cards.clone());  
    let mut counts: HashMap<i32, i32>;

    for card in &hand {
        if !counts.contains_key(&card.value) {
            counts[&card.value] = 1;
        }
        counts[&card.value] += 1;
    }
    
    if let Some(list) = has_two_pair(&counts) {
        return 400 + max_by(list[0], list[1], compare);
    }

    if let Some(val) = has_quadruple(&counts) {
        return 300 + val;
    }

    if let Some(val) = has_triple(&counts) {
        return 200 + val;
    }

    if let Some(val) = has_pair(&counts) {
        return 100 + val;
    }

    return 0;
}

pub fn has_two_pair(counts: &HashMap<i32, i32>) -> Option<Vec<i32>> {
    let mut found = vec![];

    for (key, value) in counts {
        if *value == 2 {
            found.push(*key); 
        }
    }
    if found.len() == 2 {
        return Some(found);
    }
    return None;
}

pub fn has_quadruple(counts: &HashMap<i32, i32>) -> Option<i32> {
    has_several(counts, 4)   
}

pub fn has_triple(counts: &HashMap<i32, i32>) -> Option<i32> {
    has_several(counts, 3)
}

pub fn has_pair(counts: &HashMap<i32, i32>) -> Option<i32> {
    has_several(counts, 2)
}

pub fn has_several(counts: &HashMap<i32, i32>, number: i32) -> Option<i32> {
    for (key, value) in counts {
        if *value == number {
            return Some(*key);
        }
    }
    return None;
}
