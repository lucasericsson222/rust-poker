use std::{collections::HashMap, cmp};

use crate::deck::Card;

pub fn calc_hand_value(
    player_cards: &Vec<Card>,
    board_cards: &Vec<Card>,
) -> i32 {
    let mut hand = player_cards.clone();
    hand.append(&mut board_cards.clone());  
    let mut counts: HashMap<i32, i32> = HashMap::new();

    for card in &hand {
        if !counts.contains_key(&card.value) {
            counts.insert(card.value, 1);
        }
        *counts.get_mut(&card.value).unwrap() += 1;
    }
     
    
    
    if let Some(val1) = has_triple(&counts) {
        if let Some(val2) = has_pair(&counts) {
            return 500 + cmp::max(val1, val2);
        }
    }

    if let Some(list) = has_two_pair(&counts) {
        return 400 + cmp::max(list[0], list[1]);
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

pub fn has_straight(hand: &mut Vec<Card>) -> Option<i32> {

    
    hand.sort();

    let mut straight: i32 = 0;

    let mut count = 1;
    let mut previous = 0; 

    for i in 0..hand.len() {

        if previous != 0 {
            if previous == hand[i].value - 1 {
                count += 1;
            } else {
                count = 1;
            }
        } 

        if count >= 5 {
            straight = cmp::max(straight, hand[i].value);  
        }
        
        previous = hand[i].value;

    }

    if straight != 0 {
        return Some(straight);
    }

    return None;
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
    
    let mut max_val = 0;

    for (key, value) in counts {
        if *value == number {
            max_val = cmp::max(*key, max_val);
        }
    }

    if max_val != 0 {
        return Some(max_val);
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_pair_returns_true_when_pair() {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 2);
        counts.insert(3, 1);
        counts.insert(11, 1);
        counts.insert(14, 1);
        counts.insert(4, 1);
        counts.insert(8, 1);

        assert_eq!(has_pair(&counts).unwrap(), 2);
    }

    #[test]
    fn has_pair_returns_false_when_no_pair() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 1);
        counts.insert(3, 1);
        counts.insert(7, 1);
        counts.insert(11, 1);
        counts.insert(14, 1);
        counts.insert(4, 1);
        counts.insert(8, 1);
        
        assert_eq!(has_pair(&counts), None);
    }

    #[test]
    fn has_pair_returns_highest_pair_when_multiple() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 2);
        counts.insert(3, 1);
        counts.insert(11, 1);
        counts.insert(14, 2);
        counts.insert(8, 1);
        
        assert_eq!(has_pair(&counts).unwrap(), 14);
    }

    #[test]
    fn has_triple_returns_true_when_triple() {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 1);
        counts.insert(3, 3);
        counts.insert(7, 1);
        counts.insert(4, 1);
        counts.insert(8, 1);
        
        assert_eq!(has_triple(&counts).unwrap(), 3);
    }

    #[test]
    fn has_triple_returns_false_when_no_triple() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 1);
        counts.insert(3, 1);
        counts.insert(7, 1);
        counts.insert(11, 1);
        counts.insert(14, 1);
        counts.insert(4, 1);
        counts.insert(8, 1);
        
        assert_eq!(has_triple(&counts), None);
    }

    #[test]
    fn has_triple_returns_highest_triple_when_multiple() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(7, 3);
        counts.insert(11, 3);
        counts.insert(14, 1);
        
        assert_eq!(has_triple(&counts).unwrap(), 11);
    }

    #[test]
    fn has_quadruple_returns_true_when_quadruple() {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 1);
        counts.insert(3, 1);
        counts.insert(7, 4);
        counts.insert(8, 1);
        
        assert_eq!(has_quadruple(&counts).unwrap(), 7);
    }

    #[test]
    fn has_quadruple_returns_false_when_no_quadruple() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 1);
        counts.insert(3, 1);
        counts.insert(7, 1);
        counts.insert(11, 1);
        counts.insert(14, 1);
        counts.insert(4, 1);
        counts.insert(8, 1);
        
        assert_eq!(has_quadruple(&counts), None);
    }

    #[test]
    fn has_two_pair_returns_true_when_two_pair() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 2);
        counts.insert(3, 1);
        counts.insert(10, 2);
        counts.insert(12, 1);
        counts.insert(14, 1);

        let mut result = has_two_pair(&counts).unwrap();
        result.sort();
        assert_eq!(result, vec![2, 10]);
    }

    #[test]
    fn has_two_pair_returns_false_when_one_pair() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 2);
        counts.insert(3, 1);
        counts.insert(10, 1);
        counts.insert(12, 1);
        counts.insert(14, 1);
        counts.insert(11, 1);

        let result = has_two_pair(&counts);

        assert_eq!(result, None);
    }

    #[test]
    fn has_two_pair_returns_false_when_no_pair() {

        let mut counts: HashMap<i32, i32> = HashMap::new();

        counts.insert(2, 1);
        counts.insert(3, 1);
        counts.insert(10, 1);
        counts.insert(12, 1);
        counts.insert(15, 1);
        counts.insert(4, 1);
        counts.insert(8, 1);

        let result = has_two_pair(&counts);

        assert_eq!(result, None);
    }

    #[test]
    fn has_straight_return_true_when_straight() {

        let mut hand = vec![
            Card {value: 2, suite: "H".to_string()},
            Card {value: 3, suite: "D".to_string()},
            Card {value: 4, suite: "C".to_string()},
            Card {value: 5, suite: "S".to_string()},
            Card {value: 6, suite: "C".to_string()},
            Card {value: 11, suite: "D".to_string()},
            Card {value: 10, suite: "S".to_string()},
        ];

        assert_eq!(has_straight(&mut hand).unwrap(), 6);
    }

}
