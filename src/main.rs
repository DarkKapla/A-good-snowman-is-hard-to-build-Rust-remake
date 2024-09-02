//!
//!
//!

// http://docs.piston.rs/piston_window/piston_window/
use piston_window::*;

mod game;
mod save;
mod view;

const TITLE: &str = "A good snowcrab is hard to build.";
const SAVE_FILE: &str = "save.txt";

fn main() {
	let mut window: PistonWindow = WindowSettings::new(TITLE, [1200, 800])
		.exit_on_esc(true)
		.automatic_close(true)
		.build()
		.unwrap();

	window.set_max_fps(10);
	window.set_ups(100); // 0 disable update-events but also bumps the CPU consumption to 100%.
	window.set_lazy(false); // if true, the application consumes 100% of my CPU. Very intuitive.

	println!(
		"Rendering on {}.",
		window.device.get_info().platform_name.renderer
	);

	let commands = r#"
--- Controls ---
Use Z/Q/S/D to move around.
Press E or R to rewind one turn.
Press T to reset the current level. (WARNING: it erases the rewind memory)
Press space bar to recenter the view. Hold to make the cam follow the player.
Press ESC to quit.
"#;
	print!("{}", commands);

	let mut must_redraw = true;
	let mut cam_follows = false;
	let mut game = game::Game::instanciate();
	let mut viewport = view::Viewport::new(&game, (1200, 800));

	// Attempt to load the last game's save.
	if std::path::Path::new(SAVE_FILE).exists() {
		if let Err(e) = save::load(&mut game, SAVE_FILE) {
			println!("Error when loading the save file {SAVE_FILE}: {:?}.", e);
		} else {
			println!("Previous save loaded.");
		}
	}

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

		if let Some(Button::Keyboard(key)) = event.press_args() {
			let has_moved = match key {
				Key::Z | Key::Up => game.process_player_input(game::Direction::Up),
				Key::Q | Key::Left => game.process_player_input(game::Direction::Left),
				Key::S | Key::Down => game.process_player_input(game::Direction::Down),
				Key::D | Key::Right => game.process_player_input(game::Direction::Right),

				Key::E | Key::R => game.rewind(),

				Key::T => game.reset_current_level(),

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

		if let Some(Button::Keyboard(Key::Space)) = event.release_args() {
			cam_follows = false;
		}
	}

	// save the current game
	match save::save(&game, SAVE_FILE) {
		Err(e) => println!("Couldn't save file {SAVE_FILE}: {:?}", e.kind()),
		Ok(true) => println!("Game was saved to {SAVE_FILE}."),
		Ok(false) => {}
	}

	let snowmen_count = game
		.snowballs
		.iter()
		.flatten()
		.filter(|o| **o == Some(game::SnowBall::Snowman))
		.count();
	println!("Number of snowmen: {snowmen_count}. ⛄️");
}
