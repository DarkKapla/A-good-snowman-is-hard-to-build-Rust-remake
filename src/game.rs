//!
//!
//!

pub const SIZE_X: usize = 12;
pub const SIZE_Y: usize = 11;

/*
   +-----> y
   |
   v  x
*/
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SnowBall {
	Small,
	Medium,
	Big,
	SmallOnBig,
	SmallOnMedium,
	MediumOnBig,
	Snowman,
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
	rewind_stack: Vec<Update>,
}

impl Game {
	pub fn process_player_input(&mut self, dir: Direction) -> bool {
		if let Some(map_diff) = self.step(dir) {
			self.apply_update(&map_diff.new);
			self.rewind_stack.push(map_diff.old);
			// We ain't gonna let the stack grow to out of memory and beyond.
			if self.rewind_stack.len() >= 2048 {
				self.rewind_stack.rotate_right(1024);
				self.rewind_stack.truncate(1024);
			}
			return true;
		} else {
			return false;
		}
	}

	pub fn rewind(&mut self) -> bool {
		if let Some(update) = self.rewind_stack.pop() {
			self.apply_update(&update);
			return true;
		} else {
			println!("Cannot rewind any further.");
			return false;
		}
	}

	fn apply_update(&mut self, update: &Update) {
		self.player = update.player;

		let mut apply_one_time_update = |u: &OneTileUpdate| {
			if let Some(tile) = u.new_tile {
				self.tiles[u.x][u.y] = tile;
			}
			if let Some(snowball_opt) = u.new_snowball {
				self.snowballs[u.x][u.y] = snowball_opt;
			}
		};

		if let Some((ref tile0, ref tile1)) = update.tiles {
			apply_one_time_update(tile0);
			apply_one_time_update(tile1);
		}
	}

	// The logic of the game goes here, basically.
	fn step(&self, dir: Direction) -> Option<MapDiff> {
		let (x0, y0) = self.player;
		let target = try_step(x0, y0, dir);
		if target.is_none() {
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
					(SnowBall::Small, SnowBall::Big) => Some(SnowBall::SmallOnBig),
					(SnowBall::Medium, SnowBall::Big) => Some(SnowBall::MediumOnBig),
					(SnowBall::Small, SnowBall::MediumOnBig) => Some(SnowBall::Snowman),
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
				match target_snowball {
					SnowBall::Small | SnowBall::Medium | SnowBall::Big => {
						let new_snowball = if self.tiles[beyond_x][beyond_y] == Tile::Snow {
							// the snow ball grows
							match target_snowball {
								SnowBall::Small => SnowBall::Medium,
								SnowBall::Medium => SnowBall::Big,
								SnowBall::Big => SnowBall::Big,
								_ => unreachable!(),
							}
						} else {
							target_snowball
						};

						return Some(self.player_pushes_snowball(
							target_x,
							target_y,
							beyond_x,
							beyond_y,
							new_snowball,
							self.tiles[beyond_x][beyond_y] == Tile::Snow,
						));
					}

					SnowBall::SmallOnMedium | SnowBall::SmallOnBig | SnowBall::MediumOnBig => {
						let (still_snowball, pushed_snowball) = match target_snowball {
							SnowBall::SmallOnMedium => (SnowBall::Medium, SnowBall::Small),
							SnowBall::SmallOnBig => (SnowBall::Big, SnowBall::Small),
							SnowBall::MediumOnBig => (SnowBall::Big, SnowBall::Medium),
							_ => unreachable!(),
						};

						return Some(self.snowball_descends(
							target_x,
							target_y,
							still_snowball,
							beyond_x,
							beyond_y,
							pushed_snowball,
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
			new_tile: None,
			new_snowball: Some(Some(moved_snowball_type)),
		};
		let tile_1_prev = OneTileUpdate {
			x: moved_snowball_x,
			y: moved_snowball_y,
			new_tile: None,
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

//////////////////////////////////

/// Data allowing to update one tile. Consists of the tile's
/// coordinates and either the new tile or new snowball, or both.
#[derive(Clone, PartialEq, Eq, Debug)]
struct OneTileUpdate {
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

const MAP: &'static str = include_str!("../map.txt");

impl Game {
	pub fn instanciate() -> Game {
		let mut tiles = [[Tile::Empty; SIZE_Y]; SIZE_X];
		let mut snowballs = [[None; SIZE_Y]; SIZE_X];
		let mut player_x = !0;
		let mut player_y = !0;

		for (i, c) in MAP.chars().filter(|c| *c != '\n').enumerate() {
			let x = i / SIZE_Y;
			let y = i % SIZE_Y;

			tiles[x][y] = Tile::from_char(c);
			snowballs[x][y] = SnowBall::from_char(c);
			if c == 'P' {
				player_x = x;
				player_y = y;
			}
		}
		assert_ne!(player_x, !0);
		assert_ne!(player_y, !0);

		return Game {
			tiles,
			snowballs,
			player: (player_x, player_y),
			rewind_stack: Vec::with_capacity(64),
		};
	}
}

impl SnowBall {
	fn from_char(c: char) -> Option<SnowBall> {
		match c {
			's' | 'S' => Some(SnowBall::Small),
			'm' | 'M' => Some(SnowBall::Medium),
			'b' | 'B' => Some(SnowBall::Big),

			_ => None,
		}
	}
}

impl Tile {
	fn from_char(c: char) -> Tile {
		match c {
			' ' => Tile::Empty,
			'.' | 's' | 'm' | 'b' => Tile::Dirt,
			',' | 'S' | 'M' | 'B' => Tile::Snow,
			'h' => Tile::Hedge,
			't' => Tile::Tree,
			'o' => Tile::Obstacle,

			_ => Tile::Empty,
		}
	}
}
