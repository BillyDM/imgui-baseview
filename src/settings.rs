//! Configure your application;

use crate::{HiDpiMode, RenderSettings};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowScalePolicy {
    /// Use the provided scale factor.
    ScaleFactor(f64),
    /// Use the system's scale factor.
    SystemScaleFactor,
}

impl Default for WindowScalePolicy {
    fn default() -> Self {
        WindowScalePolicy::SystemScaleFactor
    }
}

impl From<WindowScalePolicy> for baseview::WindowScalePolicy {
    fn from(p: WindowScalePolicy) -> Self {
        match p {
            WindowScalePolicy::ScaleFactor(scale) => {
                baseview::WindowScalePolicy::ScaleFactor(scale)
            }
            WindowScalePolicy::SystemScaleFactor => baseview::WindowScalePolicy::SystemScaleFactor,
        }
    }
}

/// The settings of an application.
pub struct Settings {
    /// The [`Window`] settings
    ///
    /// [`Window`]: struct.Window.html
    pub window: Window,

    /// The color to clear the screen on render (R, G, B).
    pub clear_color: (f32, f32, f32),

    /// DPI factor handling mode.
    ///
    /// Applications that use imgui-rs might want to customize the used DPI factor and not use
    /// directly the value coming from baseview.
    ///
    /// **Note: if you use a mode other than default and the DPI factor is adjusted, baseview and imgui-rs
    /// will use different logical coordinates, so be careful if you pass around logical size or
    /// position values.**
    pub hidpi_mode: HiDpiMode,

    /// The settings for the rendering backend.
    pub render_settings: RenderSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            window: Window::default(),
            clear_color: (0.0, 0.0, 0.0),
            hidpi_mode: HiDpiMode::Default,
            render_settings: RenderSettings::default(),
        }
    }
}

/// The window settings of an application.
#[derive(Debug)]
pub struct Window {
    /// The window title.
    pub title: String,
    /// The logical size of the window.
    pub logical_size: (u32, u32),
    /// The initial dpi scaling policy
    pub scale_policy: WindowScalePolicy,
}

impl Default for Window {
    fn default() -> Window {
        Window {
            title: String::from("imgui-baseview"),
            logical_size: (1024, 768),
            scale_policy: Default::default(),
        }
    }
}
