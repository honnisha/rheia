use std::collections::HashMap;


pub struct TextureMapper {
    textures_map: HashMap<String, u16>,
}

impl TextureMapper {
    pub fn new() -> TextureMapper {
        TextureMapper {}
    }
}
