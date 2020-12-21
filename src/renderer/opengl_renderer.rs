use raw_gl_context::{GlConfig, GlContext};
use baseview::Window;

pub struct Renderer {
    context: GlContext
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let context = GlContext::create(window, GlConfig::default()).unwrap();

        context.make_current();

        context.make_not_current();

        Self {
            context,
        }
    }

    pub fn render(&mut self) {
        self.context.make_current();

        // ...

        self.context.swap_buffers();
        self.context.make_not_current();
    }
}