mod args;
mod builtin_words;
mod game;
mod recorder;
use wordle_lib::Wordle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wordle_cli = Wordle::new();
    wordle_cli.launch()?;

    Ok(())
}
