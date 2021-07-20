use jumping_jack::entities::*;
use jumping_jack::player::*;
use jumping_jack::platform::*;
use jumping_jack::mainstate::*;

use ggez::event::EventHandler;
use rand::Rng;

use std::env;
use std::path;

#[test]
fn player_doesnt_exceed_horizontal_limits() {
	let screen_midway_point =
		ggez::mint::Point2 {
			x: WindowMeasurements::WINDOW_WIDTH / 2.0,
			y: WindowMeasurements::WINDOW_HEIGHT,
		};

	let mut player = PlayerEntity::new(screen_midway_point);
	player.update(-1.0, 1.0);

	assert!(player.position.x >= 0.0);
	assert_eq!(player.position.x, PlayerEntity::LEFT_WINDOW_EDGE_IN_REGARDS_TO_PLAYERMODEL);

	player.update(1.0, 2.0);

	assert!(player.position.x <= WindowMeasurements::WINDOW_WIDTH);
	assert_eq!(player.position.x, PlayerEntity::RIGHT_WINDOW_EDGE_IN_REGARDS_TO_PLAYERMODEL);
}

#[test]
fn mobile_platform_doesnt_exceed_horizontal_limits() {
	let mut rng = rand::thread_rng();

	let design =
		match rng.gen_range(0_u32 ..= 4_u32) {
			0 => PlatformDesign::GrassDesign,
			1 => PlatformDesign::DesertDesign,
			2 => PlatformDesign::ForestDesign,
			3 => PlatformDesign::SnowDesign,
			4 => PlatformDesign::AutumnDesign,
			_ => PlatformDesign::GrassDesign,
		};
	
	let size =
		match rng.gen_range(0_u32 ..= 1_u32) {
			0 => PlatformSize::Small,
			1 => PlatformSize::Regular,
			_ => PlatformSize::Regular,
		};
	
	let point =
		ggez::mint::Point2 {
			x: rng.gen_range(MainState::HORIZONTAL_MOBILE_PLATFORM_GENERATION_RANGE),
			y: WindowMeasurements::WINDOW_HEIGHT / 2.0,
		};

	let mut platform = PlatformEntity::new(
			design,
   	   		size, 
	point,
  	  0,
   true
	);

	platform.update(false);
	assert!(platform.position.x >= 0.0);
	assert!(platform.position.x <= WindowMeasurements::WINDOW_WIDTH);
}

#[test]
fn player_doesnt_fall_below_bottom_screen_edge() {
	let (ref mut ctx, ref mut _event_loop) =
		ggez::ContextBuilder::new("Jumping Jack", "Georgi Gerginov").
			window_setup(
				ggez::conf::WindowSetup::default().
				title("Jumping Jack")).
			window_mode(
				ggez::conf::WindowMode::default().
				dimensions(WindowMeasurements::WINDOW_WIDTH, WindowMeasurements::WINDOW_HEIGHT)).
			build().
			unwrap();
	
	if let Ok(manifest_dir) =
	env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		
		path.push("resources");
		ggez::filesystem::mount(ctx, &path, true);
	}

	let state = &mut MainState::new(ctx).unwrap();

	while !state.platforms.is_empty() {
		state.platforms.pop_front();
	}

	state.player.position.y = WindowMeasurements::WINDOW_HEIGHT / 2.0;
	state.player.state = PlayerState::Falling;
	let _ = state.update(ctx);

	assert!(state.player.position.y <= WindowMeasurements::WINDOW_HEIGHT);
}

#[test]
fn player_doesnt_exceed_jump_crest() {
	let (ref mut ctx, ref mut _event_loop) =
		ggez::ContextBuilder::new("Jumping Jack", "Georgi Gerginov").
			window_setup(
				ggez::conf::WindowSetup::default().
				title("Jumping Jack")).
			window_mode(
				ggez::conf::WindowMode::default().
				dimensions(WindowMeasurements::WINDOW_WIDTH, WindowMeasurements::WINDOW_HEIGHT)).
			build().
			unwrap();
	
	if let Ok(manifest_dir) =
	env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		
		path.push("resources");
		ggez::filesystem::mount(ctx, &path, true);
	}

	let state = &mut MainState::new(ctx).unwrap();

	while !state.platforms.is_empty() {
		state.platforms.pop_front();
	}

	state.player_movement.can_jump = true;
	state.player_movement.is_jumping = true;

	let _ = state.update(ctx);

	assert!(state.player.position.y >= state.player.max_jump_height);
}

#[test]
fn player_animation_works() {
	let (ref mut ctx, ref mut _event_loop) =
		ggez::ContextBuilder::new("Jumping Jack", "Georgi Gerginov").
			window_setup(
				ggez::conf::WindowSetup::default().
				title("Jumping Jack")).
			window_mode(
				ggez::conf::WindowMode::default().
				dimensions(WindowMeasurements::WINDOW_WIDTH, WindowMeasurements::WINDOW_HEIGHT)).
			build().
			unwrap();
	
	if let Ok(manifest_dir) =
	env::var("CARGO_MANIFEST_DIR") {
		let mut path = path::PathBuf::from(manifest_dir);
		
		path.push("resources");
		ggez::filesystem::mount(ctx, &path, true);
	}

	let state = &mut MainState::new(ctx).unwrap();

	state.player.update(1.0, 60.0);
	let animation_changes = (state.player_movement.distance_travelled_laterally / 20.0).round() as u32;
	
	if animation_changes == 0 {
		assert_eq!(state.player.state, PlayerState::Idle);
	} else if animation_changes == 1 {
		assert_eq!(state.player.state, PlayerState::RunningPhase0);
	} else if animation_changes == 2 {
		assert_eq!(state.player.state, PlayerState::RunningPhase1);
	} else if animation_changes == 3 {
		assert_eq!(state.player.state, PlayerState::RunningPhase2);
	}
}
