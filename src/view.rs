//!
//!
//!

pub const TILE_SIDE: f64 = 64.0;
const TILE_RECTANGLE: [f64; 4] = [0.0, 0.0, TILE_SIDE, TILE_SIDE];
// snow ball
const RADIUS_SMALL: f64 = 0.1875;
const RADIUS_MEDIUM: f64 = 0.4375;
const RADIUS_LARGE: f64 = 0.6875;
const BALL_SMALL: [f64; 4] = [
	TILE_SIDE * (1.0 - RADIUS_SMALL) / 2.0,
	TILE_SIDE * 0.5625,
	TILE_SIDE * RADIUS_SMALL,
	TILE_SIDE * RADIUS_SMALL,
];
const BALL_MEDIUM: [f64; 4] = [
	TILE_SIDE * (1.0 - RADIUS_MEDIUM) / 2.0,
	TILE_SIDE * 0.375,
	TILE_SIDE * RADIUS_MEDIUM,
	TILE_SIDE * RADIUS_MEDIUM,
];
const BALL_LARGE: [f64; 4] = [
	TILE_SIDE * (1.0 - RADIUS_LARGE) / 2.0,
	TILE_SIDE * 0.1875,
	TILE_SIDE * RADIUS_LARGE,
	TILE_SIDE * RADIUS_LARGE,
];
// the player
const PLAYER_RECTANGLE: [f64; 4] = [
	TILE_SIDE / 4.0,
	TILE_SIDE / 4.0,
	TILE_SIDE / 2.0,
	TILE_SIDE * 0.75,
];

const SNOWBALL_DRAWER: Ellipse = Ellipse {
	color: [0.875, 0.875, 0.875, 1.0],
	border: Some(ellipse::Border {
		color: [0.0, 0.0, 0.0, 1.0],
		radius: 1.0,
	}),
	resolution: 128,
};

use piston_window::{clear, ellipse, line_from_to, rectangle, Context, Ellipse, G2d, Transformed};

use crate::game;

pub fn draw_all(vp: Viewport, game: &game::Game, context: Context, graphics: &mut G2d) {
	clear([0.125, 0.125, 0.125, 1.0], graphics);

	let max_x = usize::min(vp.base_x + vp.len_x + 1, game::SIZE_X);
	let max_y = usize::min(vp.base_y + vp.len_y + 1, game::SIZE_Y);

	for x in (vp.base_x)..max_x {
		for y in (vp.base_y)..max_y {
			draw_one_tile(x, y, vp, game, context, graphics);
		}
	}

	let (px, py) = game.player;
	if vp.base_x <= px && px < max_x && vp.base_y <= py && py < max_y {
		draw_player(px - vp.base_x, py - vp.base_y, context, graphics);
	}
}

/// Draw all but the player.
fn draw_one_tile(
	x: usize,
	y: usize,
	vp: Viewport,
	game: &game::Game,
	context: Context,
	graphics: &mut G2d,
) {
	let tx = (x - vp.base_x) as f64 * TILE_SIDE;
	let ty = (y - vp.base_y) as f64 * TILE_SIDE;
	// Background tile.
	let color = match game.tiles[x][y] {
		game::Tile::Empty => [0.375, 0.375, 0.375, 1.0],
		game::Tile::Dirt => [0.625, 0.4375, 0.125, 1.0],
		game::Tile::Snow => [0.8125, 0.8125, 0.875, 1.0],
		game::Tile::Hedge => [0.0625, 0.375, 0.0, 1.0],
		game::Tile::Tree => [0.125, 0.75, 0.25, 1.0],
		game::Tile::Obstacle => [0.375, 0.25, 0.375, 1.0],
	};
	rectangle(
		color,
		TILE_RECTANGLE,
		context.transform.trans(ty, tx),
		graphics,
	);
	// Foreground snow balls.
	if let Some(snowball) = game.snowballs[x][y] {
		let rect = match snowball {
			game::SnowBall::Small => BALL_SMALL,
			game::SnowBall::Medium | game::SnowBall::SmallOnMedium => BALL_MEDIUM,
			game::SnowBall::Large
			| game::SnowBall::MediumOnLarge
			| game::SnowBall::SmallOnLarge
			| game::SnowBall::Snowman => BALL_LARGE,
		};
		SNOWBALL_DRAWER.draw(
			rect,
			&context.draw_state,
			context.transform.trans(ty, tx),
			graphics,
		);

		if snowball == game::SnowBall::SmallOnMedium {
			SNOWBALL_DRAWER.draw(
				BALL_SMALL,
				&context.draw_state,
				context.transform.trans(ty, tx - TILE_SIDE * 0.3125),
				graphics,
			);
		} else if snowball == game::SnowBall::SmallOnLarge {
			SNOWBALL_DRAWER.draw(
				BALL_SMALL,
				&context.draw_state,
				context.transform.trans(ty, tx - TILE_SIDE * 0.4375),
				graphics,
			);
		} else if snowball == game::SnowBall::MediumOnLarge {
			SNOWBALL_DRAWER.draw(
				BALL_MEDIUM,
				&context.draw_state,
				context.transform.trans(ty, tx - TILE_SIDE * 0.25),
				graphics,
			);
		} else if snowball == game::SnowBall::Snowman {
			SNOWBALL_DRAWER.draw(
				BALL_MEDIUM,
				&context.draw_state,
				context.transform.trans(ty, tx - TILE_SIDE * 0.25),
				graphics,
			);
			SNOWBALL_DRAWER.draw(
				BALL_SMALL,
				&context.draw_state,
				context.transform.trans(ty, tx - TILE_SIDE * 0.5),
				graphics,
			);
			line_from_to(
				[0.375, 0.25, 0.0652, 1.0],
				2.0,
				[ty + TILE_SIDE * 0.375, tx + TILE_SIDE * 0.5],
				[ty + TILE_SIDE * 0.125, tx + TILE_SIDE * 0.125],
				context.transform,
				graphics,
			);
			line_from_to(
				[0.375, 0.25, 0.0652, 1.0],
				2.0,
				[ty + TILE_SIDE * 0.652, tx + TILE_SIDE * 0.5],
				[ty + TILE_SIDE * 0.875, tx + TILE_SIDE * 0.125],
				context.transform,
				graphics,
			);
		}
	}
}

fn draw_player(x: usize, y: usize, context: Context, graphics: &mut G2d) {
	let tx = x as f64 * TILE_SIDE;
	let ty = y as f64 * TILE_SIDE;
	rectangle(
		[0.75, 0.0625, 0.125, 1.0],
		PLAYER_RECTANGLE,
		context.transform.trans(ty, tx),
		graphics,
	);
}

/// Store data to print only a sub-view of the map.
/// It's a rectangle. All the map tiles inside the
/// rectangle will be drawn in the window.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Viewport {
	pub base_x: usize,
	pub base_y: usize,
	pub len_x: usize,
	pub len_y: usize,
}

impl Viewport {
	pub fn new(game: &game::Game, window_size: (usize, usize)) -> Viewport {
		let len_x = (window_size.0 as f64 / TILE_SIDE) as usize;
		let len_y = (window_size.1 as f64 / TILE_SIDE) as usize;
		let base_x = game.player.0.checked_sub(len_x / 2).unwrap_or(0);
		let base_y = game.player.1.checked_sub(len_y / 2).unwrap_or(0);
		Viewport {
			base_x,
			base_y,
			len_x,
			len_y,
		}
	}

	pub fn center_around_player(&mut self, game: &game::Game) {
		self.base_x = game.player.0.checked_sub(self.len_x / 2).unwrap_or(0).min(game::SIZE_X - self.len_x);
		self.base_y = game.player.1.checked_sub(self.len_y / 2).unwrap_or(0).min(game::SIZE_Y - self.len_y);
	}

	pub fn resize(&mut self, args: piston_window::ResizeArgs) {
		self.len_x = (args.window_size[1] / TILE_SIDE) as usize;
		self.len_y = (args.window_size[0] / TILE_SIDE) as usize;
	}
}
