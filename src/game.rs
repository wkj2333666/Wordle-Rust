use colored::Colorize;
use itertools::izip;
use rand::SeedableRng;
use rand::prelude::SliceRandom;
use std::collections::{BTreeMap, HashMap};

pub const MAX_ATTEMPTS: usize = 6;
pub const WORD_LENGTH: usize = 5;

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub enum CharStatus {
    Correct,
    Misplaced,
    Wrong,
    Unknown,
}

pub struct GuessResult {
    pub content: String,
    pub status: [CharStatus; WORD_LENGTH],
    pub keyboard: BTreeMap<char, CharStatus>,
}

impl GuessResult {
    fn new(_content: &str) -> Self {
        let mut new_self = Self {
            content: _content.to_string(),
            status: [CharStatus::Unknown; WORD_LENGTH],
            keyboard: BTreeMap::new(),
        };

        for key in 'a'..='z' {
            new_self.keyboard.insert(key, CharStatus::Unknown);
        }

        new_self
    }

    /// Clone the last keyboard, but with the new content and unknown status
    fn clone(&self, _content: &str) -> Self {
        Self {
            content: _content.to_string(),
            status: [CharStatus::Unknown; WORD_LENGTH],
            keyboard: self.keyboard.clone(),
        }
    }

    fn print(&self, is_tty: bool) {
        for (status, guess_char) in self.status.iter().zip(self.content.chars()) {
            match status {
                CharStatus::Correct => {
                    if is_tty {
                        print!("{}", guess_char.to_string().to_uppercase().color("green"))
                    } else {
                        print!("G")
                    }
                }
                CharStatus::Misplaced => {
                    if is_tty {
                        print!("{}", guess_char.to_string().to_uppercase().color("yellow"))
                    } else {
                        print!("Y")
                    }
                }
                CharStatus::Wrong => {
                    if is_tty {
                        print!("{}", guess_char.to_string().to_uppercase().color("red"))
                    } else {
                        print!("R")
                    }
                }
                CharStatus::Unknown => {
                    if is_tty {
                        print!("{}", guess_char.to_string().to_uppercase())
                    } else {
                        print!("X")
                    }
                }
            }
        }
        print!(" ");

        for (key, status) in self.keyboard.iter() {
            match status {
                CharStatus::Correct => {
                    if is_tty {
                        print!("{}", key.to_string().to_uppercase().color("green"))
                    } else {
                        print!("G")
                    }
                }
                CharStatus::Misplaced => {
                    if is_tty {
                        print!("{}", key.to_string().to_uppercase().color("yellow"))
                    } else {
                        print!("Y")
                    }
                }
                CharStatus::Wrong => {
                    if is_tty {
                        print!("{}", key.to_string().to_uppercase().color("red"))
                    } else {
                        print!("R")
                    }
                }
                CharStatus::Unknown => {
                    if is_tty {
                        print!("{}", key.to_string().to_uppercase())
                    } else {
                        print!("X")
                    }
                }
            }
        }
        println!();
    }
}

pub struct Guess {
    pub history: Vec<GuessResult>,
}

impl Guess {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn append(&mut self, guess: &str) {
        if self.history.is_empty() {
            self.history.push(GuessResult::new(guess));
        } else {
            self.history.push(self.history.last().unwrap().clone(guess));
        }
    }

    pub fn print(&self, is_tty: bool) {
        if is_tty {
            for guess in &self.history {
                guess.print(is_tty);
            }
        } else {
            self.history.last().unwrap().print(is_tty);
        }
    }

    /// check if the new guess is valid in difficult mode
    pub fn difficult_check(&mut self, is_difficult: bool, guess: &str) -> bool {
        if !is_difficult {
            return true;
        }

        if self.history.is_empty() {
            return true;
        }

        // check for Correct char
        for (last_guess_result, last_guess_char, this_guess_char) in izip!(
            self.history.last().unwrap().status.iter(),
            self.history.last().unwrap().content.chars(),
            guess.chars()
        ) {
            if *last_guess_result == CharStatus::Correct && this_guess_char != last_guess_char {
                return false;
            }
        }

        // check for WrongPlace char
        let mut last_guess_counts: HashMap<char, u32> = HashMap::new();
        let mut this_guess_counts: HashMap<char, u32> = HashMap::new();
        for (last_guess_char, last_guess_status) in izip!(
            self.history.last().unwrap().content.chars(),
            self.history.last().unwrap().status.iter(),
        ) {
            if *last_guess_status == CharStatus::Misplaced
                || *last_guess_status == CharStatus::Correct
            {
                *last_guess_counts.entry(last_guess_char).or_insert(0) += 1;
            }
        }
        for this_guess_char in guess.chars() {
            *this_guess_counts.entry(this_guess_char).or_insert(0) += 1;
        }
        for (last_guess_char, last_guess_char_count) in last_guess_counts {
            if last_guess_char_count > *this_guess_counts.get(&last_guess_char).unwrap_or(&0) {
                return false;
            }
        }

        true
    }
}

pub struct AnsChecker<'a> {
    ans: &'a str,
    counts: HashMap<char, i32>,
}

impl<'a> AnsChecker<'a> {
    pub fn new(ans: &'a str) -> Self {
        let mut counts = HashMap::new();
        for c in ans.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        Self { ans, counts }
    }

    pub fn check(&'a mut self, guess_result: &mut GuessResult) -> bool {
        let guess = guess_result.content.clone();

        // find correct
        for (idx, ans_char) in self.ans.chars().enumerate() {
            let guess_char = guess.chars().nth(idx).unwrap();
            if guess_char == ans_char {
                guess_result.status[idx] = CharStatus::Correct;
                *self.counts.get_mut(&ans_char).unwrap() -= 1;
            }
        }

        // find wrong position
        for (idx, guess_char) in guess.chars().enumerate() {
            if guess_result.status[idx] == CharStatus::Unknown
                && *self.counts.entry(guess_char).or_insert(0) > 0
            {
                guess_result.status[idx] = CharStatus::Misplaced;
                *self.counts.get_mut(&guess_char).unwrap() -= 1;
            } else if guess_result.status[idx] == CharStatus::Unknown
                && *self.counts.entry(guess_char).or_insert(0) <= 0
            {
                guess_result.status[idx] = CharStatus::Wrong;
            }
        }

        // update keyboard status
        for (status, guess_char) in guess_result.status.iter().zip(guess.chars()) {
            match status {
                CharStatus::Correct => {
                    *guess_result.keyboard.get_mut(&guess_char).unwrap() = CharStatus::Correct
                }
                CharStatus::Misplaced
                    if guess_result.keyboard[&guess_char] != CharStatus::Correct =>
                {
                    *guess_result.keyboard.get_mut(&guess_char).unwrap() = CharStatus::Misplaced
                }
                CharStatus::Wrong if guess_result.keyboard[&guess_char] == CharStatus::Unknown => {
                    *guess_result.keyboard.get_mut(&guess_char).unwrap() = CharStatus::Wrong
                }
                _ => (),
            };
        }

        // check game success
        guess_result
            .status
            .iter()
            .all(|x| *x == CharStatus::Correct)
    }
}

pub fn init_shuffle(seed: u64, final_words: &mut Vec<String>) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    final_words.shuffle(&mut rng);
}
