mod errors;
mod game_state;
mod validation;
mod word_list;

use game_state::GameState;

use validation::{HangmanCliOptions, Validatable};

use clap::{Clap, Error as ClapError};

use std::process::exit;

use crate::game_state::render::render_game;

fn hangman_game(args: HangmanCliOptions) {
	let state = GameState::new(args);
	// TODO: term Errors
	render_game(state).unwrap();
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
