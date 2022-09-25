//!
//!
//!

const TILE_SIDE: f64 = 64.0;
const TILE_RECTANGLE: [f64; 4] = [0.0, 0.0, TILE_SIDE, TILE_SIDE];
// snow ball
const RADIUS_SMALL: f64 = 0.1875;
const RADIUS_MEDIUM: f64 = 0.4375;
const RADIUS_BIG: f64 = 0.6875;
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
const BALL_BIG: [f64; 4] = [
	TILE_SIDE * (1.0 - RADIUS_BIG) / 2.0,
	TILE_SIDE * 0.1875,
	TILE_SIDE * RADIUS_BIG,
	TILE_SIDE * RADIUS_BIG,
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

pub fn draw_afer_move(game: &game::Game, context: Context, graphics: &mut G2d) {
	todo!();
}

pub fn draw_all(game: &game::Game, context: Context, graphics: &mut G2d) {
	clear([0.125, 0.125, 0.125, 1.0], graphics);
	for x in 0..game::SIZE_X {
		for y in 0..game::SIZE_Y {
			draw_one_tile(x, y, game, context, graphics);
		}
	}
	draw_player(game.player.0, game.player.1, context, graphics)
}

/// Draw all but the player.
fn draw_one_tile(x: usize, y: usize, game: &game::Game, context: Context, graphics: &mut G2d) {
	let tx = x as f64 * TILE_SIDE;
	let ty = y as f64 * TILE_SIDE;
	// Background tile.
	let color = match game.tiles[x][y] {
		game::Tile::Empty => [0.125, 0.125, 0.125, 1.0],
		game::Tile::Dirt => [0.625, 0.4375, 0.125, 1.0],
		game::Tile::Snow => [0.8125, 0.8125, 0.875, 1.0],
		game::Tile::Hedge => [0.125, 0.5, 0.0, 1.0],
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
			game::SnowBall::Big
			| game::SnowBall::MediumOnBig
			| game::SnowBall::SmallOnBig
			| game::SnowBall::Snowman => BALL_BIG,
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
		} else if snowball == game::SnowBall::SmallOnBig {
			SNOWBALL_DRAWER.draw(
				BALL_SMALL,
				&context.draw_state,
				context.transform.trans(ty, tx - TILE_SIDE * 0.4375),
				graphics,
			);
		} else if snowball == game::SnowBall::MediumOnBig {
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
