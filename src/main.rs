mod art;
mod errors;
mod game_state;
mod input;
mod render;
mod validation;
mod word_list;

use crossterm::{
	cursor,
	terminal::{self, ClearType},
};
use game_state::GameState;

use validation::{HangmanCliOptions, Validatable};

use clap::{Clap, Error as ClapError};

use std::{io::stdout, process::exit};

use render::render_game;

fn hangman_game(args: HangmanCliOptions) {
	let state = GameState::new(args);
	// TODO: term Errors
	render_game(state).unwrap();
}

/// cleans up changes to the terminal environment we might have made inbetween,
/// used to have an easier time handling ctrl-c or error paths.
fn clean_exit(code: i32) -> ! {
	crossterm::execute!(
		stdout(),
		cursor::DisableBlinking,
		terminal::LeaveAlternateScreen
	)
	.unwrap();
	if terminal::is_raw_mode_enabled().unwrap() {
		terminal::disable_raw_mode().unwrap();
	}
	exit(code)
}

fn terminate() -> ! {
	clean_exit(1)
}

fn reset_screen() -> crossterm::Result<()> {
	crossterm::execute!(
		stdout(),
		terminal::Clear(ClearType::FromCursorUp),
		cursor::MoveTo(0, 0)
	)?;
	Ok(())
}

fn main() -> ! {
	let parsed: Result<HangmanCliOptions, ClapError> =
		HangmanCliOptions::try_parse().and_then(|parsed| {
			parsed.validate()?;
			Ok(parsed)
		});

	match parsed {
		Err(error) => {
			eprintln!("{}", error);
			exit(1)
		}
		Ok(args) => {
			crossterm::execute!(stdout(), terminal::EnterAlternateScreen).unwrap();

			ctrlc::set_handler(|| {
				terminate();
			})
			.expect("Error setting Ctrl-C handler");

			println!("Welcome to Hangman!");
			hangman_game(args);
			clean_exit(0)
		}
	}
}
