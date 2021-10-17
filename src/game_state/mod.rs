pub mod render;

use crate::{
	validation::{GameMode, HangmanCliOptions, ManualGame, RandomGame},
	word_list::get_filtered_word_list,
};
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fmt::{self, Debug, Formatter};

#[derive(Debug)]
pub enum GuessFeedback {
	LetsGo,
	Correct(char),
	Wrong(char),
	AlreadyTried(char),
	BadChar(char),
}

#[derive(Debug)]
pub enum EndFeedback {
	NoWordsFound,
	NoMoreWordsFound,
	ManuallyEnded,
}

#[derive(Debug)]
pub enum GameScene {
	Init,
	AwaitingGuess {
		word: String,
		letters_guessed: HashSet<char>,
		feedback: GuessFeedback,
	},
	RoundEnd {
		word: String,
		won: bool,
		round_score: f64,
		letters_guessed: HashSet<char>,
	},
	GameEnd {
		feedback: EndFeedback,
	},
}

/// helper for better debugging
#[derive(Debug)]
pub struct HugeWordsVec<'a> {
	length: &'a usize,
	next_ten: Vec<&'static str>,
}

pub struct GameState {
	pub args: HangmanCliOptions,
	pub unplayed_words: Vec<&'static str>,
	pub played_words: Vec<String>,
	pub score: f64,
	pub scene: GameScene,
	pub rounds_played: u32,
}
impl Debug for GameState {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let some_unplayed_words = &mut self.unplayed_words.to_owned();
		some_unplayed_words.reverse();
		some_unplayed_words.truncate(10);
		let unplayed_words_debug = &HugeWordsVec {
			length: &self.unplayed_words.len(),
			next_ten: some_unplayed_words.to_vec(),
		};

		f.debug_struct("GameState")
			.field("scene", &self.scene)
			.field("args", &self.args)
			.field("score", &self.score)
			.field("rounds_played", &self.rounds_played)
			.field("played_words", &self.played_words)
			.field("unplayed_words", &unplayed_words_debug)
			.finish()
	}
}

static GAME_WON_BASE_SCORE: f64 = 10.0;
static GAME_WON_LEFT_GUESS_MULTIPLIER: f64 = 1.75;

impl GameState {
	pub fn new(args: HangmanCliOptions) -> GameState {
		match args.mode {
			GameMode::Manual(ManualGame { .. }) => GameState {
				args,
				unplayed_words: Vec::new(),
				played_words: Vec::new(),
				score: 0.0,
				rounds_played: 0,
				scene: GameScene::Init,
			},
			GameMode::Random(RandomGame {
				min_length,
				max_length,
				min_score,
				max_score,
				..
			}) => {
				let unplayed_words =
					get_filtered_word_list(min_length, max_length, min_score, max_score);
				if !unplayed_words.is_empty() {
					GameState {
						args,
						unplayed_words,
						played_words: Vec::new(),
						score: 0.0,
						rounds_played: 0,
						scene: GameScene::Init,
					}
				} else {
					GameState {
						args,
						unplayed_words,
						played_words: Vec::new(),
						score: 0.0,
						rounds_played: 0,
						scene: GameScene::GameEnd {
							feedback: EndFeedback::NoWordsFound,
						},
					}
				}
			}
		}
	}

	pub fn start_manual_game(self, word: String) -> GameState {
		match self.scene {
			GameScene::Init => GameState {
				scene: GameScene::AwaitingGuess {
					word,
					letters_guessed: HashSet::new(),
					feedback: GuessFeedback::LetsGo,
				},
				rounds_played: self.rounds_played + 1,
				..self
			},
			scene => panic!(
				"Invalid scene transition start_manual_game executed on scene {:#?}",
				scene
			),
		}
	}

	pub fn start_random_game(self) -> GameState {
		match self.scene {
			GameScene::Init => {
				let mut unplayed_words = self.unplayed_words;
				let first_word = unplayed_words.pop();
				match first_word {
					Some(word) => GameState {
						scene: GameScene::AwaitingGuess {
							word: word.to_string(),
							letters_guessed: HashSet::new(),
							feedback: GuessFeedback::LetsGo,
						},
						rounds_played: self.rounds_played + 1,
						unplayed_words,
						..self
					},
					None => GameState {
						scene: GameScene::GameEnd {
							feedback: EndFeedback::NoMoreWordsFound,
						},
						unplayed_words,
						..self
					},
				}
			}
			scene => panic!(
				"Invalid scene transition start_random_game executed on scene {:#?}",
				scene
			),
		}
	}

	pub fn make_guess(self, guess: char) -> GameState {
		match self.scene {
			GameScene::AwaitingGuess {
				word,
				mut letters_guessed,
				..
			} => {
				let guess = guess.to_ascii_lowercase();
				let word_letters: HashSet<char> = word
					.chars()
					.filter_map(|c| {
						if c.is_ascii_alphabetic() {
							Some(c.to_ascii_lowercase())
						} else {
							None
						}
					})
					.collect();

				let guess_in_guessed = letters_guessed.get(&guess).is_some();
				let guess_in_word = word_letters.get(&guess).is_some();
				let wrong_guesses =
					letters_guessed.len() - letters_guessed.intersection(&word_letters).count();

				let max_wrong_guesses = match self.args.mode {
					GameMode::Manual(ManualGame {
						max_wrong_guesses, ..
					}) => max_wrong_guesses,
					GameMode::Random(RandomGame {
						max_wrong_guesses, ..
					}) => max_wrong_guesses,
				};

				if !guess.is_ascii_alphabetic() {
					// Guess was letter other than A-z
					GameState {
						scene: GameScene::AwaitingGuess {
							word,
							letters_guessed,
							feedback: GuessFeedback::BadChar(guess),
						},
						..self
					}
				} else if guess_in_guessed {
					// This letter was already guessed!
					GameState {
						scene: GameScene::AwaitingGuess {
							word,
							letters_guessed,
							feedback: GuessFeedback::AlreadyTried(guess),
						},
						..self
					}
				} else if guess_in_word {
					// This letter is correct!
					letters_guessed.insert(guess);

					if letters_guessed.intersection(&word_letters).count() == word_letters.len() {
						// Won!
						let left_guesses =
							i32::from(max_wrong_guesses) - i32::try_from(wrong_guesses).unwrap();
						let left_guesses_multiplier: f64 =
							GAME_WON_LEFT_GUESS_MULTIPLIER.powi(left_guesses);
						let round_score: f64 =
							(GAME_WON_BASE_SCORE * left_guesses_multiplier).round();
						GameState {
							scene: GameScene::RoundEnd {
								won: true,
								word,
								round_score,
								letters_guessed,
							},
							score: self.score + round_score,
							..self
						}
					} else {
						GameState {
							scene: GameScene::AwaitingGuess {
								word,
								letters_guessed,
								feedback: GuessFeedback::Correct(guess),
							},
							..self
						}
					}
				} else {
					// Wrong guess!
					letters_guessed.insert(guess);

					// Assumption: u8 can always fit into usize
					if wrong_guesses + 1 > max_wrong_guesses.into() {
						let mut played_words = self.played_words;
						let word_copy = word.clone();
						played_words.push(word);
						// Lost!
						GameState {
							scene: GameScene::RoundEnd {
								won: false,
								word: word_copy,
								round_score: 0.0,
								letters_guessed,
							},
							played_words,
							..self
						}
					} else {
						GameState {
							scene: GameScene::AwaitingGuess {
								word,
								letters_guessed,
								feedback: GuessFeedback::Wrong(guess),
							},
							..self
						}
					}
				}
			}
			scene => panic!(
				"Invalid scene transition make_guess executed on scene {:#?}",
				scene
			),
		}
	}

	pub fn new_round(self) -> GameState {
		GameState {
			scene: GameScene::Init,
			..self
		}
	}

	pub fn end_game(self, feedback: EndFeedback) -> GameState {
		GameState {
			scene: GameScene::GameEnd { feedback },
			..self
		}
	}
}
