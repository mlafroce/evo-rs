use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum TextureId {
    Dot,
}

const TEXTURES: [(TextureId, &str); 1] = [(TextureId::Dot, "asset/texture/dot.bmp")];

pub struct TextureManager<'a> {
    _texture_creator: &'a TextureCreator<WindowContext>,
    texture_map: HashMap<TextureId, Texture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(_texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let mut texture_map = HashMap::new();
        for (texture_id, path) in TEXTURES {
            let texture = _texture_creator.load_texture(path)?;
            texture_map.insert(texture_id, texture);
        }
        Ok(Self {
            _texture_creator,
            texture_map,
        })
    }

    pub fn get_texture(&self, id: TextureId) -> &Texture {
        self.texture_map.get(&id).unwrap()
    }
}
