use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::{Point2};
use rand::Rng;
use std::ops::Range;

use crate::entities::{WindowMeasurements, BoundableEntity, BoundingRectEdges};
use crate::assets::LevelModels;

#[derive(Debug, Eq, PartialEq)]
pub enum PlatformDesign {
	GrassDesign,
	DesertDesign,
	ForestDesign,
	SnowDesign,
	AutumnDesign,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PlatformSize {
	Small,
	Regular,
	Large,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PlatformState {
	Intact,
	Broken,
	Nonexistent,
}

#[derive(Debug)]
pub struct PlatformHorizontalMovementHandler {
	pub is_mobile:                  bool,
	pub can_go_left:                bool,
	pub leftmost_lateral_position:  f32,
	pub can_go_right:               bool,
	pub rightmost_lateral_position: f32,
	pub gravity:                    f32,
}

impl PlatformHorizontalMovementHandler {
	const HORIZONTAL_MOVEMENT_OFFSET: f32 = 141.0;

	pub const GRASS_LEVEL_LATERAL_UNITS_TRAVELLED: f32  = 1.0;
	pub const DESERT_LEVEL_LATERAL_UNITS_TRAVELLED: f32 = 1.1;
	pub const FOREST_LEVEL_LATERAL_UNITS_TRAVELLED: f32 = 1.2;
	pub const SNOW_LEVEL_LATERAL_UNITS_TRAVELLED: f32   = 1.3;
	pub const AUTUMN_LEVEL_LATERAL_UNITS_TRAVELLED: f32 = 1.4;

	pub fn new(position: Point2<f32>, is_mobile: bool) -> Self {
		let mut rng = rand::thread_rng();
		let (can_go_left, can_go_right) =
			match is_mobile {
				true =>
					match rng.gen_range(0_u32 .. 2_u32) == 0 {
						true  => (true, false),
						false => (false, true),
					},

				false => (false, false),
			};

		let (leftmost_lateral_position, rightmost_lateral_position) =
			match is_mobile {
				true => (position.x - Self::HORIZONTAL_MOVEMENT_OFFSET, position.x + Self::HORIZONTAL_MOVEMENT_OFFSET),
				false => (position.x, position.x),
			};

		PlatformHorizontalMovementHandler {
			is_mobile,
			can_go_left,
			leftmost_lateral_position,
			can_go_right,
			rightmost_lateral_position,
			gravity: 0.0,
		}
	}
}

#[derive(Debug)]
pub struct PlatformVerticalMovementHandler {
	pub platform_update_timer:       f32,
	pub distance_till_next_platform: f32,
	pub score_of_latest_platform:    u32,
}

impl PlatformVerticalMovementHandler {
	pub const PLATFORM_UPDATE_DURATION: f32              = 0.5;
	pub const MAX_PLAYER_HEIGHT_FOR_PLATFORM_UPDATE: f32 = 641.0;

	pub const DEFAULT_GRAVITY_MULTIPLIER: f32      = 2.0;

	pub fn new() -> Self {
		PlatformVerticalMovementHandler {
			platform_update_timer:       Self::PLATFORM_UPDATE_DURATION,
			distance_till_next_platform: WindowMeasurements::WINDOW_HEIGHT - 84.0,
			score_of_latest_platform:    0,
		}
	}
}

#[derive(Debug)]
pub struct PlatformScoreHandler {
	pub score:                    u32,
	pub score_has_been_collected: bool,
}

impl PlatformScoreHandler {
	pub const POINTS: u32 = 100;

	pub fn new(score: u32) -> Self {
		PlatformScoreHandler {
			score,
			score_has_been_collected: false,
		}
	}
}

#[derive(Debug)]
pub struct PtoPInteractionHandler {
	pub player_is_on_platform:      bool,
	pub time_spent_on_platform:     f32,
}

impl PtoPInteractionHandler {
	pub const GRASS_LEVEL_BROKEN_STATE_TIME_RANGE: Range<f32>  = 0.7 .. 0.9;
	pub const DESERT_LEVEL_BROKEN_STATE_TIME_RANGE: Range<f32> = 0.6 .. 0.8;
	pub const FOREST_LEVEL_BROKEN_STATE_TIME_RANGE: Range<f32> = 0.5 .. 0.7;
	pub const SNOW_LEVEL_BROKEN_STATE_TIME_RANGE: Range<f32>   = 0.4 .. 0.6;
	pub const AUTUMN_LEVEL_BROKEN_STATE_TIME_RANGE: Range<f32> = 0.3 .. 0.5;

	pub fn new() -> PtoPInteractionHandler {
		PtoPInteractionHandler {
			player_is_on_platform:      false,
			time_spent_on_platform:     0.0,
		}
	}
}

#[derive(Debug)]
pub struct PlatformEntity {
	pub design:              PlatformDesign,
	pub size:                PlatformSize,
	pub state:               PlatformState,
	pub position:            Point2<f32>,
	pub bounding_rect_edges: Option<BoundingRectEdges>,
	pub movement:            PlatformHorizontalMovementHandler,
	pub scoring:             PlatformScoreHandler,
	pub interaction:         PtoPInteractionHandler,
}

impl PlatformEntity {
	pub const VERTICAL_PLATFORM_OFFSET: f32 = 206.0;
	const LEFT_HITBOX_EDGE_OFFSET: f32      = 7.5;
	const RIGHT_HITBOX_EDGE_OFFSET: f32     = 7.5;
	
	pub fn new(design: PlatformDesign, size: PlatformSize, position: Point2<f32>, score: u32, is_mobile: bool) -> Self {
		PlatformEntity {
			design,
			size,
			state:               PlatformState::Intact,
			position,
			bounding_rect_edges: None,
			movement:            PlatformHorizontalMovementHandler::new(position, is_mobile),
			scoring:             PlatformScoreHandler::new(score),
			interaction:         PtoPInteractionHandler::new(),
		}
	}

	fn move_right(&mut self) {
		match self.design {
			PlatformDesign::GrassDesign  => self.position.x += PlatformHorizontalMovementHandler::GRASS_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::DesertDesign => self.position.x += PlatformHorizontalMovementHandler::DESERT_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::ForestDesign => self.position.x += PlatformHorizontalMovementHandler::FOREST_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::SnowDesign   => self.position.x += PlatformHorizontalMovementHandler::SNOW_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::AutumnDesign => self.position.x += PlatformHorizontalMovementHandler::AUTUMN_LEVEL_LATERAL_UNITS_TRAVELLED,
		}
	}
	
	fn move_left(&mut self) {
		match self.design {
			PlatformDesign::GrassDesign  => self.position.x -= PlatformHorizontalMovementHandler::GRASS_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::DesertDesign => self.position.x -= PlatformHorizontalMovementHandler::DESERT_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::ForestDesign => self.position.x -= PlatformHorizontalMovementHandler::FOREST_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::SnowDesign   => self.position.x -= PlatformHorizontalMovementHandler::SNOW_LEVEL_LATERAL_UNITS_TRAVELLED,
			PlatformDesign::AutumnDesign => self.position.x -= PlatformHorizontalMovementHandler::AUTUMN_LEVEL_LATERAL_UNITS_TRAVELLED,
		}
	}

	fn move_right_to_left(&mut self) {
		if self.movement.can_go_right {
			if self.position.x > self.movement.rightmost_lateral_position {
				self.movement.can_go_right = false;
				self.movement.can_go_left  = true;

			} else {
				self.move_right();
			}

		} else if self.movement.can_go_left {
			if self.position.x < self.movement.leftmost_lateral_position {
				self.movement.can_go_left  = false;
				self.movement.can_go_right = true;

			} else {
				self.move_left();
			}
		}
	}

	fn change_platform_position_along_x_axis(&mut self, player_is_in_jumping_state: bool) {
		if !player_is_in_jumping_state {
			if self.movement.is_mobile {
				self.move_right_to_left();
			}
		}
	}

	fn change_platform_position_along_y_axis(&mut self, player_is_in_jumping_state: bool) {
		if player_is_in_jumping_state {
			self.position.y += self.movement.gravity;
		}
	}
	
	pub fn update(&mut self, player_is_in_jumping_state: bool) {
		self.change_platform_position_along_x_axis(player_is_in_jumping_state);
		self.change_platform_position_along_y_axis(player_is_in_jumping_state);
	}

	fn update_bounding_rect_edges(&mut self, image: &graphics::ImageGeneric<graphics::GlBackendSpec>) {
		let left   = self.position.x + Self::LEFT_HITBOX_EDGE_OFFSET;
		let top    = self.position.y;
		let right  = self.position.x + image.width() as f32 - Self::RIGHT_HITBOX_EDGE_OFFSET;
		let bottom = self.position.y;

		self.bounding_rect_edges = Some(BoundingRectEdges::new(left, top, right, bottom));
	}

	fn draw_platform(&mut self, ctx: &mut Context,  image: &graphics::ImageGeneric<graphics::GlBackendSpec>) {
		self.update_bounding_rect_edges(image);
		graphics::draw(ctx, image, graphics::DrawParam {
			dest: self.position,
			.. Default::default()
		}).unwrap();
	}
	
	pub fn draw(&mut self, ctx: &mut Context, models: &LevelModels) -> GameResult<()> {
		let image =
			match self.design {

				PlatformDesign::GrassDesign =>
					match self.size {

						PlatformSize::Small =>
							match self.state {
								PlatformState::Intact      => &models.grass_small_platform,
								PlatformState::Broken      => &models.grass_small_broken_platform,
								PlatformState::Nonexistent => &models.transparent_small_platform,
							},

						PlatformSize::Regular =>
							match self.state {
								PlatformState::Intact      => &models.grass_regular_platform,
								PlatformState::Broken      => &models.grass_regular_broken_platform,
								PlatformState::Nonexistent => &models.transparent_regular_platform,
							},

						PlatformSize::Large => &models.grass_large_platform,
					},
				
				PlatformDesign::DesertDesign =>
					match self.size {

						PlatformSize::Small =>
							match self.state {
								PlatformState::Intact      => &models.desert_small_platform,
								PlatformState::Broken      => &models.desert_small_broken_platform,
								PlatformState::Nonexistent => &models.transparent_small_platform,
							},

						PlatformSize::Regular =>
							match self.state {
								PlatformState::Intact      => &models.desert_regular_platform,
								PlatformState::Broken      => &models.desert_regular_broken_platform,
								PlatformState::Nonexistent => &models.transparent_regular_platform,
							},

						PlatformSize::Large => &models.desert_large_platform,
					},
				
				PlatformDesign::ForestDesign =>
					match self.size {

						PlatformSize::Small =>
							match self.state {
								PlatformState::Intact      => &models.forest_small_platform,
								PlatformState::Broken      => &models.forest_small_broken_platform,
								PlatformState::Nonexistent => &models.transparent_small_platform,
							},

						PlatformSize::Regular =>
							match self.state {
								PlatformState::Intact      => &models.forest_regular_platform,
								PlatformState::Broken      => &models.forest_regular_broken_platform,
								PlatformState::Nonexistent => &models.transparent_regular_platform,
							},

						PlatformSize::Large => &models.forest_large_platform,
					},
				
				PlatformDesign::SnowDesign =>
					match self.size {

						PlatformSize::Small =>
							match self.state {
								PlatformState::Intact      => &models.snow_small_platform,
								PlatformState::Broken      => &models.snow_small_broken_platform,
								PlatformState::Nonexistent => &models.transparent_small_platform,
							},

						PlatformSize::Regular =>
							match self.state {
								PlatformState::Intact      => &models.snow_regular_platform,
								PlatformState::Broken      => &models.snow_regular_broken_platform,
								PlatformState::Nonexistent => &models.transparent_regular_platform,
							},

						PlatformSize::Large => &models.snow_large_platform,
					},

				PlatformDesign::AutumnDesign =>
					match self.size {
					
						PlatformSize::Small =>
							match self.state {
								PlatformState::Intact      => &models.autumn_small_platform,
								PlatformState::Broken      => &models.autumn_small_broken_platform,
								PlatformState::Nonexistent => &models.transparent_small_platform,
							},
						
						PlatformSize::Regular =>
							match self.state {
								PlatformState::Intact      => &models.autumn_regular_platform,
								PlatformState::Broken      => &models.autumn_regular_broken_platform,
								PlatformState::Nonexistent => &models.transparent_regular_platform,
							},
						
						PlatformSize::Large => &models.autumn_large_platform,
					},
			};
		
		self.draw_platform(ctx, image);
		Ok(())
	}
}

impl BoundableEntity for PlatformEntity {
	fn bounding_rect(&self) -> graphics::Rect {
		match self.bounding_rect_edges {
			None => graphics::Rect::zero(),
			Some(e) => {
				graphics::Rect::new(
					e.left, 
					e.top, 
					e.right - e.left, 
					e.bottom - e.top,
				)
			},
		}
	}
}
