use baseview::{Parent, Size, WindowOpenOptions, WindowScalePolicy};
use imgui::Ui;
use imgui_baseview::{HiDpiMode, RenderSettings, Runner, Settings};

fn main() {
    let settings = Settings {
        window: WindowOpenOptions {
            title: String::from("imgui-baseview demo window"),
            size: Size::new(800.0, 600.0),
            parent: Parent::None,
            scale: WindowScalePolicy::SystemScaleFactor,
        },
        clear_color: (0.0, 0.0, 0.0),
        hidpi_mode: HiDpiMode::Default,
        render_settings: RenderSettings::default(),
    };

    let state = ();

    let (_, opt_app_runner) = Runner::open(
        settings,
        state,
        move |run: &mut bool, ui: &Ui, _state: &mut ()| {
            ui.show_demo_window(run);
        },
    );

    opt_app_runner.unwrap().app_run_blocking();
}
