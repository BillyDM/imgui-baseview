use baseview::Window;
use raw_gl_context::{GlConfig, GlContext};

pub struct Renderer {
    context: GlContext,
    imgui_renderer: imgui_opengl_renderer::Renderer,
}

impl Renderer {
    pub fn new(window: &Window, imgui_context: &mut imgui::Context) -> Self {
        let context = GlContext::create(window, GlConfig::default()).unwrap();

        context.make_current();

        gl::load_with(|s| context.get_proc_address(s) as _);

        let imgui_renderer = imgui_opengl_renderer::Renderer::new(imgui_context, |s| {
            context.get_proc_address(s) as _
        });

        context.make_not_current();

        Self {
            context,
            imgui_renderer,
        }
    }

    pub fn render(&mut self, imgui_ui: imgui::Ui, clear_color: (f32, f32, f32)) {
        self.context.make_current();

        unsafe {
            gl::ClearColor(clear_color.0, clear_color.1, clear_color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        self.imgui_renderer.render(imgui_ui);

        // ...

        self.context.swap_buffers();
        self.context.make_not_current();
    }
}
