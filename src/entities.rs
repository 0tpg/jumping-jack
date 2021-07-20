use ggez::graphics;

pub struct Physics;

impl Physics {
	pub const GRAVITY: f32 = 2.05;
}

pub struct WindowMeasurements;

impl WindowMeasurements {
	pub const WINDOW_WIDTH: f32  = 1024.0;
	pub const WINDOW_HEIGHT: f32 = 924.0;
}

#[derive(Debug, Copy, Clone)]
pub struct BoundingRectEdges {
	pub left: f32,
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
}

impl BoundingRectEdges {
	pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
		BoundingRectEdges {
			left,
			top,
			right,
			bottom,
		}
	}
}

pub trait BoundableEntity {
	fn bounding_rect(&self) -> graphics::Rect;
}
