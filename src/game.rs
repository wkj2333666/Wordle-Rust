use crate::builtin_words;
use colored::Colorize;
use rand::Rng;
use std::{
    collections::{BTreeMap, HashMap},
    io,
};

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
    keyboard: BTreeMap<char, CharStatus>,
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

    fn clone(&self, _content: &str) -> Self {
        Self {
            content: _content.to_string(),
            status: [CharStatus::Unknown; WORD_LENGTH],
            keyboard: self.keyboard.clone(),
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
        print!(" ");

        for (key, status) in self.keyboard.iter() {
            match status {
                CharStatus::Correct => print!("{}", key.to_string().color("green")),
                CharStatus::WrongPosition => print!("{}", key.to_string().color("yellow")),
                CharStatus::TooMany => print!("{}", key.to_string().color("red")),
                CharStatus::Unknown => print!("{}", key),
            }
        }
        println!();
    }
}

struct Guess {
    history: Vec<GuessResult>,
}

impl Guess {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    fn append(&mut self, guess: &str) {
        if self.history.is_empty() {
            self.history.push(GuessResult::new(guess));
        } else {
            self.history.push(self.history.last().unwrap().clone(guess));
        }
    }

    fn print(&self) {
        for guess in &self.history {
            guess.print();
        }
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

    fn check(&'a mut self, guess_result: &mut GuessResult) -> bool {
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
                guess_result.status[idx] = CharStatus::WrongPosition;
                *self.counts.get_mut(&guess_char).unwrap() -= 1;
            } else if guess_result.status[idx] == CharStatus::Unknown
                && *self.counts.entry(guess_char).or_insert(0) <= 0
            {
                guess_result.status[idx] = CharStatus::TooMany;
            }
        }

        // update keyboard status
        for (status, guess_char) in guess_result.status.iter().zip(guess.chars()) {
            match status {
                CharStatus::Correct => {
                    *guess_result.keyboard.get_mut(&guess_char).unwrap() = CharStatus::Correct
                }
                CharStatus::WrongPosition
                    if guess_result.keyboard[&guess_char] != CharStatus::Correct =>
                {
                    *guess_result.keyboard.get_mut(&guess_char).unwrap() = CharStatus::WrongPosition
                }
                CharStatus::TooMany
                    if guess_result.keyboard[&guess_char] == CharStatus::Unknown =>
                {
                    *guess_result.keyboard.get_mut(&guess_char).unwrap() = CharStatus::TooMany
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

pub fn start_game() {
    let mut rng = rand::rng();

    let final_length = builtin_words::FINAL.len();
    let ans = builtin_words::FINAL[rng.random_range(0..final_length)];

    let mut attempt = 0;
    let mut guess_results = Guess::new();

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
        guess_results.append(&guess);
        let game_win: bool =
            AnsChecker::new(&ans).check(&mut guess_results.history.last_mut().unwrap());

        // render output
        guess_results.print();

        attempt += 1;
        if game_win {
            println!("CORRECT {attempt}");
            return;
        }
    }
    println!("FAILED {ans}");
}
