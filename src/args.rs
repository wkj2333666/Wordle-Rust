use clap::Parser;
use serde::{Deserialize, Serialize};

/// A simple wordle game
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Specify the answer, conflicts with -r
    #[arg(
        short,
        long,
        conflicts_with = "random",
        conflicts_with = "day",
        conflicts_with = "seed"
    )]
    pub word: Option<String>,

    /// Use random answer, conflicts with -w
    #[arg(short, long)]
    pub random: bool,

    /// Use difficult mode
    #[arg(short = 'D', long)]
    pub difficult: bool,

    /// Record statistical data of the game
    #[arg(short = 't', long)]
    pub stats: bool,

    /// Set the day of the game
    #[arg(short, long, default_value = "1", requires = "random")]
    pub day: Option<usize>,

    /// Set the seed of the game
    #[arg(short, long, default_value = "114514", requires = "random")]
    pub seed: Option<u64>,

    /// Specify the final words list
    #[arg(short, long)]
    pub final_set: Option<String>,

    /// Specify the acceptable words list
    #[arg(short, long)]
    pub acceptable_set: Option<String>,

    /// Persistent Storage
    #[arg(short = 'S', long)]
    pub state: Option<String>,

    /// Set configuration file
    #[arg(short, long)]
    pub config: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub word: Option<String>,

    #[serde(default)]
    pub random: bool,

    #[serde(default)]
    pub difficult: bool,

    #[serde(default)]
    pub stats: bool,

    pub day: Option<usize>,
    pub seed: Option<u64>,
    pub final_set: Option<String>,
    pub acceptable_set: Option<String>,
    pub state: Option<String>,
}
