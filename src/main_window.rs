use crate::gui::Gui;
use glow::HasContext;
use imgui::Context;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use sdl2::event::Event;
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
    sdl: Sdl,
    window: Window,
}

impl MainWindow {
    pub fn new() -> Self {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

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
        /* create a new OpenGL context and make it current */
        Self { sdl, window }
    }

    pub fn run(&self) {
        let gl_context = self.window.gl_create_context().unwrap();
        self.window.gl_make_current(&gl_context).unwrap();

        /* enable vsync to cap framerate */
        self.window.subsystem().gl_set_swap_interval(1).unwrap();
        /* create new glow and imgui contexts */
        let gl = glow_context(&self.window);

        let mut imgui = Context::create();

        /* disable creation of files on disc */
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        /* setup platform and renderer, and fonts to imgui */
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let mut platform = SdlPlatform::init(&mut imgui);
        let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

        let mut timer = self.sdl.timer().unwrap();
        let mut last_tick = timer.ticks();
        let mut event_pump = self.sdl.event_pump().unwrap();

        let mut gui = Gui::default();
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

                if let Event::Quit { .. } = event {
                    break 'main;
                }
            }

            // handle gui events
            platform.prepare_frame(&mut imgui, &self.window, &event_pump);
            gui.tick(timer.ticks());
            let draw_data = gui.render(&mut imgui);

            unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
            renderer.render(draw_data).unwrap();

            self.window.gl_swap_window();
        }
    }
}
