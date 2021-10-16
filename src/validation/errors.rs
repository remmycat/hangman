use clap::{Error as ClapError, ErrorKind as ClapErrorKind};
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum ValidationError {
	/// Occurs in random mode, when the passed parameters did not match any
	/// entries.
	NoWordsFound,

	/// Occurs in random mode, when the passed parameters did not match any
	/// entries other than the ones already played.
	NoMoreWordsFound,

	/// Occurs in random mode, when a passed min length is higher than max length
	MinLengthIsBiggerThanMaxLength { min_length: u8, max_length: u8 },

	/// Occurs in random mode, when a passed min score is higher than max score
	MinScoreIsBiggerThanMaxScore { min_score: u8, max_score: u8 },
}

impl Error for ValidationError {}

impl From<ValidationError> for ClapError {
	fn from(e: ValidationError) -> Self {
		match e {
			ValidationError::NoWordsFound => ClapError::with_description(
				"Could not find any words using the given parameters".to_string(),
				ClapErrorKind::ValueValidation,
			),
			ValidationError::NoMoreWordsFound => ClapError::with_description(
				"Could not find any more words using the given parameters".to_string(),
				ClapErrorKind::ValueValidation,
			),
			ValidationError::MinLengthIsBiggerThanMaxLength {
				min_length,
				max_length,
			} => ClapError::with_description(
				format!(
					"Min word length ({}) must be smaller than max length ({})",
					min_length, max_length
				),
				ClapErrorKind::ValueValidation,
			),
			ValidationError::MinScoreIsBiggerThanMaxScore {
				min_score,
				max_score,
			} => ClapError::with_description(
				format!(
					"Min word score ({}) must be smaller than max score ({})",
					min_score, max_score
				),
				ClapErrorKind::ValueValidation,
			),
		}
	}
}

impl Display for ValidationError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let as_clap: ClapError = self.to_owned().into();
		as_clap.fmt(f)
	}
}
