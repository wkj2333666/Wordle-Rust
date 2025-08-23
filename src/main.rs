use clap::Parser;
use std::io;
mod args;
use args::Args;
mod builtin_words;
mod game;
mod recorder;

fn game_loop(is_tty: bool, args: &Args, game_recorder: &mut recorder::GameRecorder) {
    loop {
        game::start_game(is_tty, args, game_recorder);
        // Show stats if requested
        if args.stats {
            game_recorder.print();
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
}

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // init
    let is_tty = atty::is(atty::Stream::Stdout);
    let args = Args::parse();
    let mut game_recorder = recorder::GameRecorder::new();

    if !args.is_validity() {
        return Err("Invalid arguments".into());
    }
    game_loop(is_tty, &args, &mut game_recorder);

    Ok(())
}
