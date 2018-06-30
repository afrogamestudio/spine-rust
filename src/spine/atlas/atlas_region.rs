#[derive(Clone)]
pub struct AtlasRegion {
	pub name: String,
    pub rotate: bool,
    pub xy: (i32, i32),
    pub size: (i32, i32),
    pub split: (i32, i32, i32, i32),
    pub pad: (i32, i32, i32, i32),
    pub orig: (i32, i32),
    pub offset: (i32, i32),
    pub index: Option<i32>
}
