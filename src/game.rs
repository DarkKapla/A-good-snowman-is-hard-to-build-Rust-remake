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

/// Tell what happened in the Game, optionally contain data about the
/// previous positions of some entity, which need to be redrawn.
/// Does not contain the new position of the Player as the Game has got it.
#[derive(Clone, Debug)]
pub enum MoveEvent {
	NoMove,
	PlayerMoved {
		old_x: usize,
		old_y: usize,
	},
	SnowBallDescended {
		still_snowball_x: usize,
		still_snowball_y: usize,
		new_snowball_x: usize,
		new_snowball_y: usize,
	},
	PlayerAndSnowBallMoved {
		old_player_x: usize,
		old_player_y: usize,
		new_snowball_x: usize,
		new_snowball_y: usize,
	},
}

#[derive(Clone, Debug)]
pub struct Game {
	pub tiles: [[Tile; SIZE_Y]; SIZE_X],
	pub snowballs: [[Option<SnowBall>; SIZE_Y]; SIZE_X],
	pub player: (usize, usize),
}

impl Game {
	// The logic of the game goes here, basically.
	pub fn step(&mut self, dir: Direction) -> MoveEvent {
		let (x0, y0) = self.player;
		let target = try_step(x0, y0, dir);
		if target.is_none() {
			return MoveEvent::NoMove;
		}
		let (x, y) = target.unwrap();

		match (self.tiles[x][y].blocks(), self.snowballs[x][y]) {
			(true, _) | (false, Some(SnowBall::Snowman)) => {
				return MoveEvent::NoMove;
			}
			(false, None) => {
				self.player = (x, y);
				return MoveEvent::PlayerMoved {
					old_x: x0,
					old_y: y0,
				};
			}
			(false, Some(_)) => {
				return self.try_push_snowball(dir, x, y);
			}
		}
	}

	fn try_push_snowball(&mut self, dir: Direction, target_x: usize, target_y: usize) -> MoveEvent {
		if let Some((beyond_x, beyond_y)) = try_step(target_x, target_y, dir) {
			if self.tiles[beyond_x][beyond_y].blocks()
				|| self.snowballs[beyond_x][beyond_y] == Some(SnowBall::Snowman)
			{
				return MoveEvent::NoMove;
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
				if let Some(mounted_snowball) = result {
					self.snowballs[target_x][target_y] = None;
					self.snowballs[beyond_x][beyond_y] = Some(mounted_snowball);

					// the player
					let (old_x, old_y) = self.player;
					self.player = (target_x, target_y);

					return MoveEvent::PlayerAndSnowBallMoved {
						old_player_x: old_x,
						old_player_y: old_y,
						new_snowball_x: beyond_x,
						new_snowball_y: beyond_y,
					};
				} else {
					return MoveEvent::NoMove;
				}
			} else {
				match target_snowball {
					SnowBall::Small | SnowBall::Medium | SnowBall::Big => {
						let new_snowball = if self.tiles[beyond_x][beyond_y] == Tile::Snow {
							// the snow ball grows
							match target_snowball {
								SnowBall::Small => {
									self.tiles[beyond_x][beyond_y] = Tile::Dirt;
									SnowBall::Medium
								}
								SnowBall::Medium => {
									self.tiles[beyond_x][beyond_y] = Tile::Dirt;
									SnowBall::Big
								}
								SnowBall::Big => SnowBall::Big,
								_ => unreachable!(),
							}
						} else {
							target_snowball
						};

						self.snowballs[target_x][target_y] = None;
						self.snowballs[beyond_x][beyond_y] = Some(new_snowball);

						// the player
						let (old_x, old_y) = self.player;
						self.player = (target_x, target_y);

						return MoveEvent::PlayerAndSnowBallMoved {
							old_player_x: old_x,
							old_player_y: old_y,
							new_snowball_x: beyond_x,
							new_snowball_y: beyond_y,
						};
					}

					SnowBall::SmallOnMedium | SnowBall::SmallOnBig | SnowBall::MediumOnBig => {
						let (still_snowball, pushed_snowball) = match target_snowball {
							SnowBall::SmallOnMedium => (SnowBall::Medium, SnowBall::Small),
							SnowBall::SmallOnBig => (SnowBall::Big, SnowBall::Small),
							SnowBall::MediumOnBig => (SnowBall::Big, SnowBall::Medium),
							_ => unreachable!(),
						};

						self.snowballs[target_x][target_y] = Some(still_snowball);
						self.snowballs[beyond_x][beyond_y] = Some(pushed_snowball);

						return MoveEvent::SnowBallDescended {
							still_snowball_x: target_x,
							still_snowball_y: target_y,
							new_snowball_x: beyond_x,
							new_snowball_y: beyond_y,
						};
					}

					SnowBall::Snowman => unreachable!(),
				}
			}
		} else {
			return MoveEvent::NoMove;
		}
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
