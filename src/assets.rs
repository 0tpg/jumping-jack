use ggez::audio;
use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets {
	pub player:    PlayerModel,
	pub levels:    LevelModels,
	pub announcer: AnnouncerSounds,
}

impl Assets {
	pub fn new(ctx: &mut Context) -> GameResult<Assets> {
		let player 		 = PlayerModel::new(ctx)?;
		let levels 		 = LevelModels::new(ctx)?;
		let announcer = AnnouncerSounds::new(ctx)?;

		Ok(Assets {
			player,
			levels,
			announcer,
		})
	}
}

pub struct PlayerModel {
	pub idle_right: graphics::Image,
	pub idle_left:  graphics::Image,

	pub run0_right: graphics::Image,
	pub run0_left:  graphics::Image,

	pub run1_right: graphics::Image,
	pub run1_left:  graphics::Image,

	pub run2_right: graphics::Image,
	pub run2_left:  graphics::Image,

	pub jump_right: graphics::Image,
	pub jump_left:  graphics::Image,

	pub fall_right: graphics::Image,
	pub fall_left:  graphics::Image,

	pub dead_right: graphics::Image,
	pub dead_left:  graphics::Image,
}

impl PlayerModel {
	pub fn new(ctx: &mut Context) -> GameResult<PlayerModel> {
		let idle_right = graphics::Image::new(ctx, "/character_malePerson_idle_right.png")?;
		let idle_left  = graphics::Image::new(ctx, "/character_malePerson_idle_left.png")?;

		let run0_right = graphics::Image::new(ctx, "/character_malePerson_run0_right.png")?;
		let run0_left  = graphics::Image::new(ctx, "/character_malePerson_run0_left.png")?;

		let run1_right = graphics::Image::new(ctx, "/character_malePerson_run1_right.png")?;
		let run1_left  = graphics::Image::new(ctx, "/character_malePerson_run1_left.png")?;

		let run2_right = graphics::Image::new(ctx, "/character_malePerson_run2_right.png")?;
		let run2_left  = graphics::Image::new(ctx, "/character_malePerson_run2_left.png")?;

		let jump_right = graphics::Image::new(ctx, "/character_malePerson_jump_right.png")?;
		let jump_left  = graphics::Image::new(ctx, "/character_malePerson_jump_left.png")?;

		let fall_right = graphics::Image::new(ctx, "/character_malePerson_fall_right.png")?;
		let fall_left  = graphics::Image::new(ctx, "/character_malePerson_fall_left.png")?;

		let dead_right = graphics::Image::new(ctx, "/character_malePerson_fallDown_right.png")?;
		let dead_left  = graphics::Image::new(ctx, "/character_malePerson_fallDown_left.png")?;

		Ok(PlayerModel {
			idle_right,
			idle_left,

			run0_right,
			run0_left,

			run1_right,
			run1_left,

			run2_right,
			run2_left,

			jump_right,
			jump_left,

			fall_right,
			fall_left,

			dead_right,
			dead_left,
		})
	}
}

pub struct LevelModels {
	pub grass_background:	  		   graphics::Image,
	pub grass_small_platform: 	  	   graphics::Image,
	pub grass_small_broken_platform:   graphics::Image,
	pub grass_regular_platform: 	   graphics::Image,
	pub grass_regular_broken_platform: graphics::Image,
	pub grass_large_platform: 		   graphics::Image,
	pub grass_platform_impact_sound:   audio::Source,
	pub grass_platform_break_sound:    audio::Source,
	
	pub desert_background:	  		 	graphics::Image,
	pub desert_small_platform: 	  	 	graphics::Image,
	pub desert_small_broken_platform:   graphics::Image,
	pub desert_regular_platform: 	   	graphics::Image,
	pub desert_regular_broken_platform: graphics::Image,
	pub desert_large_platform: 		 	graphics::Image,
	pub desert_platform_impact_sound:   audio::Source,
	pub desert_platform_break_sound:    audio::Source,
	
	pub forest_background:	  		   	graphics::Image,
	pub forest_small_platform: 	  	   	graphics::Image,
	pub forest_small_broken_platform:  	graphics::Image,
	pub forest_regular_platform: 	   	graphics::Image,
	pub forest_regular_broken_platform: graphics::Image,
	pub forest_large_platform: 		    graphics::Image,
	pub forest_platform_impact_sound:   audio::Source,
	pub forest_platform_break_sound:    audio::Source,
	
	pub autumn_background:	 		     graphics::Image,
	pub autumn_small_platform: 	  	     graphics::Image,
	pub autumn_small_broken_platform:    graphics::Image,
	pub autumn_regular_platform: 	     graphics::Image,
	pub autumn_regular_broken_platform:  graphics::Image,
	pub autumn_large_platform: 		     graphics::Image,
	pub autumn_platform_impact_sound:    audio::Source,
	pub autumn_platform_break_sound:     audio::Source,
	
	pub snow_background:	  		  graphics::Image,
	pub snow_small_platform: 	  	  graphics::Image,
	pub snow_small_broken_platform:   graphics::Image,
	pub snow_regular_platform: 	   	  graphics::Image,
	pub snow_regular_broken_platform: graphics::Image,
	pub snow_large_platform: 		  graphics::Image,
	pub snow_platform_impact_sound:   audio::Source,
	pub snow_platform_break_sound:    audio::Source,

	pub transparent_small_platform:   graphics::Image,
	pub transparent_regular_platform: graphics::Image,
}

impl LevelModels {
	pub fn new(ctx: &mut Context) -> GameResult<LevelModels> {
		let grass_background              = graphics::Image::new(ctx, "/backgroundColorGrass.png")?;
		let grass_small_platform          = graphics::Image::new(ctx, "/ground_grass_small.png")?;
		let grass_small_broken_platform   = graphics::Image::new(ctx, "/ground_grass_small_broken.png")?;
		let grass_regular_platform        = graphics::Image::new(ctx, "/ground_grass.png")?;
		let grass_regular_broken_platform = graphics::Image::new(ctx, "/ground_grass_broken.png")?;
		let grass_large_platform          = graphics::Image::new(ctx, "/ground_grass_elongated.png")?;
		let grass_platform_impact_sound                    = audio::Source::new(ctx, "/grass_platform.ogg")?;
		let grass_platform_break_sound                     = audio::Source::new(ctx, "/grass_platform_break.ogg")?;
		
		let desert_background 			   = graphics::Image::new(ctx, "/backgroundColorDesert.png")?;
		let desert_small_platform 		   = graphics::Image::new(ctx, "/ground_sand_small.png")?;
		let desert_small_broken_platform   = graphics::Image::new(ctx, "/ground_sand_small_broken.png")?;
		let desert_regular_platform 	   = graphics::Image::new(ctx, "/ground_sand.png")?;
		let desert_regular_broken_platform = graphics::Image::new(ctx, "/ground_sand_broken.png")?;
		let desert_large_platform 		   = graphics::Image::new(ctx, "/ground_sand_elongated.png")?;
		let desert_platform_impact_sound                    = audio::Source::new(ctx, "/desert_platform.ogg")?;
		let desert_platform_break_sound                     = audio::Source::new(ctx, "/desert_platform_break.ogg")?;

		let forest_background 		       = graphics::Image::new(ctx, "/backgroundColorForest.png")?;
		let forest_small_platform          = graphics::Image::new(ctx, "/ground_wood_small.png")?;
		let forest_small_broken_platform   = graphics::Image::new(ctx, "/ground_wood_small_broken.png")?;
		let forest_regular_platform        = graphics::Image::new(ctx, "/ground_wood.png")?;
		let forest_regular_broken_platform = graphics::Image::new(ctx, "/ground_wood_broken.png")?;
		let forest_large_platform          = graphics::Image::new(ctx, "/ground_wood_elongated.png")?;
		let forest_platform_impact_sound                    = audio::Source::new(ctx, "/forest_platform.ogg")?;
		let forest_platform_break_sound                     = audio::Source::new(ctx, "/forest_platform_break.ogg")?;

		let autumn_background 		       = graphics::Image::new(ctx, "/backgroundColorFall.png")?;
		let autumn_small_platform          = graphics::Image::new(ctx, "/ground_cake_small.png")?;
		let autumn_small_broken_platform   = graphics::Image::new(ctx, "/ground_cake_small_broken.png")?;
		let autumn_regular_platform        = graphics::Image::new(ctx, "/ground_cake.png")?;
		let autumn_regular_broken_platform = graphics::Image::new(ctx, "/ground_cake_broken.png")?;
		let autumn_large_platform          = graphics::Image::new(ctx, "/ground_cake_elongated.png")?;
		let autumn_platform_impact_sound                    = audio::Source::new(ctx, "/autumn_platform.ogg")?;
		let autumn_platform_break_sound                     = audio::Source::new(ctx, "/autumn_platform_break.ogg")?;

		let snow_background 		     = graphics::Image::new(ctx, "/backgroundColorCastles.png")?;
		let snow_small_platform 	     = graphics::Image::new(ctx, "/ground_snow_small.png")?;
		let snow_small_broken_platform   = graphics::Image::new(ctx, "/ground_snow_small_broken.png")?;
		let snow_regular_platform        = graphics::Image::new(ctx, "/ground_snow.png")?;
		let snow_regular_broken_platform = graphics::Image::new(ctx, "/ground_snow_broken.png")?;
		let snow_large_platform 	     = graphics::Image::new(ctx, "/ground_snow_elongated.png")?;
		let snow_platform_impact_sound                    = audio::Source::new(ctx, "/castles_platform.ogg")?;
		let snow_platform_break_sound                     = audio::Source::new(ctx, "/castles_platform_break.ogg")?;

		let transparent_small_platform   = graphics::Image::new(ctx, "/ground_transparent_small.png")?;
		let transparent_regular_platform = graphics::Image::new(ctx, "/ground_transparent.png")?;

		Ok(LevelModels {
			grass_background,
			grass_small_platform,
			grass_small_broken_platform,
			grass_regular_platform,
			grass_regular_broken_platform,
			grass_large_platform,
			grass_platform_impact_sound,
			grass_platform_break_sound,
						
			desert_background,
			desert_small_platform,
			desert_small_broken_platform,
			desert_regular_platform,
			desert_regular_broken_platform,
			desert_large_platform,
			desert_platform_impact_sound,
			desert_platform_break_sound,
						
			forest_background,
			forest_small_platform,
			forest_small_broken_platform,
			forest_regular_platform,
			forest_regular_broken_platform,
			forest_large_platform,
			forest_platform_impact_sound,
			forest_platform_break_sound,
					
			autumn_background,
			autumn_small_platform,
			autumn_small_broken_platform,
			autumn_regular_platform,
			autumn_regular_broken_platform,
			autumn_large_platform,
			autumn_platform_impact_sound,
			autumn_platform_break_sound,
						
			snow_background,
			snow_small_platform,
			snow_small_broken_platform,
			snow_regular_platform,
			snow_regular_broken_platform,
			snow_large_platform,
			snow_platform_impact_sound,
			snow_platform_break_sound,

			transparent_small_platform,
			transparent_regular_platform,
		})
	}
}

pub struct AnnouncerSounds {
	pub music: 		   audio::Source,
	pub game_over: 	   audio::Source,
}

impl AnnouncerSounds {
	pub fn new(ctx: &mut Context) -> GameResult<AnnouncerSounds> {
		let music		  = audio::Source::new(ctx, "/music.wav")?;
		let game_over 	  = audio::Source::new(ctx, "/game_over.ogg")?;

		Ok(AnnouncerSounds {
			music,
			game_over,
		})
	}
}
