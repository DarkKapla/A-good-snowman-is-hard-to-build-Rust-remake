//!
//!
//!

// http://docs.piston.rs/piston_window/piston_window/
use piston_window::*;

mod game;
mod view;

const TITLE: &'static str = "A good snowcrab is hard to build.";

fn main() {
	let mut window: PistonWindow = WindowSettings::new(TITLE, [1200, 800])
		.exit_on_esc(true)
		.automatic_close(true)
		.build()
		.unwrap();

	window.set_max_fps(10);
	window.set_ups(0); // disable update-events
	window.set_lazy(false); // if true, the application consumes 100% of my CPU. Very intuitive.

	let commands = r#"
--- Controls ---
Use Z/Q/S/D to move around.
Press E or R to rewind one turn.
Press space bar to recenter the view.
Press ESC to quit."#;
	println!(
		"Rendering on {}.\n{}\n",
		window.device.get_info().platform_name.renderer,
		commands
	);

	let mut must_redraw = true;
	let mut cam_follows = false;
	let mut game = game::Game::instanciate();
	let mut viewport = view::Viewport::new(&game, (1200, 800));

	// let texture_context = window.create_texture_context();
	// let mut glyph =
	// 	piston_window::Glyphs::from_bytes(FONT_DATA, texture_context, TextureSettings::new())
	// 		.unwrap();

	while let Some(event) = window.next() {
		// It draws only if the event is a render event. The check is automatic.
		window.draw_2d(&event, |context, graphics, _device| {
			if must_redraw {
				must_redraw = false;
				view::draw_all(viewport, &game, context, graphics);
			}
		});

		if let Some(args) = event.resize_args() {
			viewport.resize(args);
			viewport.center_around_player(&game);
			must_redraw = true;
		}

		if let Some(button) = event.press_args() {
			if let Button::Keyboard(key) = button {
				let has_moved = match key {
					Key::Z | Key::Up => game.process_player_input(game::Direction::Up),
					Key::Q | Key::Left => game.process_player_input(game::Direction::Left),
					Key::S | Key::Down => game.process_player_input(game::Direction::Down),
					Key::D | Key::Right => game.process_player_input(game::Direction::Right),

					Key::E | Key::R => game.rewind(),

					_ => false,
				};
				if (has_moved && cam_follows) || key == Key::Space {
					viewport.center_around_player(&game);
					must_redraw = true;
					if key == Key::Space {
						cam_follows = true;
					}
				} else if has_moved {
					must_redraw = true;
				}
			}
		}

		if let Some(button) = event.release_args() {
			if button == Button::Keyboard(Key::Space) {
				cam_follows = false;
			}
		}
	}

	let snowmen_count = game
		.snowballs
		.iter()
		.flatten()
		.filter(|o| **o == Some(game::SnowBall::Snowman))
		.count();
	println!("Number of snowmen: {snowmen_count}. ⛄️");
}

/// Used to parse the SIZE_X and SIZE_Y global constants of game.rs
const fn str_to_usize(s: &str) -> usize {
	const fn d_to_usize(byte: u8) -> usize {
		match byte {
			0x30 => 0,
			0x31 => 1,
			0x32 => 2,
			0x33 => 3,
			0x34 => 4,
			0x35 => 5,
			0x36 => 6,
			0x37 => 7,
			0x38 => 8,
			0x39 => 9,
			_ => panic!("non-digit character"),
		}
	}
	let s = s.as_bytes();
	let unit = d_to_usize(s[s.len() - 1]);
	let tens = 10 * d_to_usize(s[s.len() - 2]);
	let hundreds = 100 * d_to_usize(s[s.len() - 3]);

	return hundreds + tens + unit;
}
