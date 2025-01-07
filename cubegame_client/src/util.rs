pub use std::f32::consts::PI;

pub fn to_rads(degrees: f32) -> f32 {
	degrees * PI / 180.0
}