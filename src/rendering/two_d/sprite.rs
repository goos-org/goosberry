use crate::rendering::texture::Texture;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Sprite {
    pub texture: Box<dyn Texture>,
    pub shader_label: String,
    pub shader: String,
}
impl Sprite {
    pub fn new<T: Texture + 'static>(texture: T) -> Self {
        Self {
            texture: Box::new(texture),
            shader_label: "SpriteUnlit".to_string(),
            shader: include_str!("shaders/sprite_unlit.wgsl").to_string(),
        }
    }
    pub fn new_with_shader<T: Texture + 'static>(
        texture: T,
        shader: String,
        label: String,
    ) -> Self {
        Self {
            texture: Box::new(texture),
            shader_label: label,
            shader,
        }
    }
}
