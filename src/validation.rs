use clap::Clap;

use crate::errors::ValidationError;

pub trait Validatable {
	fn validate(&self) -> Result<(), ValidationError>;
}

#[derive(Clap, Debug)]
#[clap(name = "hangman", author = "remmycat")]
pub struct HangmanCliOptions {
	#[clap(subcommand)]
	pub mode: GameMode,
}
impl Validatable for HangmanCliOptions {
	fn validate(&self) -> Result<(), ValidationError> {
		let HangmanCliOptions { mode } = self;
		mode.validate()
	}
}

/// The different game modes of playing hangman
#[derive(Clap, Debug)]
pub enum GameMode {
	/// Random mode allows you to play with random entries from a huge list of
	/// words and phrases, that is usually used by crossword constructors. This
	/// might lead to you finding some crosswordese or unexpected words
	/// inbetween :)
	Random(RandomGame),
	/// Manual mode allows you to enter a word in secret, that another person
	/// can then guess.
	Manual(ManualGame),
}

impl Validatable for GameMode {
	fn validate(&self) -> Result<(), ValidationError> {
		match self {
			GameMode::Manual(manual) => manual.validate(),
			GameMode::Random(random) => random.validate(),
		}
	}
}

#[derive(Clap, Debug)]
pub struct RandomGame {
	/// Minimum number of letters a word should have
	#[clap(short = 'l', long, default_value = "3")]
	pub min_length: u8,

	/// Maximum number of letters a word should have
	#[clap(short = 'L', long, default_value = "50")]
	pub max_length: u8,

	/// Minimum "word coolness score" (0-100)
	#[clap(short = 's', long, default_value = "51")]
	pub min_score: u8,

	/// Minimum "word coolness score" (0-100)
	#[clap(short = 'S', long, default_value = "100")]
	pub max_score: u8,

	/// TODO make better
	#[clap(short = 'W', long, default_value = "6")]
	pub max_wrong_guesses: u8,
}

impl Validatable for RandomGame {
	fn validate(&self) -> Result<(), ValidationError> {
		match self {
			RandomGame {
				min_length,
				max_length,
				..
			} if min_length > max_length => Err(ValidationError::MinLengthIsBiggerThanMaxLength {
				min_length: *min_length,
				max_length: *max_length,
			}),
			RandomGame {
				min_score,
				max_score,
				..
			} if min_score > max_score => Err(ValidationError::MinScoreIsBiggerThanMaxScore {
				min_score: *min_score,
				max_score: *max_score,
			}),
			_ => Ok(()),
		}
	}
}

#[derive(Clap, Debug)]
pub struct ManualGame {
	/// TODO make better
	#[clap(short = 'W', long, default_value = "6")]
	pub max_wrong_guesses: u8,
}
impl Validatable for ManualGame {
	fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
