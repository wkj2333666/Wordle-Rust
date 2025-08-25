use crate::Wordle;
use crate::args::Config;
use crate::builtin_words;
use crate::recorder::SingleGameData;
use colored::Colorize;
use itertools::izip;
use rand::SeedableRng;
use rand::prelude::SliceRandom;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

const MAX_ATTEMPTS: u32 = 6;
const WORD_LENGTH: usize = 5;

#[derive(Copy, Clone, PartialEq)]
enum CharStatus {
    Correct,
    WrongPosition,
    TooMany,
    Unknown,
}

pub struct GuessResult {
    pub content: String,
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
                CharStatus::WrongPosition => {
                    if is_tty {
                        print!("{}", guess_char.to_string().to_uppercase().color("yellow"))
                    } else {
                        print!("Y")
                    }
                }
                CharStatus::TooMany => {
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
                CharStatus::WrongPosition => {
                    if is_tty {
                        print!("{}", key.to_string().to_uppercase().color("yellow"))
                    } else {
                        print!("Y")
                    }
                }
                CharStatus::TooMany => {
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

    fn append(&mut self, guess: &str) {
        if self.history.is_empty() {
            self.history.push(GuessResult::new(guess));
        } else {
            self.history.push(self.history.last().unwrap().clone(guess));
        }
    }

    fn print(&self, is_tty: bool) {
        if is_tty {
            for guess in &self.history {
                guess.print(is_tty);
            }
        } else {
            self.history.last().unwrap().print(is_tty);
        }
    }

    /// check if the new guess is valid in difficult mode
    fn difficult_check(&mut self, is_difficult: bool, guess: &str) -> bool {
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
            if *last_guess_status == CharStatus::WrongPosition
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

impl Wordle {
    pub fn init_game(
        &mut self,
        final_words: &mut Vec<String>,
        acceptable: &mut Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.args.final_set.is_some() {
            let final_set_file = File::open(&self.args.final_set.as_ref().unwrap())?;
            *final_words = io::BufReader::new(final_set_file)
                .lines()
                .map(|line| line.unwrap())
                .collect();
        } else {
            *final_words = builtin_words::FINAL
                .iter()
                .map(|word| word.to_string())
                .collect();
        }

        if self.args.acceptable_set.is_some() {
            let acceptable_set_file = File::open(&self.args.acceptable_set.as_ref().unwrap())?;
            *acceptable = io::BufReader::new(acceptable_set_file)
                .lines()
                .map(|line| line.unwrap())
                .collect();
        } else {
            *acceptable = builtin_words::ACCEPTABLE
                .iter()
                .map(|word| word.to_string())
                .collect();
        }

        // check if final is a subset of acceptable
        let hash_set_acceptable = acceptable.iter().collect::<HashSet<_>>();
        if !final_words
            .iter()
            .all(|word| hash_set_acceptable.contains(word))
        {
            return Err("Final words must be a subset of acceptable words!".into());
        }

        if self.args.random {
            init_shuffle(self.args.seed.unwrap(), final_words);
        }
        Ok(())
    }

    fn gen_answer(&self, final_words: &Vec<String>) -> String {
        if self.args.random {
            final_words[self.args.day.unwrap() - 1 % final_words.len()].to_string()
        } else {
            if let Some(given_answer) = &self.args.word {
                assert!(final_words.contains(&given_answer));
                given_answer.clone()
            } else {
                loop {
                    let mut tmp = String::new();
                    io::stdin().read_line(&mut tmp).unwrap();
                    tmp = tmp.trim().to_string();
                    if final_words.contains(&tmp) {
                        break tmp;
                    }
                    println!("INVALID");
                }
            }
        }
    }

    pub fn start_one_game(&mut self, final_words: &Vec<String>, acceptable: &Vec<String>) {
        let mut game_win = false;

        // Set answer
        let ans = self.gen_answer(final_words);

        let mut attempt = 0;
        let mut guess_results = Guess::new();

        // Guess until exceeds MAX_ATTEMPTS
        while attempt < MAX_ATTEMPTS {
            // input guess
            let guess = loop {
                let mut tmp: String = String::new();
                io::stdin().read_line(&mut tmp).unwrap();
                tmp = tmp.trim().to_string();
                if acceptable.contains(&tmp)
                    && guess_results.difficult_check(self.args.difficult, &tmp)
                {
                    break tmp;
                }
                println!("INVALID");
            };
            self.game_recorder.add_tried_word(guess.clone());

            // check guess
            guess_results.append(&guess);
            game_win = AnsChecker::new(&ans).check(&mut guess_results.history.last_mut().unwrap());

            // render output
            guess_results.print(self.is_tty);

            attempt += 1;
            if game_win {
                break;
            }
        }

        // Record this game
        self.game_recorder.add_game(game_win, attempt);
        if self.args.state.is_some() {
            self.game_data.add_game(&guess_results, &ans);
        }

        if game_win {
            println!("CORRECT {attempt}");
        } else {
            println!("FAILED {}", ans.to_uppercase());
        }
    }

    pub fn load_game(&mut self) -> Result<(), std::io::Error> {
        if self.args.state.is_some() {
            if let Result::Ok(data_file) = File::open(self.args.state.as_ref().unwrap()) {
                self.game_data = serde_json::from_reader(BufReader::new(data_file))?;
            } // else: no such file, ignore, and use a empty game data

            for SingleGameData {
                answer: game_answer,
                guesses: game_guesses,
            } in &self.game_data.games
            {
                let is_game_win = game_answer == game_guesses.last().unwrap();
                self.game_recorder
                    .add_game(is_game_win, game_guesses.len() as u32);
                for one_guess in game_guesses {
                    self.game_recorder
                        .add_tried_word(one_guess.clone().to_lowercase());
                }
            }
        }

        // load config
        if let Some(config_path) = self.args.config.as_ref() {
            let config_file = File::open(config_path)?;
            let config: Config = serde_json::from_reader(BufReader::new(config_file))?;

            if self.args.word.is_none() {
                self.args.word = config.word;
            }
            if self.args.random == false {
                self.args.random = config.random;
            }
            if self.args.difficult == false {
                self.args.difficult = config.difficult;
            }
            if self.args.stats == false {
                self.args.stats = config.stats;
            }
            // day & seed
            if self.args.word.is_none() {
                if self.args.day.is_none() {
                    self.args.day = config.day;
                } else {
                    self.args.day = Some(1);
                }

                if self.args.seed.is_none() {
                    self.args.seed = config.seed;
                } // and if the config does not specify seed, use default value
                if self.args.seed.is_none() {
                    self.args.seed = Some(114514);
                }
            }
            if self.args.final_set.is_none() {
                self.args.final_set = config.final_set;
            }
            if self.args.acceptable_set.is_none() {
                self.args.acceptable_set = config.acceptable_set;
            }
        }

        // even if there is no config file, a default seed must be specified
        if self.args.random {
            if self.args.seed.is_none() {
                self.args.seed = Some(114514);
            }
            // so does the day
            if self.args.day.is_none() {
                self.args.day = Some(1);
            }
        }

        Ok(())
    }
}

fn init_shuffle(seed: u64, final_words: &mut Vec<String>) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    final_words.shuffle(&mut rng);
}
