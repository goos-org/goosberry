use image::{ImageBuffer, Rgba};
use nalgebra::Vector2;

pub trait Texture {
    fn sample(&self, uv: Vector2<f32>) -> &Rgba<f32>;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn complete(&self) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
        let mut image = ImageBuffer::new(self.width(), self.height());
        for x in 0..self.width() {
            for y in 0..self.height() {
                image.put_pixel(x, y, *self.sample(Vector2::new(x as f32, y as f32)));
            }
        }
        image
    }
}
impl Texture for ImageBuffer<Rgba<f32>, Vec<f32>> {
    fn sample(&self, uv: Vector2<f32>) -> &Rgba<f32> {
        self.get_pixel(uv.x as u32, uv.y as u32)
    }
    fn width(&self) -> u32 {
        self.width()
    }
    fn height(&self) -> u32 {
        self.height()
    }
    fn complete(&self) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
        self.clone()
    }
}

pub struct Sprite {
    pub texture: Box<dyn Texture>,
}

pub struct Tilemap {
    pub textures: Vec<Box<dyn Texture>>,
    pub tile_size: Vector2<u32>,
    tilemap: Vec<Vec<usize>>,
}
impl Tilemap {
    pub fn new(tile_size: Vector2<u32>) -> Self {
        Tilemap {
            textures: Vec::new(),
            tile_size,
            tilemap: Vec::new(),
        }
    }
    pub fn add_texture<T: 'static + Texture>(&mut self, texture: T) {
        self.textures.push(Box::new(texture));
    }
}
impl Texture for Tilemap {
    fn sample(&self, uv: Vector2<f32>) -> &Rgba<f32> {
        let tile_x = uv.x as usize / self.tile_size.x as usize;
        let tile_y = uv.y as usize / self.tile_size.y as usize;
        let tile_id = self.tilemap[tile_y][tile_x];
        self.textures[tile_id].sample(
            uv - Vector2::new(
                tile_x as f32 * self.tile_size.x as f32,
                tile_x as f32 * self.tile_size.x as f32,
            ),
        )
    }
    fn width(&self) -> u32 {
        self.tilemap.get(0).map_or(0, |v| v.len()) as u32 * self.tile_size.x
    }
    fn height(&self) -> u32 {
        self.tilemap.len() as u32 * self.tile_size.y
    }
    fn complete(&self) -> ImageBuffer<Rgba<f32>, Vec<f32>> {
        let mut vec = Vec::new();
        for x in 0..self.width() / self.tile_size.x {
            for y in 0..self.height() / self.tile_size.y {
                let tile_id = self.tilemap[y as usize][x as usize];
                vec.push(self.textures[tile_id].complete().into_raw());
            }
        }
        let mut final_vec = Vec::new();
        for y in 0..self.height() {
            for buf in vec.iter() {
                final_vec.extend_from_slice(
                    &buf[(y * self.width()) as usize
                        ..(y * self.width() + self.width() - 1) as usize],
                );
            }
        }
        ImageBuffer::from_raw(self.width(), self.height(), final_vec).unwrap()
    }
}
