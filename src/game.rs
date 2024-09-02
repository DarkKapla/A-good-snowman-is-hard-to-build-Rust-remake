//!
//!
//!

mod reset;

use std::collections::VecDeque;

pub const SIZE_X: usize = str_to_usize(env!("SNOWCRAB_SIZE_X"));
pub const SIZE_Y: usize = str_to_usize(env!("SNOWCRAB_SIZE_Y"));

/*
   +-----> y
   |
   v  x
*/
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnowBall {
	Small,
	Medium,
	Large,
	SmallOnLarge,
	SmallOnMedium,
	MediumOnLarge,
	Snowman,
}

impl SnowBall {
	/// Return a grown version of the snowball which must be a single small, medium or large ball.
	/// Return `None` if not applicable.
	pub fn grow(self) -> Option<Self> {
		match self {
			SnowBall::Small => Some(SnowBall::Medium),
			SnowBall::Medium | SnowBall::Large => Some(SnowBall::Large),
			_ => Option::<SnowBall>::None,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
	Empty,
	Dirt,
	Snow,

	Hedge,
	Tree,
	Obstacle,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
	Up,
	Right,
	Down,
	Left,
}

#[derive(Clone, Debug)]
pub struct Game {
	pub tiles: [[Tile; SIZE_Y]; SIZE_X],
	pub snowballs: [[Option<SnowBall>; SIZE_Y]; SIZE_X],
	pub player: (usize, usize),
	rewind_queue: VecDeque<Update>,
	input_history: String,
}

impl Game {
	pub fn process_player_input(&mut self, dir: Direction) -> bool {
		self.process_player_input_no_history(dir)
			.then(|| self.input_history.push(dir.into()))
			.is_some()
	}

	fn process_player_input_no_history(&mut self, dir: Direction) -> bool {
		if let Some(map_diff) = self.step(dir) {
			// We ain't gonna let the stack grow to out of memory and beyond.
			if self.rewind_queue.len() >= 2048 {
				self.rewind_queue.pop_front();
			}

			self.apply_update(&map_diff.new);
			self.rewind_queue.push_back(map_diff.old);
			return true;
		} else {
			return false;
		}
	}

	pub fn rewind(&mut self) -> bool {
		if let Some(update) = self.rewind_queue.pop_back() {
			self.apply_update(&update);
			self.input_history.pop();
			return true;
		} else {
			println!("Cannot rewind any further.");
			return false;
		}
	}

	pub fn reset_current_level(&mut self) -> bool {
		self.reset_current_level_no_history()
			.then(|| self.input_history.push('T'))
			.is_some()
	}

	fn reset_current_level_no_history(&mut self) -> bool {
		// TODO: How to reset the player's position?
		let changes = self.current_level_diff();
		if changes.is_empty() {
			println!("Cannot reset the level.");
			return false;
		}
		for update in changes.into_iter() {
			self.apply_unit_update(&update);
		}
		self.rewind_queue.clear();
		return true;
	}

	fn apply_update(&mut self, update: &Update) {
		self.player = update.player;

		if let Some((ref tile0, ref tile1)) = update.tiles {
			self.apply_unit_update(tile0);
			self.apply_unit_update(tile1);
		}
	}

	fn apply_unit_update(&mut self, u: &OneTileUpdate) {
		if let Some(tile) = u.new_tile {
			self.tiles[u.x][u.y] = tile;
		}
		if let Some(snowball_opt) = u.new_snowball {
			self.snowballs[u.x][u.y] = snowball_opt;
		}
	}

	// The logic of the game goes here, basically.
	fn step(&self, dir: Direction) -> Option<MapDiff> {
		let (x0, y0) = self.player;
		let target = try_step(x0, y0, dir);
		if target.is_none() {
			println!("Cannot move outside the map.");
			return None;
		}
		let (x, y) = target.unwrap();

		match (self.tiles[x][y].blocks(), self.snowballs[x][y]) {
			(true, _) | (false, Some(SnowBall::Snowman)) => {
				return None;
			}
			(false, None) => {
				return Some(self.player_moves(x, y));
			}
			(false, Some(_)) => {
				return self.try_push_snowball(dir, x, y);
			}
		}
	}

	fn try_push_snowball(
		&self,
		dir: Direction,
		target_x: usize,
		target_y: usize,
	) -> Option<MapDiff> {
		if let Some((beyond_x, beyond_y)) = try_step(target_x, target_y, dir) {
			if self.tiles[beyond_x][beyond_y].blocks()
				|| self.snowballs[beyond_x][beyond_y] == Some(SnowBall::Snowman)
				|| self.tiles[beyond_x][beyond_y] == Tile::Empty
			{
				return None;
			}

			let target_snowball = self.snowballs[target_x][target_y]
				.expect("The tile to move to must be occupied by a snow ball.");
			if let Some(beyond_snowball) = self.snowballs[beyond_x][beyond_y] {
				let result = match (target_snowball, beyond_snowball) {
					(SnowBall::Small, SnowBall::Medium) => Some(SnowBall::SmallOnMedium),
					(SnowBall::Small, SnowBall::Large) => Some(SnowBall::SmallOnLarge),
					(SnowBall::Medium, SnowBall::Large) => Some(SnowBall::MediumOnLarge),
					(SnowBall::Small, SnowBall::MediumOnLarge) => Some(SnowBall::Snowman),
					_ => None,
				};
				match result {
					Some(mounted_snowball) => {
						return Some(self.player_mounts_snowballs(
							target_x,
							target_y,
							beyond_x,
							beyond_y,
							mounted_snowball,
						))
					}
					None => return None,
				};
			} else {
				let beyond_is_snow = self.tiles[beyond_x][beyond_y] == Tile::Snow;

				match target_snowball {
					SnowBall::Small | SnowBall::Medium | SnowBall::Large => {
						let new_snowball = if beyond_is_snow {
							target_snowball.grow().expect("unreachable")
						} else {
							target_snowball
						};

						return Some(self.player_pushes_snowball(
							target_x,
							target_y,
							beyond_x,
							beyond_y,
							new_snowball,
							beyond_is_snow,
						));
					}

					SnowBall::SmallOnMedium | SnowBall::SmallOnLarge | SnowBall::MediumOnLarge => {
						let (still_snowball, mut pushed_snowball) = match target_snowball {
							SnowBall::SmallOnMedium => (SnowBall::Medium, SnowBall::Small),
							SnowBall::SmallOnLarge => (SnowBall::Large, SnowBall::Small),
							SnowBall::MediumOnLarge => (SnowBall::Large, SnowBall::Medium),
							_ => unreachable!(),
						};

						if beyond_is_snow {
							// The pushed snowball grows if it lands on snow.
							pushed_snowball = pushed_snowball.grow().expect("unreachable");
						}

						return Some(self.snowball_descends(
							target_x,
							target_y,
							still_snowball,
							beyond_x,
							beyond_y,
							pushed_snowball,
							beyond_is_snow,
						));
					}

					SnowBall::Snowman => unreachable!(),
				}
			}
		} else {
			return None;
		}
	}

	fn player_moves(&self, new_x: usize, new_y: usize) -> MapDiff {
		MapDiff {
			new: Update {
				player: (new_x, new_y),
				tiles: None,
			},
			old: Update {
				player: (self.player.0, self.player.1),
				tiles: None,
			},
		}
	}

	fn player_pushes_snowball(
		&self,
		new_player_x: usize,
		new_player_y: usize,
		new_snowball_x: usize,
		new_snowball_y: usize,
		snowball_type: SnowBall,
		remove_snow: bool,
	) -> MapDiff {
		let tile_0_next = OneTileUpdate {
			x: new_player_x,
			y: new_player_y,
			new_tile: None,
			new_snowball: Some(None),
		};
		let tile_0_prev = OneTileUpdate {
			x: new_player_x,
			y: new_player_y,
			new_tile: None,
			new_snowball: Some(self.snowballs[new_player_x][new_player_y]),
		};
		let tile_1_next = OneTileUpdate {
			x: new_snowball_x,
			y: new_snowball_y,
			new_tile: if remove_snow { Some(Tile::Dirt) } else { None },
			new_snowball: Some(Some(snowball_type)),
		};
		let tile_1_prev = OneTileUpdate {
			x: new_snowball_x,
			y: new_snowball_y,
			new_tile: if remove_snow { Some(Tile::Snow) } else { None },
			new_snowball: Some(self.snowballs[new_snowball_x][new_snowball_y]),
		};

		return MapDiff {
			new: Update {
				player: (new_player_x, new_player_y),
				tiles: Some((tile_0_next, tile_1_next)),
			},
			old: Update {
				player: (self.player.0, self.player.1),
				tiles: Some((tile_0_prev, tile_1_prev)),
			},
		};
	}

	fn snowball_descends(
		&self,
		still_snowball_x: usize,
		still_snowball_y: usize,
		still_snowball_type: SnowBall,
		moved_snowball_x: usize,
		moved_snowball_y: usize,
		moved_snowball_type: SnowBall,
		remove_snow: bool,
	) -> MapDiff {
		let tile_0_next = OneTileUpdate {
			x: still_snowball_x,
			y: still_snowball_y,
			new_tile: None,
			new_snowball: Some(Some(still_snowball_type)),
		};
		let tile_0_prev = OneTileUpdate {
			x: still_snowball_x,
			y: still_snowball_y,
			new_tile: None,
			new_snowball: Some(self.snowballs[still_snowball_x][still_snowball_y]),
		};
		let tile_1_next = OneTileUpdate {
			x: moved_snowball_x,
			y: moved_snowball_y,
			new_tile: if remove_snow { Some(Tile::Dirt) } else { None },
			new_snowball: Some(Some(moved_snowball_type)),
		};
		let tile_1_prev = OneTileUpdate {
			x: moved_snowball_x,
			y: moved_snowball_y,
			new_tile: if remove_snow { Some(Tile::Snow) } else { None },
			new_snowball: Some(None),
		};

		return MapDiff {
			new: Update {
				player: (self.player.0, self.player.1),
				tiles: Some((tile_0_next, tile_1_next)),
			},
			old: Update {
				player: (self.player.0, self.player.1),
				tiles: Some((tile_0_prev, tile_1_prev)),
			},
		};
	}

	fn player_mounts_snowballs(
		&self,
		new_player_x: usize,
		new_player_y: usize,
		new_snowball_x: usize,
		new_snowball_y: usize,
		new_snowball_type: SnowBall,
	) -> MapDiff {
		let tile_0_next = OneTileUpdate {
			x: new_player_x,
			y: new_player_y,
			new_tile: None,
			new_snowball: Some(None),
		};
		let tile_0_prev = OneTileUpdate {
			x: new_player_x,
			y: new_player_y,
			new_tile: None,
			new_snowball: Some(self.snowballs[new_player_x][new_player_y]),
		};
		let tile_1_next = OneTileUpdate {
			x: new_snowball_x,
			y: new_snowball_y,
			new_tile: None,
			new_snowball: Some(Some(new_snowball_type)),
		};
		let tile_1_prev = OneTileUpdate {
			x: new_snowball_x,
			y: new_snowball_y,
			new_tile: None,
			new_snowball: Some(self.snowballs[new_snowball_x][new_snowball_y]),
		};

		return MapDiff {
			new: Update {
				player: (new_player_x, new_player_y),
				tiles: Some((tile_0_next, tile_1_next)),
			},
			old: Update {
				player: (self.player.0, self.player.1),
				tiles: Some((tile_0_prev, tile_1_prev)),
			},
		};
	}

	pub fn get_history(&self) -> &str {
		&self.input_history
	}

	pub fn apply_history(&mut self, history: &str) -> Result<(), char> {
		if let Some(c) = history
			.chars()
			.find(|c| !matches!(c, 'U' | 'L' | 'D' | 'R' | 'T' | '\n'))
		{
			return Err(c);
		}

		for (i, c) in history.chars().filter(|&c| c != '\n').enumerate() {
			let action_worked = match c {
				'U' => self.process_player_input_no_history(Direction::Up),
				'L' => self.process_player_input_no_history(Direction::Left),
				'D' => self.process_player_input_no_history(Direction::Down),
				'R' => self.process_player_input_no_history(Direction::Right),
				'T' => self.reset_current_level_no_history(),
				_ => unreachable!(),
			};
			if !action_worked {
				println!("Warning: the save data is not coherent with the current map. Error at character of index {i}.");
				self.input_history.push_str(&history[..i]);
				return Ok(());
			}
		}

		self.input_history.push_str(history);
		Ok(())
	}
}

impl Tile {
	pub fn blocks(self) -> bool {
		self == Self::Hedge || self == Self::Tree || self == Self::Obstacle
	}
}

fn try_step(x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
	match dir {
		Direction::Up => (x != 0).then(|| (x - 1, y)),
		Direction::Right => (y != SIZE_Y - 1).then(|| (x, y + 1)),
		Direction::Down => (x != SIZE_X - 1).then(|| (x + 1, y)),
		Direction::Left => (y != 0).then(|| (x, y - 1)),
	}
}

impl From<Direction> for char {
	fn from(value: Direction) -> Self {
		match value {
			Direction::Up => 'U',
			Direction::Left => 'L',
			Direction::Down => 'D',
			Direction::Right => 'R',
		}
	}
}

//////////////////////////////////

/// Data allowing to update one tile. Consists of the tile's
/// coordinates and either the new tile or new snowball, or both.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OneTileUpdate {
	x: usize,
	y: usize,
	new_tile: Option<Tile>,
	new_snowball: Option<Option<SnowBall>>,
}

/// Data allowing to perform a player's step. Changing the player's
/// position and optionally the map's tiles. If the player doesn't move,
/// his coordinates will be present nonetheless but identical to the
/// player's previous position.
#[derive(Clone, PartialEq, Eq, Debug)]
struct Update {
	player: (usize, usize),
	tiles: Option<(OneTileUpdate, OneTileUpdate)>,
}

/// A pair of Updates (new, old) which are each other's opposite.
/// Applying new and then old leaves the game in the same state.
#[derive(Clone, PartialEq, Eq, Debug)]
struct MapDiff {
	new: Update,
	old: Update,
}

//////////////////////////////////

const MAP: &str = include_str!("../map.txt");

impl Game {
	pub fn instanciate() -> Game {
		let mut tiles = [[Tile::Empty; SIZE_Y]; SIZE_X];
		let mut snowballs = [[None; SIZE_Y]; SIZE_X];
		let mut player_pos = None::<(usize, usize)>;

		for (i, c) in MAP.chars().filter(|c| *c != '\n').enumerate() {
			let x = i / SIZE_Y;
			let y = i % SIZE_Y;

			tiles[x][y] = Tile::from_char(c);
			snowballs[x][y] = SnowBall::from_char(c);
			if c == 'P' {
				assert!(player_pos.is_none(), "There cannot be two player's initial positions on the map.");
				player_pos = Some((x, y));
			}
		}
		assert!(player_pos.is_some(), "Missing player's initial positions on the map.");

		return Game {
			tiles,
			snowballs,
			player: player_pos.unwrap(),
			rewind_queue: VecDeque::with_capacity(64),
			input_history: String::with_capacity(64),
		};
	}
}

impl SnowBall {
	fn from_char(c: char) -> Option<SnowBall> {
		match c {
			's' | 'S' => Some(SnowBall::Small),
			'm' | 'M' => Some(SnowBall::Medium),
			'l' | 'L' => Some(SnowBall::Large),
			'x' | 'X' => Some(SnowBall::SmallOnMedium),

			_ => None,
		}
	}
}

impl Tile {
	fn from_char(c: char) -> Tile {
		match c {
			' ' => Tile::Empty,
			'.' | 's' | 'm' | 'l' | 'x' => Tile::Dirt,
			',' | 'S' | 'M' | 'L' | 'X' => Tile::Snow,
			'h' | '#' => Tile::Hedge,
			't' => Tile::Tree,
			'o' => Tile::Obstacle,

			_ => Tile::Empty,
		}
	}
}


/// Used to parse the SIZE_X and SIZE_Y global constants of game.rs
const fn str_to_usize(s: &str) -> usize {
	assert!(s.len() == 3);
	const fn d_to_usize(byte: u8) -> usize {
		assert!(0x30 <= byte && byte <= 0x39, "non-digit character");
		return (byte & 15) as usize;
	}

	let s = s.as_bytes();
	let unit = d_to_usize(s[s.len() - 1]);
	let tens = 10 * d_to_usize(s[s.len() - 2]);
	let hundreds = 100 * d_to_usize(s[s.len() - 3]);

	return hundreds + tens + unit;
}
