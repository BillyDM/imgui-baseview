use imgui_baseview::{Runner, settings, Settings, WindowScalePolicy, Parent, HiDpiMode};

fn main() {
    let settings = Settings {
        window: settings::Window {
            title: String::from("imgui-baseview hello world"),
            logical_size: (500, 350),
            scale_policy: WindowScalePolicy::SystemScaleFactor,
        },
        clear_color: (0.0, 0.0, 0.0),
        hidpi_mode: HiDpiMode::Default,
    };

    let (_, opt_app_runner) = Runner::open(settings, Parent::None);

    opt_app_runner.unwrap().app_run_blocking();
}