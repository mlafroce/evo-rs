use crate::scene::entity::Entity;
use crate::scene::texture_manager::{TextureId, TextureManager};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::sys::rand;
use sdl2::video::WindowContext;

pub struct Scenario<'a> {
    texture_manager: TextureManager<'a>,
    entities: Vec<Entity>,
}

impl<'a> Scenario<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let texture_manager = TextureManager::new(texture_creator).unwrap();
        let entities = vec![];
        Self {
            entities,
            texture_manager,
        }
    }

    pub fn add_entity(&mut self) {
        let x = unsafe { rand() } as i32 % 800;
        let y = unsafe { rand() } as i32 % 600;
        let entity = Entity::new(x, y, TextureId::Dot);
        self.entities.push(entity)
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for entity in &self.entities {
            entity.render(canvas, &self.texture_manager);
        }
    }
}
