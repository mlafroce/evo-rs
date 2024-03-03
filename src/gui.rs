use imgui::{Context, DrawData};

const FPS_INTERVAL: u32 = 500;

#[derive(Default)]
pub struct Gui {
    last_fps: f32,
    last_fps_tick: u32,
    tick_counter: u32,
}

impl Gui {
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

        /* render */
        imgui.render()
    }
}
