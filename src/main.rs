use clap::Parser;
use std::io;
mod args;
use args::Args;
mod builtin_words;
mod game;

fn game_loop(is_tty: bool, args: &Args) {
    loop {
        game::start_game(is_tty, args);
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
    let is_tty = atty::is(atty::Stream::Stdout);
    let args = Args::parse();
    if !args.is_validity() {
        return Err("Invalid arguments".into());
    }
    game_loop(is_tty, &args);

    Ok(())
}
