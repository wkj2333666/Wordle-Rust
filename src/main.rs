mod args;
mod builtin_words;
mod game;
mod recorder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wordle_cli = wordle::Wordle::new();
    wordle_cli.launch()?;

    Ok(())
}
