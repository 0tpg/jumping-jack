use ggez::event;
use ggez::graphics;
use ggez::input;
use ggez::input::keyboard;
use ggez::mint::Point2;
use ggez::timer;
use ggez::{Context, GameResult};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::{ops::RangeFrom, ops::RangeInclusive, collections::VecDeque};

use crate::assets::*;
use crate::entities::*;
use crate::player::*;
use crate::platform::*;
use crate::debug;

pub struct MainState {
	pub player:             PlayerEntity,
    pub player_movement:    PlayerMovementHandler,
	pub platforms:          VecDeque<PlatformEntity>,
	pub platforms_movement: PlatformVerticalMovementHandler,
	pub score:              u32,
	pub rng:                ThreadRng,
	pub assets:             Assets,
	pub game_over:          bool,
}

impl MainState {
	pub const STARTING_PLAYER_X: f32 = WindowMeasurements::WINDOW_WIDTH / 2.0;
	pub const STARTING_PLAYER_Y: f32 = WindowMeasurements::WINDOW_HEIGHT - 84.0;
	pub const STARTING_PLAYER_POSITION: Point2<f32> = Point2 {
		x: Self::STARTING_PLAYER_X,
		y: Self::STARTING_PLAYER_Y,
	};

	pub const DEFAULT_LARGE_PLATFORM_POINT: Point2<f32> =
		Point2 {
			x: 0.0,
			y: 0.0,
		};
	pub const STARTING_PLATFORM_Y: f32 = WindowMeasurements::WINDOW_HEIGHT - 84.0;
	pub const STARTING_LARGE_PLATFORM_POINT: Point2<f32> =
		Point2 {
			x: 0.0,
			y: Self::STARTING_PLATFORM_Y,
		};

	pub const GRASS_LEVEL_SCORE_RANGE: RangeInclusive<u32>  = 0 ..= 900;
	pub const DESERT_LEVEL_SCORE_RANGE: RangeInclusive<u32> = 1000 ..= 1900;
	pub const FOREST_LEVEL_SCORE_RANGE: RangeInclusive<u32> = 2000 ..= 2900;
	pub const SNOW_LEVEL_SCORE_RANGE: RangeInclusive<u32>   = 3000 ..= 3900;
	pub const AUTUMN_LEVEL_SCORE_RANGE: RangeFrom<u32>      = 4000 ..;
	
	pub const GRASS_LEVEL_MOBILE_PLATFORMS_RANGE: RangeInclusive<u32>  = 400 ..= 900;
	pub const DESERT_LEVEL_MOBILE_PLATFORMS_RANGE: RangeInclusive<u32> = 1400 ..= 1900;
	pub const FOREST_LEVEL_MOBILE_PLATFORMS_RANGE: RangeInclusive<u32> = 2400 ..= 2900;
	pub const SNOW_LEVEL_MOBILE_PLATFORMS_RANGE: RangeInclusive<u32>   = 3400 ..= 3900;
	pub const AUTUMN_LEVEL_MOBILE_PLATFORMS_RANGE: RangeFrom<u32>      = 4400 ..;
	
	pub const HORIZONTAL_STATIONARY_PLATFORM_GENERATION_RANGE: RangeInclusive<f32> = 25.0 ..= 619.0;
	pub const HORIZONTAL_MOBILE_PLATFORM_GENERATION_RANGE: RangeInclusive<f32>     = 141.0 ..= 597.0;

	pub const DEAD_PLAYER_IMAGE_OFFSET: f32 = 1.5;

    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            player:             PlayerEntity::new(Self::STARTING_PLAYER_POSITION),
            player_movement:    PlayerMovementHandler::new(),
			platforms:          VecDeque::new(),
			platforms_movement: PlatformVerticalMovementHandler::new(),
            score:              0,
			rng:                rand::thread_rng(),
			assets:             Assets::new(ctx)?,
            game_over:          false,
        })
    }

	pub fn handle_collisions(&mut self, ctx: &mut Context) { 
		let platforms_len = self.platforms.len();
		let mut overlap_exists = false;

        for platform in &mut self.platforms {
			if platform.bounding_rect().overlaps(&self.player.bounding_rect()) {
				match self.player.state {
					PlayerState::Jumping => (),
					_ => {
						overlap_exists = true;

						if let PlayerState::Falling = self.player.state {
							if let PlatformSize::Large = platform.size {
								if platform.scoring.score < self.score && platforms_len < 5 {
									match debug::is_active() {
										true => (),
										false => {
											self.game_over = true;
										},
									};
								}
							}

							self.player.state = PlayerState::Idle;
							self.player_movement.jump_delay = PlayerMovementHandler::JUMP_DELAY;
						}

						self.player_movement.is_jumping = false;
						self.player_movement.is_falling = false;

						if self.player_movement.jump_delay <= 0.0 {
							self.player_movement.can_jump = true;
						}

						self.player.max_jump_height = self.player.position.y - PlayerEntity::JUMP_AMPLITUDE;
						platform.interaction.player_is_on_platform = true;

						if !platform.scoring.score_has_been_collected {
							platform.scoring.score_has_been_collected = true;

							self.score = platform.scoring.score;
						}

						match platform.movement.is_mobile {
							false => (),
							true => {
								if platform.movement.can_go_right {
									match platform.design {
										PlatformDesign::GrassDesign  => self.player.position.x += PlatformHorizontalMovementHandler::GRASS_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::DesertDesign => self.player.position.x += PlatformHorizontalMovementHandler::DESERT_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::ForestDesign => self.player.position.x += PlatformHorizontalMovementHandler::FOREST_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::SnowDesign   => self.player.position.x += PlatformHorizontalMovementHandler::SNOW_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::AutumnDesign => self.player.position.x += PlatformHorizontalMovementHandler::AUTUMN_LEVEL_LATERAL_UNITS_TRAVELLED,
									}
								} else if platform.movement.can_go_left {
									match platform.design {
										PlatformDesign::GrassDesign  => self.player.position.x -= PlatformHorizontalMovementHandler::GRASS_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::DesertDesign => self.player.position.x -= PlatformHorizontalMovementHandler::DESERT_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::ForestDesign => self.player.position.x -= PlatformHorizontalMovementHandler::FOREST_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::SnowDesign   => self.player.position.x -= PlatformHorizontalMovementHandler::SNOW_LEVEL_LATERAL_UNITS_TRAVELLED,
										PlatformDesign::AutumnDesign => self.player.position.x -= PlatformHorizontalMovementHandler::AUTUMN_LEVEL_LATERAL_UNITS_TRAVELLED,
									}
								}
							},
						}

						match platform.design {

							PlatformDesign::GrassDesign => {
								match platform.size {
									PlatformSize::Large => (),
									_ =>
										if (PtoPInteractionHandler::GRASS_LEVEL_BROKEN_STATE_TIME_RANGE).contains(&platform.interaction.time_spent_on_platform) {
											platform.state = PlatformState::Broken
										} else if let PlatformState::Broken = platform.state {
											platform.state = PlatformState::Nonexistent
										},
								}
							},

							PlatformDesign::DesertDesign => {
								match platform.size {
									PlatformSize::Large => (),
									_ =>
										if (PtoPInteractionHandler::DESERT_LEVEL_BROKEN_STATE_TIME_RANGE).contains(&platform.interaction.time_spent_on_platform) {
											platform.state = PlatformState::Broken
										} else if let PlatformState::Broken = platform.state {
											platform.state = PlatformState::Nonexistent
										},
								}
							},

							PlatformDesign::ForestDesign => {
								match platform.size {
									PlatformSize::Large => (),
									_ =>
										if (PtoPInteractionHandler::FOREST_LEVEL_BROKEN_STATE_TIME_RANGE).contains(&platform.interaction.time_spent_on_platform) {
											platform.state = PlatformState::Broken
										} else if let PlatformState::Broken = platform.state {
											platform.state = PlatformState::Nonexistent
										},
								}
							},

							PlatformDesign::SnowDesign => {
								match platform.size {
									PlatformSize::Large => (),
									_ =>
										if (PtoPInteractionHandler::SNOW_LEVEL_BROKEN_STATE_TIME_RANGE).contains(&platform.interaction.time_spent_on_platform) {
											platform.state = PlatformState::Broken
										} else if let PlatformState::Broken = platform.state {
											platform.state = PlatformState::Nonexistent
										},
								}
							},

							PlatformDesign::AutumnDesign => {
								match platform.size {
									PlatformSize::Large => (),
									_ =>
										if (PtoPInteractionHandler::AUTUMN_LEVEL_BROKEN_STATE_TIME_RANGE).contains(&platform.interaction.time_spent_on_platform) {
											platform.state = PlatformState::Broken
										} else if let PlatformState::Broken = platform.state {
											platform.state = PlatformState::Nonexistent
										},
								}
							},
						}
					},
				}

				break;
			}
        }

		if !overlap_exists {
			match self.player.state {
				PlayerState::Jumping => (),
				_ => {
					match self.player.position.y < WindowMeasurements::WINDOW_HEIGHT - Self::DEAD_PLAYER_IMAGE_OFFSET {
						true => {
							self.player.state = PlayerState::Falling;
							self.player_movement.is_jumping = false;
							self.player_movement.is_falling = true;

							self.player_movement.can_jump = false;
						},

						false =>
							match debug::is_active() {
								true => {
									self.player.position.y = WindowMeasurements::WINDOW_HEIGHT - Self::DEAD_PLAYER_IMAGE_OFFSET;
									self.player_movement.is_falling = false;
									self.player_movement.can_jump = true;

									if let PlayerState::Falling = self.player.state {
										self.player.state = PlayerState::Idle;
									}
								},

								false => {
									self.player.state = PlayerState::Dead;
									self.player_movement.is_falling = false;
								},
							},
					}
				},
			}
		}
    }

	fn update_player_running_phase(&mut self) {
		let frequency_coefficient =
			if (Self::SNOW_LEVEL_SCORE_RANGE).contains(&self.score) {
				PlayerMovementHandler::SNOW_LEVEL_RUNNING_PHASE_CHANGE_FREQUENCY
			} else if (Self::AUTUMN_LEVEL_SCORE_RANGE).contains(&self.score) {
				PlayerMovementHandler::AUTUMN_LEVEL_RUNNING_PHASE_CHANGE_FREQUENCY
			} else {
				PlayerMovementHandler::DEFAULT_FREQUENCY
			};
			
		self.player_movement.distance_travelled_laterally += self.player_movement.units_travelled_laterally;

		if self.player_movement.distance_travelled_laterally.abs() % frequency_coefficient == 1.0 {
			self.player_movement.running_phase_factor += 1;
		}

		match self.player.state {
			PlayerState::Jumping | PlayerState::Falling | PlayerState::Dead => (),
			_ =>
				match self.player_movement.running_phase_factor {
					0 => self.player.state = PlayerState::Idle,
					1 => self.player.state = PlayerState::RunningPhase0,
					2 => self.player.state = PlayerState::RunningPhase1,
					3 => self.player.state = PlayerState::RunningPhase2,
					_ => self.player_movement.running_phase_factor = 1,
				},
		}
	}

	fn update_player_jumping_motion(&mut self, seconds: f32) {
		self.player_movement.jump_delay -= seconds;
		if self.player_movement.is_jumping {
			if self.player.position.y >= self.player.max_jump_height {
				self.player.state = PlayerState::Jumping;
				self.player.position.y -= Physics::GRAVITY;

			} else {
				self.player.state = PlayerState::Falling;
				self.player_movement.is_jumping = false;
				self.player_movement.is_falling = true;
			}

		} else if self.player_movement.is_falling {
			if self.player.position.y <= WindowMeasurements::WINDOW_HEIGHT {
				self.player.position.y += Physics::GRAVITY;
				
			} else {
				match debug::is_active() {
					true => (),
					false => {
						self.player.state = PlayerState::Dead;
						self.player_movement.is_falling = false;
					},
				};
			}
		}
	}

	fn clear_old_interaction_data(&mut self) {
		for platform in &mut self.platforms {
			if platform.interaction.player_is_on_platform {
				platform.interaction.player_is_on_platform = false;
			}
		}
	}

	fn readjust_platforms_vertically(&mut self, seconds: f32) {
		for platform in &mut self.platforms {
			platform.movement.gravity = Physics::GRAVITY * PlatformVerticalMovementHandler::DEFAULT_GRAVITY_MULTIPLIER;

			platform.update(true);
			self.platforms_movement.platform_update_timer -= seconds;
		}
	} 

	fn update_distance_till_next_platform(&mut self) {
		if self.platforms_movement.distance_till_next_platform <= 0.0 {
			self.platforms_movement.distance_till_next_platform = 0.0 + self.platforms.back().unwrap().position.y - PlatformEntity::VERTICAL_PLATFORM_OFFSET;
		}
	}

	fn reset_platform_update_timer(&mut self) {
		if self.platforms_movement.platform_update_timer <= 0.0 {
			self.platforms_movement.platform_update_timer = PlatformVerticalMovementHandler::PLATFORM_UPDATE_DURATION;
		}
	}

	fn update_time_spent_on_platform(&mut self, seconds: f32) {
		for platform in &mut self.platforms {
			if platform.interaction.player_is_on_platform {
				platform.interaction.time_spent_on_platform += seconds;
			}
		}
	}

	fn generate_new_platform(&mut self) {
		let current_platform_point = 
			match self.platforms_movement.score_of_latest_platform {
				0 => Self::STARTING_LARGE_PLATFORM_POINT,
				1000 | 2000 | 3000 | 4000 => Self::DEFAULT_LARGE_PLATFORM_POINT,
				_ =>
					if (Self::GRASS_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
					|| (Self::DESERT_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
					|| (Self::FOREST_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
					|| (Self::SNOW_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
					|| (Self::AUTUMN_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform) {
						Point2 {
							x: self.rng.gen_range(Self::HORIZONTAL_MOBILE_PLATFORM_GENERATION_RANGE),
							y: self.platforms_movement.distance_till_next_platform,
						}
					} else {
						Point2 {
							x: self.rng.gen_range(Self::HORIZONTAL_STATIONARY_PLATFORM_GENERATION_RANGE),
							y: self.platforms_movement.distance_till_next_platform,
						}
					},
			};
			
		let current_platform_design =
			if (Self::GRASS_LEVEL_SCORE_RANGE).contains(&self.platforms_movement.score_of_latest_platform) {
				PlatformDesign::GrassDesign
			} else if (Self::DESERT_LEVEL_SCORE_RANGE).contains(&self.platforms_movement.score_of_latest_platform) {
				PlatformDesign::DesertDesign
			} else if (Self::FOREST_LEVEL_SCORE_RANGE).contains(&self.platforms_movement.score_of_latest_platform) {
				PlatformDesign::ForestDesign
			} else if (Self::SNOW_LEVEL_SCORE_RANGE).contains(&self.platforms_movement.score_of_latest_platform) {
				PlatformDesign::SnowDesign
			} else {
				PlatformDesign::AutumnDesign
			};

		let current_platform_size = 
			match self.platforms_movement.score_of_latest_platform {
				0 | 1000 | 2000 | 3000 | 4000 => PlatformSize::Large,
				_ => 
					match self.rng.gen_range(0_u32 .. 2_u32) == 0 {
						true  => PlatformSize::Small,
						false => PlatformSize::Regular
					},
			};

		let current_platform_is_moving = 
			if (Self::GRASS_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
			|| (Self::DESERT_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
			|| (Self::FOREST_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
			|| (Self::SNOW_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform)
			|| (Self::AUTUMN_LEVEL_MOBILE_PLATFORMS_RANGE).contains(&self.platforms_movement.score_of_latest_platform) {
				true
			} else {
				false
			};
					
		let current_platform =
			PlatformEntity::new(
				current_platform_design,
				current_platform_size,
				current_platform_point,
				self.platforms_movement.score_of_latest_platform,
				current_platform_is_moving,
			);
					
		self.platforms_movement.score_of_latest_platform += PlatformScoreHandler::POINTS;
		self.platforms_movement.distance_till_next_platform -= PlatformEntity::VERTICAL_PLATFORM_OFFSET;

		self.platforms.push_back(current_platform);
	}

	fn drop_platforms_out_of_reach(&mut self) {
		if self.platforms.front().unwrap().position.y >= WindowMeasurements::WINDOW_HEIGHT {
			self.platforms.pop_front();
		}
	}

	fn drop_broken_platforms(&mut self) {
		let mut index_counter: usize = 0;
		let mut index_to_remove: Option<usize> = None;

		for platform in &self.platforms {
			match platform.state {
				PlatformState::Nonexistent => index_to_remove = Some(index_counter),
				_ => index_counter += 1,
			}
		}
			
		match index_to_remove {
			None => (),
			Some(i) => {
				self.platforms.remove(i);
			},
		}
	}

	fn draw_background(&mut self, ctx: &mut Context) {
		let background =
			if (Self::GRASS_LEVEL_SCORE_RANGE).contains(&self.score) {
				&self.assets.levels.grass_background
			} else if (Self::DESERT_LEVEL_SCORE_RANGE).contains(&self.score) {
				&self.assets.levels.desert_background
			} else if (Self::FOREST_LEVEL_SCORE_RANGE).contains(&self.score) {
				&self.assets.levels.forest_background
			} else if (Self::SNOW_LEVEL_SCORE_RANGE).contains(&self.score) {
				&self.assets.levels.snow_background
			} else {
				&self.assets.levels.autumn_background
			};

        graphics::draw(ctx, background, graphics::DrawParam {
			.. Default::default()
		}).unwrap();
	}

	fn draw_black_rectangle(&mut self, ctx: &mut Context, top_left: &Point2<f32>) {
		let left: f32   = top_left.x - 20.0;
		let top: f32    = top_left.y - 2.0;
		let right: f32  = top_left.x - 20.0;
		let bottom: f32 = top_left.y - 130.0;
		let rect   = graphics::Rect::new(left, top, right, bottom);

		let rect_mesh = graphics::MeshBuilder::new().
								rectangle(graphics::DrawMode::fill(), rect, graphics::BLACK).
								build(ctx).
								unwrap();

		graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default()).unwrap();
	}

	fn draw_game_over(&mut self, ctx: &mut Context) {
		let window_width  = WindowMeasurements::WINDOW_WIDTH;
		let window_height = WindowMeasurements::WINDOW_HEIGHT;

		let text_font            = graphics::Font::new(ctx, "/Kenney Pixel.ttf").unwrap();
		let text_contents      = format!("Game Over\nScore: {}", self.score);
		let text_fragment = graphics::TextFragment::from((text_contents, text_font, 60.0));
        let text                = graphics::Text::new(text_fragment);
			
        let top_left = Point2 {
			x: (window_width - text.width(ctx) as f32) / 2.0,
            y: (window_height - text.height(ctx) as f32) / 3.0,
        };

		self.draw_black_rectangle(ctx, &top_left);
        graphics::draw(ctx, &text, graphics::DrawParam {
            dest: top_left,
            .. Default::default()
        }).unwrap();
	}
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		if self.game_over {
            return Ok(());
        }

        const DESIRED_FPS: u32 = 300;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            let seconds = 1.0 / (DESIRED_FPS as f32);

			if let PlayerState::Dead = self.player.state {
				match debug::is_active() {
					true => (),
					false => {
						self.game_over = true;
					},
				};
			}
			
			if (Self::SNOW_LEVEL_SCORE_RANGE).contains(&self.score) {
				self.player.update(self.player_movement.units_travelled_laterally * PlayerMovementHandler::SNOW_LEVEL_SPEED_MULTIPLIER, seconds);
			} else if (Self::AUTUMN_LEVEL_SCORE_RANGE).contains(&self.score) {
				self.player.update(self.player_movement.units_travelled_laterally * PlayerMovementHandler::AUTUMN_LEVEL_SPEED_MULTIPLIER, seconds);
			} else {
				self.player.update(self.player_movement.units_travelled_laterally, seconds);
			}

			self.update_player_running_phase();
			self.update_player_jumping_motion(seconds);

			for platform in &mut self.platforms {
				platform.update(false);
			}

			if let PlayerState::Jumping = self.player.state {
				self.clear_old_interaction_data();

				if self.platforms_movement.platform_update_timer > 0.0 && self.player.position.y < PlatformVerticalMovementHandler::MAX_PLAYER_HEIGHT_FOR_PLATFORM_UPDATE {
					self.readjust_platforms_vertically(seconds);
					self.update_distance_till_next_platform();
					self.reset_platform_update_timer();
				}

			} else {
				if let PlayerState::Falling = self.player.state {
					self.clear_old_interaction_data();
				}

				self.update_time_spent_on_platform(seconds);
			}
		
			if self.platforms_movement.distance_till_next_platform >= 0.0 {
				self.generate_new_platform();
			}

			self.handle_collisions(ctx);

			self.drop_platforms_out_of_reach();
			self.drop_broken_platforms();
        }

        Ok(())
    }

    fn key_down_event(&mut self,
                      ctx: &mut Context,
                      keycode: event::KeyCode,
                      _keymod: input::keyboard::KeyMods,
                      _repeat: bool) {

        match keycode {
            event::KeyCode::Space => {
				if !keyboard::is_key_repeated(ctx) {
					match self.player_movement.can_jump {
						true => {
							self.player_movement.is_jumping = true;
						},
						
						false => (),
					};
				}
			},

            event::KeyCode::Left | event::KeyCode::A => {
				match self.game_over {
					true => (),
					false => {
						self.player_movement.units_travelled_laterally = -PlayerMovementHandler::PLAYER_UNITS_TRAVELLED;
						self.player.direction = PlayerDirection::Left;
					},
				}
			},

            event::KeyCode::Right | event::KeyCode::D => {
				match self.game_over {
					true => (),
					false => {
						self.player_movement.units_travelled_laterally = PlayerMovementHandler::PLAYER_UNITS_TRAVELLED;
						self.player.direction = PlayerDirection::Right;
					},
				}
			},

            event::KeyCode::Escape => event::quit(ctx),

            _ => (),
        }
    }

    fn key_up_event(&mut self,
                    _ctx: &mut Context,
                    keycode: event::KeyCode,
                    _keymod: input::keyboard::KeyMods) {

        match keycode {
            event::KeyCode::Space => self.player_movement.can_jump = false,

            event::KeyCode::Left | event::KeyCode::A => {
				match self.game_over {
					true => (),
					false => {
						self.player_movement.units_travelled_laterally = 0.0;

						self.player_movement.running_phase_factor = 0;
						self.player_movement.distance_travelled_laterally = 0.0;
					},
				}
			},

			event::KeyCode::Right | event::KeyCode::D => {
				match self.game_over {
					true => (),
					false => {
						self.player_movement.units_travelled_laterally = 0.0;
				
						self.player_movement.running_phase_factor = 0;
						self.player_movement.distance_travelled_laterally = 0.0;
					},
				}
			},

            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		self.draw_background(ctx);

		for platform in &mut self.platforms {
			platform.draw(ctx, &self.assets.levels)?;
		}

        self.player.draw(ctx, &self.assets.player)?;

		if debug::is_active() {
			debug::draw_outline(self.player.bounding_rect(), ctx).unwrap();

            for platform in &mut self.platforms {
                debug::draw_outline(platform.bounding_rect(), ctx).unwrap();
            }
        }

		if self.game_over {
			self.draw_game_over(ctx);
			
			graphics::present(ctx)?;
			return Ok(())
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
