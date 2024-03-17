use crate::scene::entity::Entity;
use crate::scene::texture_manager::{TextureId, TextureManager};
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::sys::rand;
use sdl2::video::WindowContext;
use std::num::NonZeroU32;

pub struct Scenario<'a> {
    cur_id: u32,
    texture_manager: TextureManager<'a>,
    entities: Vec<Entity>,
    selected_id: Option<NonZeroU32>,
}

impl<'a> Scenario<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Self {
        let texture_manager = TextureManager::new(texture_creator).unwrap();
        let entities = vec![];
        Self {
            cur_id: 1,
            entities,
            texture_manager,
            selected_id: None,
        }
    }

    pub fn add_entity(&mut self) {
        let id = self.cur_id;
        self.cur_id += 1;
        let x = unsafe { rand() } as i32 % 800;
        let y = unsafe { rand() } as i32 % 600;
        let entity = Entity::new(id, x, y, TextureId::Dot);
        self.entities.push(entity)
    }

    pub fn click_entity(&mut self, x: i32, y: i32) {
        self.entities.iter_mut().for_each(|e| e.set_selected(false));
        let clicked = self
            .entities
            .iter_mut()
            .rfind(|entity| entity.contains(x, y));
        if let Some(entity) = clicked {
            entity.set_selected(true);
            self.selected_id = NonZeroU32::new(entity.get_id())
        } else {
            self.selected_id = None
        }
    }

    pub fn render(&self, canvas: &mut WindowCanvas) {
        for entity in &self.entities {
            entity.render(canvas, &self.texture_manager);
        }
    }

    pub fn get_entity(&self, id: u32) -> Option<&Entity> {
        self.entities.iter().find(|e| e.get_id() == id)
    }
    pub fn get_selection(&self) -> Option<NonZeroU32> {
        self.selected_id
    }
}
