use crate::game_state::{EndFeedback, GameScene, GameState, GuessFeedback};
use crate::validation::{GameMode, ManualGame, RandomGame};

use ansi_term::Colour::{Cyan, Red};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use itertools::Itertools;
use std::collections::HashSet;
use std::process::exit;

fn format_guesses(word: &str, guessed: &HashSet<char>) -> String {
	let letters = ('A'..='Z').filter_map(|c| {
		let lower = c.clone().to_ascii_lowercase();
		let lower_word = word.to_ascii_lowercase();
		if guessed.contains(&lower) {
			if lower_word.contains(&lower.to_string()) {
				Some(Cyan.paint(c.to_string()).to_string())
			} else {
				Some(Red.paint(c.to_string()).to_string())
			}
		} else {
			None
		}
	});

	Itertools::intersperse(letters, " ".to_string()).collect()
}

fn format_word(word: &str, guessed: &HashSet<char>) -> String {
	let chars = word.chars().map(|char| {
		if !char.is_ascii_alphabetic() {
			return char;
		}
		let lower = char.clone().to_ascii_lowercase();
		if guessed.contains(&lower) {
			char
		} else {
			'_'
		}
	});

	let withspaces = Itertools::intersperse(chars, ' ');
	withspaces.collect()
}

fn get_guess() -> crossterm::Result<char> {
	loop {
		// `read()` blocks until an `Event` is available
		// TODO
		enable_raw_mode()?;
		let char = match read()? {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => {
				disable_raw_mode()?;
				exit(1)
			}
			Event::Key(KeyEvent {
				code: KeyCode::Char(c),
				..
			}) => Some(c),
			_ => None,
		};
		disable_raw_mode()?;

		match char {
			Some(char) if char.is_ascii_alphabetic() => return Ok(char.to_ascii_lowercase()),
			Some(_) => {
				println!("Please enter a letter (A-Z)")
			}
			None => (),
		}
	}
}

fn confirm_yn() -> crossterm::Result<bool> {
	loop {
		// `read()` blocks until an `Event` is available
		// TODO
		enable_raw_mode()?;
		let char = match read()? {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => {
				disable_raw_mode()?;
				exit(1)
			}
			Event::Key(KeyEvent {
				code: KeyCode::Enter,
				..
			}) => Some('y'),
			Event::Key(KeyEvent {
				code: KeyCode::Esc, ..
			}) => Some('n'),
			Event::Key(KeyEvent {
				code: KeyCode::Char(c),
				..
			}) => Some(c),
			_ => None,
		};
		disable_raw_mode()?;

		match char {
			Some('y') => return Ok(true),
			Some('n') => return Ok(false),
			Some(_) => {
				println!("Please enter 'y' for yes, or 'n' for no")
			}
			None => (),
		}
	}
}

pub fn render_game(state: GameState) -> crossterm::Result<()> {
	// let test_word = "\"Some phrase, I said, maybe?\"".to_string();
	// let mut guessed = HashSet::new();
	// guessed.insert('s');
	// guessed.insert('e');
	// guessed.insert('d');
	// guessed.insert('y');
	// guessed.insert('x');

	// println!(
	// 	"Phrase:  {}",
	// 	format_word(test_word.clone(), guessed.clone())
	// );
	// println!("\nGuesses: {}", format_guesses(test_word, guessed));

	match &state.scene {
		GameScene::GameEnd { feedback } => {
			match feedback {
				EndFeedback::NoWordsFound => {
					println!("Unfortunately there were no words matching your criteria :(");
				}
				EndFeedback::NoMoreWordsFound => {
					println!("Unfortunately we ran out of words matching your criteria! :(");
				}
				EndFeedback::ManuallyEnded => {
					println!("Goodbye then! <3");
				}
			}
			println!("Rounds played:  {}", state.rounds_played);
			println!("Final score:    {:.0}", state.score);
			Ok(())
		}
		GameScene::Init => {
			let is_first_game = state.rounds_played == 0;
			match &state.args.mode {
				GameMode::Manual(ManualGame { .. }) => {
					if is_first_game {
						println!("Manual mode");
					} else {
						println!("Time for another round!");
					}
					println!("Go enter your word below:");
					let word = rpassword::prompt_password_stdout(">> ").unwrap();
					render_game(state.start_manual_game(word))
				}
				GameMode::Random(RandomGame { .. }) => {
					let unplayed_words = &state.unplayed_words;
					if is_first_game {
						println!("Random mode");
						println!(
							"{} words matching your criteria were found",
							unplayed_words.len()
						);
					} else {
						println!("Time for another round!");
					}

					render_game(state.start_random_game())
				}
			}
		}
		GameScene::AwaitingGuess {
			letters_guessed,
			feedback,
			word,
		} => {
			match feedback {
				GuessFeedback::LetsGo => {
					println!();
					println!("Let's hang some men!");
					println!();
				}
				GuessFeedback::AlreadyTried(guess) => {
					println!("You already tried '{}'!", guess);
				}
				GuessFeedback::Correct(_) => {
					println!("Correct!");
				}
				GuessFeedback::Wrong(_) => {
					println!("Wrong!");
				}
				GuessFeedback::BadChar(_) => {
					println!("You entered something that wasn't a letter")
				}
			}
			println!();
			println!("Phrase:      {}", format_word(word, letters_guessed));
			println!();
			println!("Guesses:     {}", format_guesses(word, letters_guessed));
			println!();

			let guess = get_guess()?;

			println!("\n\n\n\n\n\n");
			render_game(state.make_guess(guess))
			// if confirm_yn()? {
			// 	render_game(state.confirm_word())
			// } else {
			// 	println!("Alright then, pick another word.");
			// 	render_game(state.get_new_word())
			// }
		}
		GameScene::RoundEnd {
			word,
			won,
			round_score,
			letters_guessed,
		} => {
			println!();
			println!("Phrase:      {}", format_word(word, letters_guessed));
			println!();
			println!("Guesses:     {}", format_guesses(word, letters_guessed));
			println!();
			println!("\n\n\n\n\n\n");
			if *won {
				println!("You won!");
				println!("Phrase is:     {}", word);
			} else {
				println!("You lost!");
				println!("Phrase was:    {}", word);
			}
			println!();
			println!("Round score:   {}", round_score);
			println!("Total score:   {}", state.score);
			println!();
			println!("Play another round? [y]es / [n]o");
			let yes = confirm_yn()?;

			if yes {
				render_game(state.new_round())
			} else {
				render_game(state.end_game(EndFeedback::ManuallyEnded))
			}
		}
	}
}
