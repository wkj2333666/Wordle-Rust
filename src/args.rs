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

    /// record statistical data of the game
    #[arg(short = 't', long)]
    pub stats: bool,

    /// Set the day of the game
    #[arg(short, long, default_value = "1")]
    pub day: Option<u32>,

    /// Set the seed of the game
    #[arg(short, long, default_value = "114514")]
    pub seed: Option<u64>,
}

impl Args {
    pub fn is_validity(&self) -> bool {
        if self.word.is_some() && self.random {
            eprintln!("Cannot specify both -w and -r");
            return false;
        }

        if !self.random && self.seed.is_some() {
            eprintln!("Cannot specify -s without -r");
            return false;
        }

        if !self.random && self.day.is_some() {
            eprintln!("Cannot specify -d without -r");
            return false;
        }

        true
    }
}
