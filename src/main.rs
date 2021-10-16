mod validation;
mod word_list;

use word_list::get_filtered_word_list;

use validation::{GameMode, HangmanCliOptions, ManualGame, RandomGame, Validatable};

use clap::{Clap, Error as ClapError};

use std::process::exit;

fn hangman_game(args: HangmanCliOptions) {
	match args.mode {
		GameMode::Manual(ManualGame { word }) => {
			println!("Mode: [Manual] - \"{}\"", word);
		}
		GameMode::Random(RandomGame {
			min_length,
			max_length,
			min_score,
			max_score,
		}) => {
			println!("Mode: [Random]");
			let mut words = get_filtered_word_list(min_length, max_length, min_score, max_score);
			println!("Found {} words", words.len());
			words.truncate(10);
			println!("Words sample: {:#?}", words);
		}
	}
}

fn main() {
	let parsed: Result<HangmanCliOptions, ClapError> =
		HangmanCliOptions::try_parse().and_then(|parsed| {
			parsed.validate()?;
			Ok(parsed)
		});

	exit(match parsed {
		Err(error) => {
			eprintln!("{}", error);
			1
		}
		Ok(args) => {
			println!("Welcome to Hangman!");
			hangman_game(args);
			0
		}
	});
}
