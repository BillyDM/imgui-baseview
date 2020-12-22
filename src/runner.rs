use baseview::{WindowHandler, Window, Event, Parent};
use crate::renderer::Renderer;
use crate::{Settings, WindowScalePolicy};

use std::time::Instant;

pub(crate) enum HandleMessage {
    CloseRequested,
}

#[allow(missing_debug_implementations)]
pub struct Handle {
    handle_tx: rtrb::Producer<HandleMessage>,
}

impl Handle {
    pub const QUEUE_SIZE: usize = 10;

    pub(crate) fn new(handle_tx: rtrb::Producer<HandleMessage>) -> Self {
        Self {
            handle_tx,
        }
    }

    pub fn request_window_close(&mut self) {
        self.handle_tx.push(HandleMessage::CloseRequested).unwrap();
    }
}

/// Handles an imgui-baseview application
#[allow(missing_debug_implementations)]
pub struct Runner {
    handle_rx: rtrb::Consumer<HandleMessage>,
    imgui_context: imgui::Context,
    renderer: Renderer,
    last_frame: Instant,
}

impl Runner {
    /// Open a new window
    pub fn open(
        settings: Settings,
        parent: Parent,
    ) -> (Handle, Option<baseview::AppRunner>) {
        let (handle_tx, handle_rx) =
            rtrb::RingBuffer::new(Handle::QUEUE_SIZE).split();
        
        let scale_policy = settings.window.scale_policy;

        let logical_width = settings.window.logical_size.0 as f64;
        let logical_height = settings.window.logical_size.1 as f64;

        let window_settings = baseview::WindowOpenOptions {
            title: settings.window.title.clone(),
            size: baseview::Size::new(logical_width, logical_height),
            scale: settings.window.scale_policy.into(),
            parent,
        };

        (
            Handle::new(handle_tx),
            Window::open(
                window_settings,
                move |window: &mut baseview::Window<'_>| -> Runner {

                    let mut imgui_context = imgui::Context::create();
                    imgui_context.set_ini_filename(None);

                    // Assume scale for now until there is an event with a new one.
                    let scale = match scale_policy {
                        WindowScalePolicy::ScaleFactor(scale) => scale,
                        WindowScalePolicy::SystemScaleFactor => 1.0,
                    };

                    let physical_size = [
                        (logical_width * scale) as f32,
                        (logical_height * scale) as f32,
                    ];

                    imgui_context.io_mut().display_size = physical_size;
                    imgui_context.io_mut().display_framebuffer_scale = [scale as f32, scale as f32];

                    let renderer = Renderer::new(window, &mut imgui_context);

                    Self {
                        handle_rx,
                        imgui_context,
                        renderer,
                        last_frame: Instant::now(),
                    }
                }
            )
        )
    }
}

impl WindowHandler for Runner {
    fn on_frame(&mut self) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_frame = now;
        self.imgui_context.io_mut().delta_time = delta_s;

        let ui = self.imgui_context.frame();
        ui.show_demo_window(&mut true);

        self.renderer.render(ui);
    }

    fn on_event(&mut self, _window: &mut Window, _event: Event) {
        
    }
}