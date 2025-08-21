use crate::builtin_words;
use colored::Colorize;
use rand::Rng;
use std::{collections::HashMap, io};

const MAX_ATTEMPTS: usize = 6;
const WORD_LENGTH: usize = 5;

#[derive(Copy, Clone, PartialEq)]
enum CharStatus {
    Correct,
    WrongPosition,
    TooMany,
    Unknown,
}

struct GuessResult {
    content: String,
    status: [CharStatus; WORD_LENGTH],
}

impl GuessResult {
    fn new(_content: &str) -> Self {
        Self {
            content: _content.to_string(),
            status: [CharStatus::Unknown; WORD_LENGTH],
        }
    }

    fn print(&self) {
        for (status, guess_char) in self.status.iter().zip(self.content.chars()) {
            match status {
                CharStatus::Correct => print!("{}", guess_char.to_string().color("green")),
                CharStatus::WrongPosition => print!("{}", guess_char.to_string().color("yellow")),
                CharStatus::TooMany => print!("{}", guess_char.to_string().color("red")),
                CharStatus::Unknown => print!("{}", guess_char),
            }
        }
        println!("");
    }
}

struct AnsChecker<'a> {
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

    fn check(&'a mut self, guess: &str) -> GuessResult {
        let mut guess_result = GuessResult::new(guess);

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
                guess_result.status[idx] = CharStatus::WrongPosition;
                *self.counts.get_mut(&guess_char).unwrap() -= 1;
            } else if guess_result.status[idx] == CharStatus::Unknown
                && *self.counts.entry(guess_char).or_insert(0) <= 0
            {
                guess_result.status[idx] = CharStatus::TooMany;
            }
        }

        guess_result
    }
}

pub fn start_game() {
    let mut rng = rand::rng();

    let final_length = builtin_words::FINAL.len();
    let ans = builtin_words::FINAL[rng.random_range(0..final_length)];

    let mut attempt = 0;

    while attempt < MAX_ATTEMPTS {
        // input
        let guess = loop {
            let mut tmp: String = String::new();
            io::stdin().read_line(&mut tmp).unwrap();
            tmp = tmp.trim().to_string();
            if builtin_words::ACCEPTABLE.contains(&tmp.as_str()) {
                break tmp;
            }
            println!("INVALID!");
        };

        // check
        let guess_result = AnsChecker::new(ans).check(&guess);

        // render output
        guess_result.print();

        attempt += 1;
    }
}
