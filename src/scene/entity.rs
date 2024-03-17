use crate::scene::texture_manager::{TextureId, TextureManager};
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct Entity {
    id: u32,
    coords: (i32, i32),
    texture_id: TextureId,
    radius: i32,
    selected: bool,
}

impl Entity {
    pub fn new(id: u32, x: i32, y: i32, texture_id: TextureId) -> Self {
        Self {
            id,
            coords: (x, y),
            texture_id,
            radius: 16,
            selected: false,
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas, texture_manager: &TextureManager) {
        let texture = texture_manager.get_texture(self.texture_id);
        let diameter = self.radius as u32 * 2;
        canvas
            .copy(
                texture.get_texture(),
                Rect::new(0, 0, texture.width(), texture.height()),
                Rect::new(
                    self.coords.0 - self.radius,
                    self.coords.1 - self.radius,
                    diameter,
                    diameter,
                ),
            )
            .unwrap();
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        let distance = ((self.coords.0 - x) * (self.coords.0 - x))
            + ((self.coords.1 - y) * (self.coords.1 - y));
        distance < (self.radius * self.radius)
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
        if selected {
            self.texture_id = TextureId::DotSelected
        } else {
            self.texture_id = TextureId::Dot
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_coords(&self) -> (i32, i32) {
        self.coords
    }
}
