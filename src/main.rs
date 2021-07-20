use ggez::event;
use ggez::filesystem;
use ggez::graphics;
use ggez::{ContextBuilder};
use ggez::conf::{WindowMode, WindowSetup};

use jumping_jack::entities::{WindowMeasurements};
use jumping_jack::mainstate::{MainState};

use std::env;
use std::path;

pub fn main() {
	let (ref mut ctx, ref mut event_loop) =
		ContextBuilder::new("Jumping Jack", "Georgi Gerginov").
			window_setup(
				WindowSetup::default().
				title("Jumping Jack")).
			window_mode(
				WindowMode::default().
				dimensions(WindowMeasurements::WINDOW_WIDTH, WindowMeasurements::WINDOW_HEIGHT)).
			build().
			unwrap();
			
	if let Ok(manifest_dir) =
	env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		
		path.push("resources");
		filesystem::mount(ctx, &path, true);
	}

	let _ = graphics::set_window_icon(ctx, Some("/character_malePerson_idle_right.png"));

    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
