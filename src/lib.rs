use clap::Parser;
use std::io;
mod args;
use args::Args;

mod builtin_words;
mod game;
mod recorder;

struct WebInterface {
    guess: String,
    result: String,
    win: Option<bool>,
}

impl WebInterface {
    fn new() -> Self {
        WebInterface {
            guess: String::new(),
            result: String::new(),
            win: None,
        }
    }
}

pub struct Wordle {
    is_tty: bool,
    args: Args,
    game_recorder: recorder::GameRecorder,
    game_data: recorder::GameData,
    web_interface: WebInterface,
    use_web: bool,
}

impl Wordle {
    pub fn new() -> Self {
        Wordle {
            is_tty: atty::is(atty::Stream::Stdout),
            args: Args::parse(),
            game_recorder: recorder::GameRecorder::new(),
            game_data: recorder::GameData::new(),
            web_interface: WebInterface::new(),
            use_web: false,
        }
    }

    pub fn enable_web(&mut self) {
        self.use_web = true;
    }

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

    /// The main function for the Wordle game, implement your own logic here
    pub fn launch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // init
        self.load_game()?;

        self.game_loop()?;

        Ok(())
    }
}
