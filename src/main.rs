use rand::seq::SliceRandom;
use structopt::StructOpt;

use std::convert::TryInto;
use std::process::exit;

use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(StructOpt, Debug)]
#[structopt(name = "hangman", author = "remmycat")]
struct HangmanCliOptions {
    /// Minimum number of letters a word should have
    #[structopt(long, default_value = "6")]
    min: u8,

    /// Minimum number of letters a word should have
    #[structopt(long)]
    max: Option<u8>,
}

#[derive(Debug, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum HangmanError {
    MinIsZero,
    MinIsTooHigh,
    MinIsBiggerThanMax { min: u8, max: u8 },
}

impl Error for HangmanError {}

// The smallest number of letters supported as a "min" value.
const MIN_CUTOFF: u8 = 15;

impl Display for HangmanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use HangmanError::*;
        match self {
            MinIsZero => write!(f, "Provided min word length must be bigger than 0"),
            MinIsBiggerThanMax { min, max } => write!(
                f,
                "Min word length ({}) must be smaller than max length ({})",
                min, max
            ),
            MinIsTooHigh => write!(f, "Min word length must be smaller than {}, because the dataset does not have enough longer words", MIN_CUTOFF)
        }
    }
}

fn check_args(args: HangmanCliOptions) -> Result<HangmanCliOptions, HangmanError> {
    match args {
        HangmanCliOptions { min: 0, .. } => Err(HangmanError::MinIsZero),
        HangmanCliOptions { min, .. } if min >= MIN_CUTOFF => Err(HangmanError::MinIsTooHigh),
        HangmanCliOptions {
            min,
            max: Some(max),
        } if min > max => Err(HangmanError::MinIsBiggerThanMax { min, max }),
        _ => Ok(args),
    }
}

static WORD_LIST: &str = include_str!("../assets/frequent-words.txt");

fn get_filtered_word_list(min_len: u8, max_len: Option<u8>) -> Vec<String> {
    let filtered: Vec<&str> = WORD_LIST
        .lines()
        .filter(|line| {
            // todo better error handling.
            let word_len: Result<u32, _> = line
                .chars()
                .filter(|x| x.is_alphabetic())
                .count()
                .try_into();
            match (word_len, min_len, max_len) {
                (Ok(length), min, None) if length >= min.into() => true,
                (Ok(length), min, Some(max)) if length >= min.into() && length <= max.into() => {
                    true
                }
                _ => false,
            }
        })
        .collect();

    filtered
        .choose_multiple(&mut rand::thread_rng(), filtered.len())
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn hangman_game(args: HangmanCliOptions) -> Result<(), HangmanError> {
    let HangmanCliOptions { min, max } = check_args(args)?;

    println!(
        "Welcome to Hangman! Source of word list: Free Sample by https://www.wordfrequency.info/"
    );

    let words = get_filtered_word_list(min, max);
    println!("Words: {:#?}", words);
    Ok(())
}

fn main() {
    let args = HangmanCliOptions::from_args();

    exit(match hangman_game(args) {
        Err(error) => {
            eprintln!("Error running Hangman: {}", error);
            1
        }
        Ok(_) => 0,
    });
}
