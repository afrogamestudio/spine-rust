use super::TextureFilter;
use super::TextureWrap;
use super::Format;

pub struct AtlasPage {
    pub name: String,
    pub format: Format,
    pub min_filter: TextureFilter,
    pub mag_filter: TextureFilter,
    pub u_wrap: TextureWrap,
    pub v_wrap: TextureWrap,
    pub width: i32,
    pub height: i32
}