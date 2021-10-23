use crate::game_state::{EndFeedback, GameScene, GameState, GuessFeedback};
use crate::input::{confirm_enter, confirm_yn, get_char, get_word};
use crate::reset_screen;
use crate::validation::{GameMode, ManualGame, RandomGame};

use crossterm::style::Stylize;
use itertools::Itertools;
use std::collections::HashSet;
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

fn format_guesses(word: &str, guessed: &HashSet<char>) -> String {
	let letters = ('A'..='Z').filter_map(|c| {
		let lower = c.clone().to_ascii_lowercase();
		let lower_word = word.to_ascii_lowercase();

		if guessed.contains(&lower) {
			if lower_word.contains(&lower.to_string()) {
				Some(c.to_string().dark_cyan().to_string())
			} else {
				Some(c.to_string().dim().to_string())
			}
		} else {
			None
		}
	});

	letters.collect()
}

pub fn format_word(word: &str, guessed: &HashSet<char>, insert_spaces: bool) -> String {
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

	if !insert_spaces {
		chars.collect()
	} else {
		let withspaces = Itertools::intersperse(chars, ' ');
		withspaces.collect()
	}
}

const GUESS_PROMPT: &str = "> ";

fn print_last_guess(last_guess: &char) {
	println!("{}{}", GUESS_PROMPT, last_guess);
}

fn format_word_and_guesses(word: &str, guessed: &HashSet<char>) {
	println!();
	println!("Phrase:      {}", format_word(word, guessed, true));
	println!();
	println!("Guesses:     {}", format_guesses(word, guessed));
	println!();
}

pub fn render_game(state: GameState) -> crossterm::Result<()> {
	match &state.scene {
		GameScene::GameEnd { feedback } => {
			reset_screen()?;
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
			reset_screen()?;
			let is_first_game = state.rounds_played == 0;

			match &state.args.mode {
				GameMode::Manual(ManualGame { .. }) => {
					if is_first_game {
						println!("{}", "Manual mode".bold());
					} else {
						println!("Time for another round!");
					}
					println!();
					println!("Enter your word or phrase:");
					let word = get_word("> ")?;
					render_game(state.start_manual_game(word))
				}
				GameMode::Random(RandomGame { .. }) => {
					let unplayed_words = &state.unplayed_words;
					if is_first_game {
						println!("{}", "Random mode".bold());
						println!(
							"{} words and phrases matching your criteria were found",
							unplayed_words.len()
						);
					} else {
						println!("Time for another round!");
					}
					println!();
					println!("Press enter to start");
					confirm_enter()?;

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
					reset_screen()?;
					println!("Let's hang some men!");
					format_word_and_guesses(word, letters_guessed);
				}
				GuessFeedback::Correct(guess) => {
					reset_screen()?;
					print_last_guess(guess);
					println!("{}", "Correct".dark_green().bold());
					format_word_and_guesses(word, letters_guessed);
				}
				GuessFeedback::Wrong(guess) => {
					reset_screen()?;
					print_last_guess(guess);
					println!("{}", "Wrong!".dark_red().bold());
					format_word_and_guesses(word, letters_guessed);
				}
				GuessFeedback::AlreadyTried(guess) => {
					println!("You already tried '{}'!", guess);
				}
				GuessFeedback::BadChar(_) => {
					println!("Please enter a letter (A - Z)")
				}
			}

			let guess = get_char(GUESS_PROMPT)?;
			render_game(state.input_guess(guess))
		}
		GameScene::ValidGuess {
			guess,
			word,
			letters_guessed,
		} => {
			for n in 0..=3 {
				reset_screen()?;
				print_last_guess(guess);
				print!("Trying {}", guess);
				print!("{}", ".".repeat(n));
				stdout().flush().unwrap();
				println!();
				format_word_and_guesses(word, letters_guessed);
				thread::sleep(Duration::from_millis(100));
			}

			render_game(state.make_guess())
		}
		GameScene::RoundEnd {
			word,
			won,
			round_score,
			letters_guessed,
		} => {
			reset_screen()?;
			format_word_and_guesses(word, letters_guessed);
			println!();
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
			let yes = confirm_yn("> ")?;

			if yes {
				render_game(state.new_round())
			} else {
				render_game(state.end_game(EndFeedback::ManuallyEnded))
			}
		}
	}
}
