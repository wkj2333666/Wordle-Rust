use clap::Parser;
use std::io;
mod args;
use args::Args;

use crate::game::init_game;
mod builtin_words;
mod game;
mod recorder;

fn game_loop(
    is_tty: bool,
    args: &mut Args,
    game_recorder: &mut recorder::GameRecorder,
    game_data: &mut recorder::GameData,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut final_words = Vec::<String>::new();
    let mut acceptable = Vec::<String>::new();
    init_game(args, &mut final_words, &mut acceptable)?;
    loop {
        game::start_one_game(
            is_tty,
            args,
            game_recorder,
            &final_words,
            &acceptable,
            game_data,
        );
        // Show stats if requested
        if args.stats {
            game_recorder.print();
        }

        // Day++
        if args.day.is_some() {
            args.day = Some(args.day.unwrap() + 1);
        }

        // Save game data if requested
        if args.state.is_some() {
            game_data.save(&args)?;
        }

        // Do not play again if word is specified
        if args.word.is_some() {
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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init
    let is_tty = atty::is(atty::Stream::Stdout);
    let mut args = Args::parse();
    let mut game_recorder = recorder::GameRecorder::new();
    let mut game_data = recorder::GameData::new();

    game::load_game(&mut args, &mut game_recorder, &mut game_data)?;

    game_loop(is_tty, &mut args, &mut game_recorder, &mut game_data)?;

    Ok(())
}
