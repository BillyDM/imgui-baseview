//! Configure your application;

use crate::{HiDpiMode, RenderSettings};
use baseview::WindowOpenOptions;

/// The settings of an application.
pub struct Settings {
    /// The `baseview` window settings.
    pub window: WindowOpenOptions,

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
