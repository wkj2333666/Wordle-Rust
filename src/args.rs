use clap::Parser;

/// A simple wordle game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Specify the answer, conflicts with -r
    #[arg(short, long)]
    pub word: Option<String>,

    /// Use random answer, conflicts with -w
    #[arg(short, long)]
    pub random: bool,

    /// Use difficult mode
    #[arg(short = 'D', long)]
    pub difficult: bool,
}

impl Args {
    pub fn is_validity(&self) -> bool {
        if self.word.is_some() && self.random {
            eprintln!("Cannot specify both -w and -r");
            return false;
        }
        true
    }
}
