//!
//!
//!

use std::path::Path;

pub fn load<P: AsRef<Path>>(game: &mut crate::game::Game, from: P) -> std::io::Result<()> {
	let history = std::fs::read_to_string(from)?;

	game.apply_history(&history).map_err(|c| {
		std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("Unknown character: {c:?}."),
		)
	})?;
	Ok(())
}

pub fn save<P: AsRef<Path>>(game: &crate::game::Game, to: P) -> std::io::Result<bool> {
	if !game.get_history().is_empty() {
		std::fs::write(to, game.get_history()).map(|_| true)
	} else {
		Ok(false)
	}
}
