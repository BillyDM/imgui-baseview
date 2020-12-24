/*
Copyright (c) 2015-2020 The imgui-rs Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::mouse;
use crate::renderer::Renderer;
use crate::{HiDpiMode, Settings};
use baseview::{Event, Window, WindowHandler, WindowScalePolicy};

use std::time::Instant;

static CONTEXT_TRY_UNLOCK_WAIT_DURATIOIN: std::time::Duration =
    std::time::Duration::from_micros(10);

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
pub struct Runner<State, U>
where
    State: 'static + Send,
    U: Fn(&mut bool, &imgui::Ui, &mut State),
    U: 'static + Send,
{
    user_state: State,
    user_update: U,

    handle_rx: rtrb::Consumer<HandleMessage>,
    sus_context: Option<imgui::SuspendedContext>,
    renderer: Renderer,
    last_frame: Instant,
    clear_color: (f32, f32, f32),
    scale_policy: WindowScalePolicy,
    scale_factor: f64,

    hidpi_mode: HiDpiMode,
    hidpi_factor: f64,
    cursor_cache: Option<mouse::CursorSettings>,
    mouse_buttons: [mouse::Button; 5],
    run: bool,
}

impl<State, U> Runner<State, U>
where
    State: 'static + Send,
    U: Fn(&mut bool, &imgui::Ui, &mut State),
    U: 'static + Send,
{
    /// Open a new window
    pub fn open(
        settings: Settings,
        state: State,
        update: U,
    ) -> (Handle, Option<baseview::AppRunner>) {
        let (handle_tx, handle_rx) = rtrb::RingBuffer::new(Handle::QUEUE_SIZE).split();

        // WindowScalePolicy does not implement copy/clone.
        let scale_policy = match &settings.window.scale {
            WindowScalePolicy::SystemScaleFactor => WindowScalePolicy::SystemScaleFactor,
            WindowScalePolicy::ScaleFactor(scale) => WindowScalePolicy::ScaleFactor(*scale),
        };

        let logical_width = settings.window.size.width as f64;
        let logical_height = settings.window.size.height as f64;

        let hidpi_mode = settings.hidpi_mode;
        let mut render_settings = Some(settings.render_settings);
        let clear_color = settings.clear_color;

        (
            Handle::new(handle_tx),
            Window::open(
                settings.window,
                move |window: &mut baseview::Window<'_>| -> Runner<State, U> {
                    use imgui::{BackendFlags, Key};
                    use keyboard_types::Code;

                    let mut sus_context = imgui::SuspendedContext::create();

                    let mut scale: f64 = 0.0;
                    let mut hidpi_factor: f64 = 0.0;
                    let mut renderer: Option<Renderer> = None;

                    sus_context = use_context(sus_context, |mut context| {
                        context.set_ini_filename(None);

                        let io = context.io_mut();

                        // Assume scale for now until there is an event with a new one.
                        scale = match scale_policy {
                            WindowScalePolicy::ScaleFactor(scale) => scale,
                            WindowScalePolicy::SystemScaleFactor => 1.0,
                        };
                        hidpi_factor = hidpi_mode.apply(scale);
                        let logical_size = [
                            (logical_width as f64 * scale / hidpi_factor) as f32,
                            (logical_height as f64 * scale / hidpi_factor) as f32,
                        ];
                        io.display_framebuffer_scale = [hidpi_factor as f32, hidpi_factor as f32];
                        io.display_size = logical_size;

                        io.backend_flags.insert(BackendFlags::HAS_MOUSE_CURSORS);
                        io.backend_flags.insert(BackendFlags::HAS_SET_MOUSE_POS);
                        io[Key::Tab] = Code::Tab as _;
                        io[Key::LeftArrow] = Code::ArrowLeft as _;
                        io[Key::RightArrow] = Code::ArrowLeft as _;
                        io[Key::UpArrow] = Code::ArrowUp as _;
                        io[Key::DownArrow] = Code::ArrowDown as _;
                        io[Key::PageUp] = Code::PageUp as _;
                        io[Key::PageDown] = Code::PageDown as _;
                        io[Key::Home] = Code::Home as _;
                        io[Key::End] = Code::End as _;
                        io[Key::Insert] = Code::Insert as _;
                        io[Key::Delete] = Code::Delete as _;
                        io[Key::Backspace] = Code::Backspace as _;
                        io[Key::Space] = Code::Space as _;
                        io[Key::Enter] = Code::Enter as _;
                        io[Key::Escape] = Code::Escape as _;
                        io[Key::KeyPadEnter] = Code::NumpadEnter as _;
                        io[Key::A] = Code::KeyA as _;
                        io[Key::C] = Code::KeyC as _;
                        io[Key::V] = Code::KeyV as _;
                        io[Key::X] = Code::KeyX as _;
                        io[Key::Y] = Code::KeyY as _;
                        io[Key::Z] = Code::KeyZ as _;
                        context.set_platform_name(Some(imgui::ImString::from(format!(
                            "imgui-baseview {}",
                            env!("CARGO_PKG_VERSION")
                        ))));
                        context.set_renderer_name(Some(imgui::ImString::from(Renderer::name())));

                        renderer = Some(Renderer::new(
                            window,
                            &mut context,
                            render_settings.take().unwrap(),
                        ));

                        context.suspend()
                    });

                    Self {
                        user_state: state,
                        user_update: update,

                        handle_rx,
                        sus_context: Some(sus_context),
                        renderer: renderer.unwrap(),
                        last_frame: Instant::now(),
                        clear_color,
                        scale_policy,
                        scale_factor: scale,

                        hidpi_mode,
                        hidpi_factor,
                        cursor_cache: None,
                        mouse_buttons: [mouse::Button::INIT; 5],
                        run: true,
                    }
                },
            ),
        )
    }

    /// Scales a logical position from baseview using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    pub fn scale_pos_from_baseview(&self, logical_pos: baseview::Point) -> baseview::Point {
        scale_pos_from_baseview(
            logical_pos,
            self.scale_factor,
            self.hidpi_mode,
            self.hidpi_factor,
        )
    }

    /// Scales a logical position for baseview using the current DPI mode.
    ///
    /// This utility function is useful if you are using a DPI mode other than default, and want
    /// your application to use the same logical coordinates as imgui-rs.
    pub fn scale_pos_for_baseview(&self, logical_pos: baseview::Point) -> baseview::Point {
        scale_pos_for_baseview(
            logical_pos,
            self.scale_factor,
            self.hidpi_mode,
            self.hidpi_factor,
        )
    }
}

impl<State, U> WindowHandler for Runner<State, U>
where
    State: 'static + Send,
    U: Fn(&mut bool, &imgui::Ui, &mut State),
    U: 'static + Send,
{
    fn on_frame(&mut self) {
        self.sus_context = Some(use_context(
            self.sus_context.take().unwrap(),
            |mut context| {
                // Poll handle messages.
                while let Ok(message) = self.handle_rx.pop() {
                    match message {
                        HandleMessage::CloseRequested => {
                            // TODO: Send close message.
                        }
                    }
                }

                {
                    let io = context.io_mut();

                    // Sync mouse info.
                    for (io_down, button) in io.mouse_down.iter_mut().zip(&self.mouse_buttons) {
                        *io_down = button.get();
                    }
                    if io.want_set_mouse_pos {
                        let _baseview_position = scale_pos_for_baseview(
                            baseview::Point::new(io.mouse_pos[0] as f64, io.mouse_pos[1] as f64),
                            self.scale_factor,
                            self.hidpi_mode,
                            self.hidpi_factor,
                        );

                        // TODO: Set baseview cursor position.
                    }

                    let now = Instant::now();
                    io.update_delta_time(now.duration_since(self.last_frame));
                    self.last_frame = now;
                }

                let ui = context.frame();

                (self.user_update)(&mut self.run, &ui, &mut self.user_state);

                let io = ui.io();
                if !io
                    .config_flags
                    .contains(imgui::ConfigFlags::NO_MOUSE_CURSOR_CHANGE)
                {
                    let cursor = mouse::CursorSettings {
                        cursor: ui.mouse_cursor(),
                        draw_cursor: io.mouse_draw_cursor,
                    };
                    if self.cursor_cache != Some(cursor) {
                        // TODO : Set baseview cursor.

                        // cursor.apply(window);
                        self.cursor_cache = Some(cursor);
                    }
                }

                self.renderer.render(ui, self.clear_color);

                context.suspend()
            },
        ));
    }

    fn on_event(&mut self, _window: &mut Window, event: Event) {
        self.sus_context = Some(use_context(
            self.sus_context.take().unwrap(),
            |mut context| {
                let io = context.io_mut();

                match &event {
                    baseview::Event::Mouse(event) => match event {
                        baseview::MouseEvent::CursorMoved { position } => {
                            let position = scale_pos_from_baseview(
                                *position,
                                self.scale_factor,
                                self.hidpi_mode,
                                self.hidpi_factor,
                            );
                            io.mouse_pos = [position.x as f32, position.y as f32];
                        }
                        baseview::MouseEvent::ButtonPressed(button) => match button {
                            baseview::MouseButton::Left => self.mouse_buttons[0].set(true),
                            baseview::MouseButton::Middle => self.mouse_buttons[1].set(true),
                            baseview::MouseButton::Right => self.mouse_buttons[2].set(true),
                            baseview::MouseButton::Other(3) => self.mouse_buttons[3].set(true),
                            baseview::MouseButton::Other(4) => self.mouse_buttons[4].set(true),
                            _ => {}
                        },
                        baseview::MouseEvent::ButtonReleased(button) => match button {
                            baseview::MouseButton::Left => self.mouse_buttons[0].set(false),
                            baseview::MouseButton::Middle => self.mouse_buttons[1].set(false),
                            baseview::MouseButton::Right => self.mouse_buttons[2].set(false),
                            baseview::MouseButton::Other(3) => self.mouse_buttons[3].set(false),
                            baseview::MouseButton::Other(4) => self.mouse_buttons[4].set(false),
                            _ => {}
                        },
                        baseview::MouseEvent::WheelScrolled(scroll_delta) => match scroll_delta {
                            baseview::ScrollDelta::Lines { x, y } => {
                                io.mouse_wheel_h = *x;
                                io.mouse_wheel = *y;
                            }
                            baseview::ScrollDelta::Pixels { x, y } => {
                                if *x < 0.0 {
                                    io.mouse_wheel_h -= 1.0;
                                } else if *x > 1.0 {
                                    io.mouse_wheel_h += 1.0;
                                }

                                if *y < 0.0 {
                                    io.mouse_wheel -= 1.0;
                                } else if *y > 1.0 {
                                    io.mouse_wheel_h += 1.0;
                                }
                            }
                        },
                        _ => {}
                    },
                    baseview::Event::Keyboard(event) => {
                        use keyboard_types::Code;

                        let pressed = event.state == keyboard_types::KeyState::Down;

                        io.keys_down[event.code as usize] = pressed;

                        // This is a bit redundant here, but we'll leave it in. The OS occasionally
                        // fails to send modifiers keys, but it doesn't seem to send false-positives,
                        // so double checking isn't terrible in case some system *doesn't* send
                        // device events sometimes.
                        match event.code {
                            Code::ShiftLeft | Code::ShiftRight => io.key_shift = pressed,
                            Code::ControlLeft | Code::ControlRight => io.key_ctrl = pressed,
                            Code::AltLeft | Code::AltRight => io.key_alt = pressed,
                            Code::MetaLeft | Code::MetaRight => io.key_super = pressed,
                            _ => (),
                        }

                        if pressed {
                            if let keyboard_types::Key::Character(written) = &event.key {
                                for chr in written.chars() {
                                    // Exclude the backspace key ('\u{7f}'). Otherwise we will insert this char and then
                                    // delete it.
                                    if chr != '\u{7f}' {
                                        io.add_input_character(chr)
                                    }
                                }
                            }
                        }
                    }
                    baseview::Event::Window(event) => {
                        match event {
                            baseview::WindowEvent::Resized(window_info) => {
                                self.scale_factor = match self.scale_policy {
                                    WindowScalePolicy::ScaleFactor(scale) => scale,
                                    WindowScalePolicy::SystemScaleFactor => window_info.scale(),
                                };

                                let new_hidpi_factor = self.hidpi_mode.apply(self.scale_factor);

                                // Mouse position needs to be changed while we still have both the old and the new
                                // values
                                if io.mouse_pos[0].is_finite() && io.mouse_pos[1].is_finite() {
                                    io.mouse_pos = [
                                        io.mouse_pos[0]
                                            * (new_hidpi_factor / self.hidpi_factor) as f32,
                                        io.mouse_pos[1]
                                            * (new_hidpi_factor / self.hidpi_factor) as f32,
                                    ];
                                }

                                self.hidpi_factor = new_hidpi_factor;

                                let logical_size = [
                                    (window_info.physical_size().width as f64 / self.hidpi_factor)
                                        as f32,
                                    (window_info.physical_size().height as f64 / self.hidpi_factor)
                                        as f32,
                                ];

                                io.display_framebuffer_scale =
                                    [self.hidpi_factor as f32, self.hidpi_factor as f32];
                                io.display_size = logical_size;
                            }
                            baseview::WindowEvent::WillClose => {}
                            _ => {}
                        }
                    }
                }

                context.suspend()
            },
        ));
    }
}

/// Scales a logical position from baseview using the current DPI mode.
///
/// This utility function is useful if you are using a DPI mode other than default, and want
/// your application to use the same logical coordinates as imgui-rs.
#[inline]
fn scale_pos_from_baseview(
    logical_pos: baseview::Point,
    scale_factor: f64,
    hidpi_mode: HiDpiMode,
    hidpi_factor: f64,
) -> baseview::Point {
    match hidpi_mode {
        HiDpiMode::Default => logical_pos,
        _ => baseview::Point::new(
            logical_pos.x * scale_factor / hidpi_factor,
            logical_pos.y * scale_factor / hidpi_factor,
        ),
    }
}

/// Scales a logical position for baseview using the current DPI mode.
///
/// This utility function is useful if you are using a DPI mode other than default, and want
/// your application to use the same logical coordinates as imgui-rs.
#[inline]
fn scale_pos_for_baseview(
    logical_pos: baseview::Point,
    scale_factor: f64,
    hidpi_mode: HiDpiMode,
    hidpi_factor: f64,
) -> baseview::Point {
    match hidpi_mode {
        HiDpiMode::Default => logical_pos,
        _ => baseview::Point::new(
            logical_pos.x * hidpi_factor / scale_factor,
            logical_pos.y * hidpi_factor / scale_factor,
        ),
    }
}

fn use_context<F: FnMut(imgui::Context) -> imgui::SuspendedContext>(
    mut sus_context: imgui::SuspendedContext,
    mut f: F,
) -> imgui::SuspendedContext {
    loop {
        match sus_context.activate() {
            Ok(context) => {
                return (f)(context);
            }
            Err(new_sus_context) => {
                std::thread::sleep(CONTEXT_TRY_UNLOCK_WAIT_DURATIOIN);
                sus_context = new_sus_context
            }
        };
    }
}
