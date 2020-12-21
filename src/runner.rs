use baseview::{WindowHandler, Window, Event, Parent};

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
        Self { handle_tx }
    }

    pub fn request_window_close(&mut self) {
        self.handle_tx.push(HandleMessage::CloseRequested).unwrap();
    }
}

/// Handles an imgui-baseview application
#[allow(missing_debug_implementations)]
pub struct Runner {
    handle_rx: rtrb::Consumer<HandleMessage>,
}

impl Runner {
    /// Open a new window
    pub fn open(
        window_settings: baseview::WindowOpenOptions,
    ) -> (Handle, Option<baseview::AppRunner>) {
        let (handle_tx, handle_rx) =
            rtrb::RingBuffer::new(Handle::QUEUE_SIZE).split();

        (
            Handle::new(handle_tx),
            Window::open(
                window_settings,
                move |window: &mut baseview::Window<'_>| -> Runner {
                    Self {
                        handle_rx,
                    }
                }
            )
        )
    }
}

impl WindowHandler for Runner {
    fn on_frame(&mut self) {
        
    }

    fn on_event(&mut self, _window: &mut Window, _event: Event) {
        
    }
}