use rand::seq::SliceRandom;

use std::convert::TryInto;

static GIANT_CROSSWORD_WORD_LIST: &str = include_str!("../../assets/crossword-phrases.txt");

pub fn get_filtered_word_list(
	min_length: u8,
	max_length: u8,
	min_score: u8,
	max_score: u8,
) -> Vec<&'static str> {
	let filtered: Vec<&str> = GIANT_CROSSWORD_WORD_LIST
		.lines()
		.filter_map(|line| {
			// 1. Parse line of form phrase::score into those values

			let split: Vec<&str> = line.split("::").collect();
			match split[..] {
				[word, score_str] => Some((word, score_str)),
				// ignore errored lines for now
				_ => None,
			}
		})
		.filter_map(|(word, score_str)| {
			// 2. Parse score into int and filter non-matching

			let try_score_int = score_str.parse::<u8>();
			match try_score_int {
				Ok(score) if score >= min_score && score <= max_score => Some(word),
				_ => None,
			}
		})
		.filter(|word| {
			// 4. Parse length of alphabetical chars

			let try_word_len: Result<u8, _> = word
				.chars()
				.filter(|x| x.is_alphabetic())
				.count()
				.try_into();

			match try_word_len {
				Ok(word_len) => word_len >= min_length && word_len <= max_length,
				_ => false,
			}
		})
		.collect();

	filtered
		.choose_multiple(&mut rand::thread_rng(), filtered.len())
		.cloned()
		.collect()
}
