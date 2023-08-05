use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub struct Deck {
    deck: Vec<Card>,      
}

impl Deck {
    pub fn fill(&mut self) {
        for value in 2..=14 {
            for suite in vec!["H","S","C","D"] {
                self.deck.push(Card {
                    value,
                    suite: suite.to_string(),
                })
            }
        }
    }
    pub fn shuffle(&mut self, mut rng: &mut ThreadRng) {
        self.deck.shuffle(&mut rng);
    }

    pub fn draw_card(&mut self) -> Card {
        if self.deck.len() == 0 {
            self.fill();
        }                 

        self.deck.pop().unwrap()
    }
}

impl Default for Deck {
    fn default() -> Self {
        Deck {
            deck: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Card {
    value: i32,
    suite: String,
}

#[derive(Debug)]
pub struct Hand {
    hand: Vec<Card>,
}

impl Hand {
    pub fn push(&mut self, card: Card) {
        self.hand.push(card);
    }
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            hand: vec![],
        }
    }
}

#[derive(Debug)]
pub struct Bets {
    current_turn: usize,
    bets: Vec<Vec<i32>>
}

impl Bets {
    pub fn push(&mut self, bet: i32) {
        self.bets[self.current_turn].push(bet);
    }

    pub fn increment_turn(&mut self) {
        self.current_turn += 1;
    }
    
}

impl Default for Bets {
    fn default() -> Self {
        Bets {
            current_turn: 0,
            bets: vec![vec![]],
        }
    }
}

#[derive(Debug)]
pub struct Cycler {
    current_index: usize,
    max_index: usize,
    start_index: usize,
    stop: bool,
}

impl Cycler {
    pub fn set_max_index(&mut self, index: usize) {
       self.max_index = index; 
    }
    pub fn set_start_index(&mut self, index: usize) {
        self.start_index = index;
    }
    pub fn reset(&mut self) {
        self.current_index = self.start_index;
        self.stop = false;
    }
}

impl Default for Cycler {
    fn default() -> Self {
        Cycler {
            current_index: 0,
            max_index: 0,
            start_index: 0,
            stop: false,
        }
    }
}

impl Iterator for Cycler {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let current_val = self.current_index;

        self.current_index += 1;

        if self.current_index == self.max_index {
            self.current_index = 0;
        }
         
        if self.current_index == self.start_index {
            self.stop = true;
        }
        return Some(current_val);
    }
}
