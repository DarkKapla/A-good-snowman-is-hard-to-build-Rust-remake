//!
//! Reset a unique level.
//!

use std::collections::HashSet;

use super::*;

impl Game {
	pub fn current_level_diff(&self) -> Vec<super::OneTileUpdate> {
		LevelDfsExplorer::from_point(self.player.0, self.player.1)
			.filter_map(|(x, y)| try_generate_update_at(self, x, y))
			.collect()
	}
}

/// Iterate over all the tile coords `(x, y)` of the game's
/// current level. Is empty if the player is not in a level.
struct LevelDfsExplorer {
	to_explore: Vec<(usize, usize)>,
	visited: HashSet<(usize, usize)>,
}
// It's more a flood algo than a DFS. FLOOD is more accurate.

impl LevelDfsExplorer {
	fn from_point(root_x: usize, root_y: usize) -> LevelDfsExplorer {
		let mut to_explore = Vec::<(usize, usize)>::with_capacity(32);
		let mut visited = HashSet::<(usize, usize)>::with_capacity(32);

		if is_level_char(root_x, root_y) {
			to_explore.push((root_x, root_y));
			visited.insert((root_x, root_y));
		} // If that `if` is not executed, the iterator won't output anything.

		LevelDfsExplorer {
			to_explore,
			visited,
		}
	}
}

impl Iterator for LevelDfsExplorer {
	type Item = (usize, usize);
	// The DFS algorithm is probably less readable with this iterator paradigm.
	fn next(&mut self) -> Option<Self::Item> {
		if let Some((x, y)) = self.to_explore.pop() {
			for (nx, ny) in valid_neighbours(x, y) {
				if self.visited.insert((nx, ny)) {
					self.to_explore.push((nx, ny));
				}
			}
			return Some((x, y));
		} else {
			return None;
		}
	}
}

impl std::iter::FusedIterator for LevelDfsExplorer {}

fn get_char(x: usize, y: usize) -> char {
	let index = x * (SIZE_Y + 1) + y; // take the '\n' characters into account
	return (*MAP.as_bytes().get(index).unwrap()).into();
}

fn is_level_char(x: usize, y: usize) -> bool {
	let c = get_char(x, y);
	matches!(Tile::from_char(c), Tile::Dirt | Tile::Snow)
}

fn valid_neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
	[
		Direction::Up,
		Direction::Down,
		Direction::Left,
		Direction::Right,
	]
	.into_iter()
	.filter_map(move |d| try_step(x, y, d))
	.filter(|(xx, yy)| is_level_char(*xx, *yy))
}

fn try_generate_update_at(game: &Game, x: usize, y: usize) -> Option<OneTileUpdate> {
	let c = get_char(x, y);
	let tile = Tile::from_char(c);
	let snowball = SnowBall::from_char(c);

	let new_tile = (game.tiles[x][y] != tile).then_some(tile);
	let new_snowball = (game.snowballs[x][y] != snowball).then_some(snowball);

	if new_tile.is_none() && new_snowball.is_none() {
		return None;
	} else {
		return Some(OneTileUpdate {
			x,
			y,
			new_tile,
			new_snowball,
		});
	}
}

/*
 * Below is the first-attempt function of this module. It does not only explore the
 * graph to extract a connexe level, but also generates the Update units. That's two
 * different actions that can be separated. The current active implementation, relying
 * on an iterator, does the separation "explore tiles" / "turn tiles into Updates"
 * without creating a temporary vector of all the level's tiles. I keep the old function
 * anyway.
 *
 * According to valgrind with callgrind, with profile opt-level="s", lto=true:
 * The fastest function is the while-loop one. It costs about 1,550 "Ir" less than the
 * iterator function, making it 5.56% faster.
 */

/* impl Game {
pub fn current_level_diff(&self) -> Vec<super::OneTileUpdate> {
	let (root_x, root_y) = self.player;
	if !is_level_char(root_x, root_y) {
		return Vec::new();
	}
	// Simple graph search. This will be an unoptimized depth-first search.
	let mut to_explore = Vec::<(usize, usize)>::with_capacity(32);
	let mut visited = HashSet::<(usize, usize)>::with_capacity(32);
	let mut updates = Vec::with_capacity(32);

	to_explore.push((root_x, root_y));
	visited.insert((root_x, root_y));

	while let Some((x, y)) = to_explore.pop() {
		// process the current position
		if let Some(u) = try_generate_update_at(self, x, y) {
			updates.push(u);
		}

		for (nx, ny) in valid_neighbours(x, y) {
			if visited.insert((nx, ny)) {
				to_explore.push((nx, ny));
			}
		}
	}
	return updates;
}
*/
