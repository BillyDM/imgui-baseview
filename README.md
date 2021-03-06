# imgui-baseview
![Test](https://github.com/BillyDM/imgui-baseview/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/imgui-baseview.svg)](https://crates.io/crates/imgui-baseview)
[![License](https://img.shields.io/crates/l/imgui-baseview.svg)](https://github.com/BillyDM/imgui-baseview/blob/main/LICENSE)

A [`baseview`] backend for [`imgui-rs`].

<div align="center">
    <img src="screenshot.png">
</div>

## Simple Usage Example

```rust
use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use imgui::{im_str, Condition, Context, Ui, Window};
use imgui_baseview::{HiDpiMode, ImguiWindow, RenderSettings, Settings};

fn main() {
    let settings = Settings {
        window: WindowOpenOptions {
            title: String::from("imgui-baseview hello world"),
            size: Size::new(300.0, 110.0),
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
        // Called once in the constructor. This can be used to make any additional
        // configurations to the `imgui::Context` struct.
        |_context: &mut Context, _state: &mut ()| {},
        // Called before each frame. Here you should update the state of your
        // application and build the UI.
        |run: &mut bool, ui: &Ui, _state: &mut ()| {
            Window::new(im_str!("Hello world"))
                .opened(run)
                .size([300.0, 110.0], Condition::FirstUseEver)
                .position([0.0, 0.0], Condition::FirstUseEver)
                .build(ui, || {
                    ui.text(im_str!("Hello world!"));
                    ui.text(im_str!("こんにちは世界！"));
                    ui.text(im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });
        },
    );
}
```

## VST / LV2 / AU Plugins

Examples of how to use this library for audio plugins can be found here:
* [`imgui_baseview_test_vst2`]

[`baseview`]: https://github.com/RustAudio/baseview
[`imgui-rs`]: https://github.com/imgui-rs/imgui-rs
[`imgui_baseview_test_vst2`]: https://github.com/DGriffin91/imgui_baseview_test_vst2
