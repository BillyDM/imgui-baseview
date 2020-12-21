use imgui_baseview::{Runner, WindowOpenOptions, Size, WindowScalePolicy, Parent};

fn main() {
    let window_settings = WindowOpenOptions {
        title: String::from("imgui-baseview hello world"),
        size: Size::new(500.0, 350.0),
        scale: WindowScalePolicy::SystemScaleFactor,
        parent: Parent::None,
    };

    let (_, opt_app_runner) = Runner::open(window_settings);

    opt_app_runner.unwrap().app_run_blocking();
}