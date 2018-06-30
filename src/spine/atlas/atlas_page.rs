use super::TextureFilter;
use super::TextureWrap;
use super::Format;
use super::AtlasRegion;

pub struct AtlasPage {
    pub name: String,
    pub size: (i32, i32),
    pub format: Format,
    pub filter: TextureFilter,
    pub repeat: TextureWrap,
    pub regions: Vec<AtlasRegion>
}
