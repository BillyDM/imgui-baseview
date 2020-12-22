use imgui::Ui;
use imgui_baseview::{settings, HiDpiMode, Parent, Runner, Settings, WindowScalePolicy};

fn main() {
    let settings = Settings {
        window: settings::Window {
            title: String::from("imgui-baseview demo window"),
            logical_size: (800, 600),
            scale_policy: WindowScalePolicy::SystemScaleFactor,
        },
        clear_color: (0.0, 0.0, 0.0),
        hidpi_mode: HiDpiMode::Default,
    };

    let state = ();

    let (_, opt_app_runner) = Runner::open(
        settings,
        Parent::None,
        state,
        move |run: &mut bool, ui: &Ui, _state: &mut ()| {
            ui.show_demo_window(run);
        },
    );

    opt_app_runner.unwrap().app_run_blocking();
}
