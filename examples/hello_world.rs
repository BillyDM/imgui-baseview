use imgui_baseview::{Runner, settings, Settings, WindowScalePolicy, Parent};

fn main() {
    let settings = Settings {
        window: settings::Window {
            title: String::from("imgui-baseview hello world"),
            logical_size: (500, 350),
            scale_policy: WindowScalePolicy::SystemScaleFactor,
        }
    };

    let (_, opt_app_runner) = Runner::open(settings, Parent::None);

    opt_app_runner.unwrap().app_run_blocking();
}