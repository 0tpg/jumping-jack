use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::mint::{Point2};

use crate::entities::{Physics, BoundableEntity, BoundingRectEdges};
use crate::assets::PlayerModel;

#[derive(Debug, Eq, PartialEq)]
pub enum PlayerState {
	Idle,
	RunningPhase0,
	RunningPhase1,
	RunningPhase2,
	Jumping,
	Falling,
	Dead,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PlayerDirection {
	Right,
	Left,
}

#[derive(Debug)]
pub struct PlayerMovementHandler {
    pub units_travelled_laterally:    f32,
	pub distance_travelled_laterally: f32,
	pub running_phase_factor:         u32,
	pub can_jump:                     bool,
	pub jump_delay:                   f32,
	pub is_jumping:                   bool,
	pub is_falling:                   bool,
}

impl PlayerMovementHandler {
	pub const JUMP_DELAY: f32              = 0.175;
	pub const PLAYER_UNITS_TRAVELLED: f32  = 1.0;

	pub const DEFAULT_FREQUENCY: f32                           = 20.0; 
	pub const SNOW_LEVEL_RUNNING_PHASE_CHANGE_FREQUENCY: f32   = 10.0;
	pub const AUTUMN_LEVEL_RUNNING_PHASE_CHANGE_FREQUENCY: f32 = 40.0;
	
	pub const SNOW_LEVEL_SPEED_MULTIPLIER: f32    = 1.25;
	pub const AUTUMN_LEVEL_SPEED_MULTIPLIER: f32  = 0.75;

	pub fn new() -> Self {
		PlayerMovementHandler {
			units_travelled_laterally:    0.0,
			distance_travelled_laterally: 0.0,
			jump_delay:                   Self::JUMP_DELAY,
			running_phase_factor:         0,
			can_jump:                     true,
			is_jumping:                   false,
			is_falling:                   false,
		}
	}
}

#[derive(Debug)]
pub struct PlayerEntity {
	pub state:               PlayerState,
	pub direction:           PlayerDirection,
    pub position:            Point2<f32>,
	pub bounding_rect_edges: Option<BoundingRectEdges>,
	pub max_jump_height:     f32,
}

impl PlayerEntity {
	pub const RIGHT_WINDOW_EDGE_IN_REGARDS_TO_PLAYERMODEL: f32 = 974.0;
	pub const LEFT_WINDOW_EDGE_IN_REGARDS_TO_PLAYERMODEL: f32  = 50.0;
	
	const RUN_SPEED: f32          = 750.0;
	pub const JUMP_AMPLITUDE: f32 = 200.0;
	
	const LEFT_HITBOX_EDGE_OFFSET: f32  = 40.0;
	const RIGHT_HITBOX_EDGE_OFFSET: f32 = 40.0;
	const TOP_HITBOX_EDGE_OFFSET: f32   = 12.5;

    pub fn new(position: Point2<f32>) -> Self {
        PlayerEntity {
			state:               PlayerState::Idle,
			direction:           PlayerDirection::Right,
            position,
			bounding_rect_edges: None,
			max_jump_height:     position.y - Self::JUMP_AMPLITUDE,
        }
    }

	fn change_player_position_along_x_axis(&mut self, units_travelled_laterally: f32, seconds: f32) {
		let new_x   = self.position.x + Self::RUN_SPEED * seconds * units_travelled_laterally;
		self.position.x = nalgebra::clamp(new_x, Self::LEFT_WINDOW_EDGE_IN_REGARDS_TO_PLAYERMODEL, Self::RIGHT_WINDOW_EDGE_IN_REGARDS_TO_PLAYERMODEL);
	}

	fn change_player_position_along_y_axis(&mut self) {
		match self.state {
			PlayerState::Jumping => self.position.y -= Physics::GRAVITY,
			PlayerState::Falling => self.position.y += Physics::GRAVITY,
			_ => (),
		}
	}

    pub fn update(&mut self, units_travelled_laterally: f32, seconds: f32) {
        self.change_player_position_along_x_axis(units_travelled_laterally, seconds);
		self.change_player_position_along_y_axis();
    }

	fn update_bounding_rect_edges(&mut self) {
		let left   = self.position.x - Self::LEFT_HITBOX_EDGE_OFFSET;
		let top    = self.position.y - Self::TOP_HITBOX_EDGE_OFFSET;
		let right  = self.position.x + Self::RIGHT_HITBOX_EDGE_OFFSET;
		let bottom = self.position.y;

		self.bounding_rect_edges = Some(BoundingRectEdges::new(left, top, right, bottom));
	}

	fn draw_player(&mut self, ctx: &mut Context, image: &graphics::ImageGeneric<graphics::GlBackendSpec>) {
		self.update_bounding_rect_edges();
		graphics::draw(ctx, image, graphics::DrawParam {
			dest: self.position,
			offset: Point2 { x: 0.5, y: 1.0 },
			.. Default::default()
		}).unwrap();
	}

    pub fn draw(&mut self, ctx: &mut Context, model: &PlayerModel) -> GameResult<()> {
		let image =
			match self.state {
				PlayerState::Idle =>
					match self.direction {
						PlayerDirection::Right => &model.idle_right,
						PlayerDirection::Left  => &model.idle_left,
					},

				PlayerState::RunningPhase0 =>
					match self.direction {
						PlayerDirection::Right => &model.run0_right,
						PlayerDirection::Left  => &model.run0_left,
					},

				PlayerState::RunningPhase1 =>
					match self.direction {
						PlayerDirection::Right => &model.run1_right,
						PlayerDirection::Left  => &model.run1_left,
					},

				PlayerState::RunningPhase2 =>
					match self.direction {
						PlayerDirection::Right => &model.run2_right,
						PlayerDirection::Left  => &model.run2_left,
					},
				
				PlayerState::Jumping =>
					match self.direction {
						PlayerDirection::Right => &model.jump_right,
						PlayerDirection::Left  => &model.jump_left,
					},
	
				PlayerState::Falling =>
					match self.direction {
						PlayerDirection::Right => &model.fall_right,
						PlayerDirection::Left  => &model.fall_left,
					},
				
				PlayerState::Dead =>
					match self.direction {
						PlayerDirection::Right => &model.dead_right,
						PlayerDirection::Left  => &model.dead_left,
					},
			};

		self.draw_player(ctx, image);
		Ok(())
	}
}

impl BoundableEntity for PlayerEntity {
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
