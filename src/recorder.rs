use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct GameStat {
    win: bool,
    attempts: usize,
}

impl GameStat {
    fn new(win: bool, attempts: usize) -> Self {
        Self { win, attempts }
    }
}

/// Record statical data
#[derive(Debug)]
pub struct GameRecorder {
    games: Vec<GameStat>,
    tried_words: HashMap<String, u32>,
    win: u32,
    lose: u32,
}

impl GameRecorder {
    pub fn new() -> Self {
        Self {
            games: Vec::new(),
            tried_words: HashMap::new(),
            win: 0,
            lose: 0,
        }
    }

    pub fn add_game(&mut self, win: bool, attempts: usize) {
        self.games.push(GameStat::new(win, attempts));
        if win {
            self.win += 1;
        } else {
            self.lose += 1;
        }
    }

    pub fn add_tried_word(&mut self, word: String) {
        *self.tried_words.entry(word).or_insert(0) += 1;
    }

    fn print_top_5_words(&self) {
        let mut sorted_words: Vec<(&str, u32)> = self
            .tried_words
            .iter()
            .map(|(word, count)| (word.as_str(), *count))
            .collect();
        sorted_words.sort_by(|a, b| {
            if b.1.cmp(&a.1) == Ordering::Equal {
                a.0.cmp(b.0)
            } else {
                b.1.cmp(&a.1)
            }
        });
        // for (word, count) in sorted_words.iter().take(5) {
        //     print!("{} {} ", word.to_uppercase(), count);
        // }
        let mut iter = sorted_words.iter().take(5).peekable();
        while let Some((word, count)) = iter.next() {
            if iter.peek().is_some() {
                print!("{} {} ", word.to_uppercase(), count);
            } else {
                print!("{} {}", word.to_uppercase(), count);
            }
        }
    }

    pub fn print(&self) {
        let mut average_attempts: f64 = 0.0;
        let num_of_wins: u32 = self.games.iter().fold(0, |acc, game| acc + game.win as u32);
        if num_of_wins != 0 {
            average_attempts =
                self.games.iter().fold(
                    0,
                    |acc, game| {
                        if game.win { acc + game.attempts } else { acc }
                    },
                ) as f64
                    / num_of_wins as f64;
        }
        println!("{} {} {:.2}", self.win, self.lose, average_attempts);
        self.print_top_5_words();
        println!();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleGameData {
    pub answer: String,
    pub guesses: Vec<String>,
}

/// Persistent storage
#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    #[serde(default)]
    pub total_rounds: u32,

    #[serde(default)]
    pub games: Vec<SingleGameData>,
}

impl GameData {
    pub fn new() -> Self {
        GameData {
            total_rounds: 0,
            games: Vec::new(),
        }
    }

    pub fn add_game(&mut self, guess_results: &crate::game::Guess, ans: &str) {
        self.total_rounds += 1;
        self.games.push(SingleGameData {
            answer: ans.to_string().to_uppercase(),
            guesses: guess_results
                .history
                .iter()
                .map(|guess_content| guess_content.content.clone().to_uppercase())
                .collect(),
        });
    }

    pub fn save(&self, args: &crate::args::Args) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = args.state.clone().unwrap();
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(file_path, json)?;
        Ok(())
    }
}
