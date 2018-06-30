use super::AtlasPage;

pub struct AtlasRegion {
	pub page: AtlasPage,
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub offset_x: i32,
	pub offset_y: i32,
	pub original_width: i32,
	pub original_height: i32,
	pub index: i32,
	pub rotate: bool,
	pub splits: Vec<i32>,
	pub pads: Vec<i32>
}
