use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use imgui::{Context, Ui};
use imgui_baseview::{HiDpiMode, ImguiWindow, RenderSettings, Settings};

fn main() {
    let settings = Settings {
        window: WindowOpenOptions {
            title: String::from("imgui-baseview demo window"),
            size: Size::new(800.0, 600.0),
            scale: WindowScalePolicy::SystemScaleFactor,
        },
        clear_color: (0.0, 0.0, 0.0),
        hidpi_mode: HiDpiMode::Default,
        render_settings: RenderSettings::default(),
    };

    let state = ();

    ImguiWindow::open_blocking(
        settings,
        state,
        |_context: &mut Context, _state: &mut ()| {},
        |run: &mut bool, ui: &Ui, _state: &mut ()| {
            ui.show_demo_window(run);
        },
    );
}
