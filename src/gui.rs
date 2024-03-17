use crate::scene::scenario::Scenario;
use imgui::{Context, DrawData};
use std::num::NonZeroU32;

const FPS_INTERVAL: u32 = 500;

#[derive(Default)]
struct SelectionInfo {
    x: i32,
    y: i32,
    id: Option<NonZeroU32>,
}

#[derive(Default)]
pub struct Gui {
    last_fps: f32,
    last_fps_tick: u32,
    tick_counter: u32,
    selected_info: SelectionInfo,
}

impl Gui {
    pub(crate) fn update(&mut self, scenario: &Scenario) {
        if let Some(entity) = scenario.get_selection()
            .and_then(|id| scenario.get_entity(id.into()))
        {
            self.selected_info.x = entity.get_coords().0;
            self.selected_info.y = entity.get_coords().1;
            self.selected_info.id = NonZeroU32::new(entity.get_id());
        } else {
            self.selected_info.id = None;
        };
    }

    pub fn tick(&mut self, timer_ticks: u32) {
        if timer_ticks - self.last_fps_tick > FPS_INTERVAL {
            self.last_fps =
                self.tick_counter as f32 / (timer_ticks - self.last_fps_tick) as f32 * 1000f32;
            self.last_fps_tick = timer_ticks;
            self.tick_counter = 0;
        }
        self.tick_counter += 1;
    }

    pub fn render<'a>(&'a self, imgui: &'a mut Context) -> &'a DrawData {
        /* call prepare_frame before calling imgui.new_frame() */

        let ui = imgui.new_frame();
        /* create imgui UI here */
        ui.text(format!("FPS: {:.2}", self.last_fps));
        let info_window = ui.window("Entity info");
        if let Some(token) = info_window.begin() {
            if let Some(id) = self.selected_info.id {
                ui.text(format!("Selected entity: {}", id));
                ui.text(format!("X: {}", self.selected_info.x));
                ui.text(format!("Y: {}", self.selected_info.y));
                token.end();
            } else {
                ui.text("No entities selected");
            }
        }

        /* render */
        imgui.render()
    }
}
