use std::{
	collections::HashSet,
	io::{stdout, Write},
};

use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
	execute,
	terminal::{self, ClearType},
};

use crate::{render::format_word, terminate};

pub fn get_char(prompt: &'static str) -> crossterm::Result<Option<char>> {
	loop {
		terminal::enable_raw_mode()?;
		print!("{}", prompt);
		stdout().flush().unwrap();

		let key_event = match event::read()? {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => {
				terminate();
			}
			Event::Key(KeyEvent {
				code: KeyCode::Char(c),
				..
			}) => Some(Some(c)),
			Event::Key(KeyEvent { .. }) => Some(None),
			_ => None,
		};

		if let Some(Some(char)) = &key_event {
			print!("{}", char);
			stdout().flush().unwrap();
		}

		terminal::disable_raw_mode()?;
		println!();

		if let Some(key) = key_event {
			return Ok(key);
		}
	}
}

pub fn confirm_yn(prompt: &'static str) -> crossterm::Result<bool> {
	loop {
		terminal::enable_raw_mode()?;
		print!("{}", prompt);
		stdout().flush().unwrap();

		let key_event = match event::read()? {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => {
				terminate();
			}
			Event::Key(KeyEvent {
				code: KeyCode::Enter,
				..
			}) => Some(Some('y')),
			Event::Key(KeyEvent {
				code: KeyCode::Esc, ..
			}) => Some(Some('n')),
			Event::Key(KeyEvent {
				code: KeyCode::Char(c),
				..
			}) => Some(Some(c)),
			Event::Key(KeyEvent { .. }) => Some(None),
			_ => None,
		};
		terminal::disable_raw_mode()?;
		println!();

		match key_event {
			Some(Some('y')) => return Ok(true),
			Some(Some('n')) => return Ok(false),
			Some(_) => {
				println!("Please enter 'y' for yes, or 'n' for no")
			}
			None => (),
		}
	}
}

pub fn confirm_enter() -> crossterm::Result<()> {
	loop {
		terminal::enable_raw_mode()?;

		match event::read()? {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => {
				terminate();
			}
			Event::Key(KeyEvent {
				code: KeyCode::Enter,
				..
			}) => {
				terminal::disable_raw_mode()?;
				println!();
				return Ok(());
			}
			_ => {}
		};
	}
}

pub fn get_word(prompt: &'static str) -> crossterm::Result<String> {
	let mut word: String = String::new();
	let empty_map: HashSet<char> = HashSet::new();

	loop {
		terminal::enable_raw_mode()?;
		print!(
			"{}{}",
			prompt,
			format_word(word.as_str(), &empty_map, false)
		);
		stdout().flush().unwrap();

		match event::read()? {
			Event::Key(KeyEvent {
				code: KeyCode::Char('c'),
				modifiers: KeyModifiers::CONTROL,
			}) => {
				terminate();
			}
			Event::Key(KeyEvent {
				code: KeyCode::Char(c),
				..
			}) => {
				word = format!("{}{}", &word, c);
			}
			Event::Key(KeyEvent {
				code: KeyCode::Enter,
				..
			}) => {
				terminal::disable_raw_mode()?;
				return Ok(word);
			}
			Event::Key(KeyEvent {
				code: KeyCode::Backspace,
				..
			}) => {
				let mut chars = word.chars();
				chars.next_back();
				word = chars.as_str().to_string();
			}
			_ => (),
		};
		execute!(
			stdout(),
			terminal::Clear(ClearType::CurrentLine),
			cursor::MoveToColumn(0)
		)?;
		terminal::disable_raw_mode()?;
	}
}
