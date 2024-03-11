use crate::scene::texture_manager::{TextureId, TextureManager};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Entity {
    x: i32,
    y: i32,
    texture_id: TextureId,
}

impl Entity {
    pub fn new(x: i32, y: i32, texture_id: TextureId) -> Self {
        Self { x, y, texture_id }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture_manager: &TextureManager) {
        let img = texture_manager.get_texture(self.texture_id);
        canvas
            .copy(
                img,
                Rect::new(0, 0, 20, 20),
                Rect::new(self.x, self.y, 40, 40),
            )
            .unwrap();
    }
}
