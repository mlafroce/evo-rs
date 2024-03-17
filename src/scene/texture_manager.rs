use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum TextureId {
    Dot,
    DotSelected,
}

const TEXTURES: [(TextureId, &str); 2] = [
    (TextureId::Dot, "asset/texture/dot.png"),
    (TextureId::DotSelected, "asset/texture/dot-selected.png"),
];

pub struct TextureManager<'a> {
    _texture_creator: &'a TextureCreator<WindowContext>,
    texture_map: HashMap<TextureId, EvoTexture<'a>>,
}

impl<'a> TextureManager<'a> {
    pub fn new(_texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let mut texture_map = HashMap::new();
        for (texture_id, path) in TEXTURES {
            let texture = EvoTexture::create(path, _texture_creator)?;
            texture_map.insert(texture_id, texture);
        }
        Ok(Self {
            _texture_creator,
            texture_map,
        })
    }

    pub fn get_texture(&self, id: TextureId) -> &EvoTexture {
        self.texture_map.get(&id).unwrap()
    }
}

pub struct EvoTexture<'a> {
    texture: Texture<'a>,
    width: u32,
    height: u32,
}

impl<'a> EvoTexture<'a> {
    pub fn create(
        path: &str,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Self, String> {
        let texture = texture_creator.load_texture(path)?;
        let query = texture.query();
        Ok(Self {
            texture,
            height: query.height,
            width: query.width,
        })
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}
