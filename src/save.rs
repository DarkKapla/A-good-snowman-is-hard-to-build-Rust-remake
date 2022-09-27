//!
//!
//!

use std::path::Path;

pub fn load<P: AsRef<Path>>(game: &mut crate::game::Game, from: P) -> std::io::Result<()> {
	let mut history = std::fs::read_to_string(from)?;
	if history.len() == 0 {
		return Ok(());
	}
	if history.chars().last() == Some('\n') {
		history.pop();
	}
	game.apply_history(&history).map_err(|c| {
		std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("Unknown character: {c:?}."),
		)
	})?;
	Ok(())
}

pub fn save<P: AsRef<Path>>(game: &crate::game::Game, to: P) -> std::io::Result<()> {
	if game.get_history().len() != 0 {
		std::fs::write(to, game.get_history())
	} else {
		Ok(())
	}
}
