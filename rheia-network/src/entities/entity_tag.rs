use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityTag {
    content: String,
    offset: f32,
    font_size: i32,
    outline_size: i32,
}

impl EntityTag {
    pub fn create(content: String, offset: f32, font_size: i32, outline_size: i32) -> Self {
        Self { content, offset, font_size, outline_size }
    }

    pub fn get_offset(&self) -> &f32 {
        &self.offset
    }

    pub fn get_outline_size(&self) -> &i32 {
        &self.outline_size
    }

    pub fn get_font_size(&self) -> &i32 {
        &self.font_size
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }
}
