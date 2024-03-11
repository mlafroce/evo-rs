use crate::gui::Gui;
use crate::scene::scenario::Scenario;
use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::video::{GLProfile, Window};
use sdl2::Sdl;

const WINDOWS_HEIGHT: u32 = 720;
const WINDOWS_WIDTH: u32 = 1280;
const TICK_LENGTH: u32 = 16;

fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

pub struct MainWindow {
    canvas: WindowCanvas,
    sdl: Sdl,
}

impl MainWindow {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        /* hint SDL to initialize an OpenGL 3.3 core profile context */
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);

        /* create a new window, be sure to call opengl method on the builder when using glow! */
        let window = video_subsystem
            .window("Hello imgui-rs!", WINDOWS_WIDTH, WINDOWS_HEIGHT)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context()?;
        window.gl_make_current(&gl_context)?;

        /* enable vsync to cap framerate */
        window.subsystem().gl_set_swap_interval(1)?;
        /* create new glow and imgui contexts */
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Self { canvas, sdl })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let gl = glow_context(self.canvas.window());
        let mut imgui = Context::create();

        /* disable creation of files on disc */
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        /* setup platform and renderer, and fonts to imgui */
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let mut platform = SdlPlatform::init(&mut imgui);
        let mut renderer = AutoRenderer::initialize(gl, &mut imgui).map_err(|e| e.to_string())?;

        let mut timer = self.sdl.timer().unwrap();
        let mut last_tick = timer.ticks();
        let mut event_pump = self.sdl.event_pump().unwrap();

        let mut gui = Gui::default();

        let texture_creator = self.canvas.texture_creator();
        let mut scenario = Scenario::new(&texture_creator);

        'main: loop {
            let mut now = timer.ticks();
            if now - last_tick < TICK_LENGTH {
                timer.delay(TICK_LENGTH - (now - last_tick));
                now = timer.ticks();
            }
            last_tick = now;

            for event in event_pump.poll_iter() {
                /* pass all events to imgui platfrom */
                platform.handle_event(&mut imgui, &event);

                match event {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown {
                        keycode: Some(Keycode::A),
                        ..
                    } => {
                        println!("Adding entity");
                        scenario.add_entity();
                    }
                    _ => {}
                }
            }

            // handle gui events
            unsafe { renderer.gl_context().clear_color(0.4, 0.4, 0.4, 1.0) };
            unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };

            scenario.render(&mut self.canvas);

            // TODO: sdl2 0.34.5 doesn't have flush function yet
            let raw_context = self.canvas.raw();
            unsafe { sdl2::sys::SDL_RenderFlush(raw_context) };

            platform.prepare_frame(&mut imgui, self.canvas.window(), &event_pump);
            gui.tick(timer.ticks());
            let draw_data = gui.render(&mut imgui);
            renderer.render(draw_data).unwrap();

            self.canvas.present();
        }
        Ok(())
    }
}
