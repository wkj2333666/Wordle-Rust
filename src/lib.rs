mod args;
mod builtin_words;
pub mod game;
mod recorder;

#[cfg(not(target_arch = "wasm32"))]
use crate::args::Config;
use crate::game::{AnsChecker, Guess, MAX_ATTEMPTS};
use crate::recorder::SingleGameData;
use args::Args;
use clap::Parser;
#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashSet;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::{self, BufRead, BufReader};

#[cfg(target_arch = "wasm32")]
use chrono::Local;

#[cfg(target_arch = "wasm32")]
struct WebInterface {
    guess: String,
    result: String,
    win: Option<bool>,
}

#[cfg(target_arch = "wasm32")]
impl WebInterface {
    fn new() -> Self {
        WebInterface {
            guess: String::new(),
            result: String::new(),
            win: None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub struct Wordle {
    is_tty: bool,
    args: Args,
    game_recorder: recorder::GameRecorder,
    game_data: recorder::GameData,
    web_interface: WebInterface,
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Wordle {
    is_tty: bool,
    args: Args,
    game_recorder: recorder::GameRecorder,
    game_data: recorder::GameData,
}

impl Wordle {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Self {
        Wordle {
            is_tty: atty::is(atty::Stream::Stdout),
            args: Args::parse(),
            game_recorder: recorder::GameRecorder::new(),
            game_data: recorder::GameData::new(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Self {
        Wordle {
            is_tty: atty::is(atty::Stream::Stdout),
            args: Args {
                word: None,
                random: true,
                difficult: false,
                stats: true,
                day: Some(1),
                seed: Some(
                    Local::now()
                        .format("%Y%m%d")
                        .to_string()
                        .parse::<u64>()
                        .unwrap(),
                ),
                final_set: None,
                acceptable_set: None,
                state: None,
                config: None,
                debug: false,
            },
            game_recorder: recorder::GameRecorder::new(),
            game_data: recorder::GameData::new(),
            web_interface: WebInterface::new(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn game_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut final_words = Vec::<String>::new();
        let mut acceptable = Vec::<String>::new();
        self.init_game(&mut final_words, &mut acceptable)?;

        loop {
            self.start_one_game(&final_words, &acceptable);
            // Show stats if requested
            if self.args.stats {
                self.game_recorder.print();
            }

            // Day++
            if self.args.day.is_some() {
                self.args.day = Some(self.args.day.unwrap() + 1);
            }

            // Save game data if requested
            if self.args.state.is_some() {
                self.game_data.save(&self.args)?;
            }

            // Do not play again if word is specified
            if self.args.word.is_some() {
                break;
            }

            let play_again = loop {
                let mut buf = String::new();
                let is_eof = io::stdin().read_line(&mut buf).expect("Invalid input!");
                if is_eof == 0 {
                    break false;
                }
                buf = buf.trim().to_string();
                if buf == "Y" {
                    break true;
                } else if buf == "N" {
                    break false;
                }
                println!("Invalid input!");
            };

            if !play_again {
                break;
            }
        }
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// The main function for the Wordle game, implement your own logic here
    pub fn launch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // init
        self.load_game()?;

        self.game_loop()?;

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn init_game(
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
            game::init_shuffle(self.args.seed.unwrap(), final_words);
        }
        Ok(())
    }

    #[cfg(target_arch = "wasm32")]
    fn gen_answer(&self, final_words: &Vec<String>) -> String {
        final_words[self.args.day.unwrap() - 1 % final_words.len()].to_string()
    }

    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(not(target_arch = "wasm32"))]
    fn start_one_game(&mut self, final_words: &Vec<String>, acceptable: &Vec<String>) {
        let mut game_win = false;

        // Set answer
        let ans = self.gen_answer(final_words);

        let mut attempt = 0;
        let mut guess_results = Guess::new();

        // Guess until exceeds MAX_ATTEMPTS
        while attempt < MAX_ATTEMPTS {
            game_win = self.handle_one_guess(acceptable, &mut guess_results, &ans);

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

    // temporary
    #[cfg(not(target_arch = "wasm32"))]
    fn handle_one_guess(
        &mut self,
        acceptable: &Vec<String>,
        guess_results: &mut Guess,
        ans: &str,
    ) -> bool {
        // input guess
        let guess = loop {
            let mut tmp: String = String::new();
            io::stdin().read_line(&mut tmp).unwrap();
            tmp = tmp.trim().to_string();
            if acceptable.contains(&tmp) && guess_results.difficult_check(self.args.difficult, &tmp)
            {
                break tmp;
            }
            println!("INVALID");
        };
        self.game_recorder.add_tried_word(guess.clone());

        // check guess
        guess_results.append(&guess);
        let game_win = AnsChecker::new(&ans).check(&mut guess_results.history.last_mut().unwrap());

        // render output
        guess_results.print(self.is_tty);

        game_win
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_game(&mut self) -> Result<(), std::io::Error> {
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
                self.game_recorder.add_game(is_game_win, game_guesses.len());
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
