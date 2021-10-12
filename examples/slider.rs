use baseview::{Size, WindowOpenOptions, WindowScalePolicy};
use imgui::*;
use imgui_baseview::{HiDpiMode, ImguiWindow, RenderSettings, Settings};

fn main() {
    let settings = Settings {
        window: WindowOpenOptions {
            title: String::from("imgui-baseview slider"),
            size: Size::new(800.0, 600.0),
            scale: WindowScalePolicy::SystemScaleFactor,
        },
        clear_color: (0.0, 0.0, 0.0),
        hidpi_mode: HiDpiMode::Default,
        render_settings: RenderSettings::default(),
    };

    ImguiWindow::open_blocking(
        settings,
        State::default(),
        |_context: &mut Context, _state: &mut State| {},
        |run: &mut bool, ui: &Ui, state: &mut State| {
            example_selector(run, ui, state);
            match state.example {
                1 => example_1(ui, state),
                2 => example_2(ui, state),
                _ => (),
            }
        },
    );
}

fn example_selector(run: &mut bool, ui: &Ui, state: &mut State) {
    let w = Window::new("Slider examples")
        .opened(run)
        .position([20.0, 20.0], Condition::Appearing)
        .size([700.0, 80.0], Condition::Appearing)
        .resizable(false);
    w.build(&ui, || {
        let mut clicked = false;
        clicked |= ui.radio_button("Example 1: Basic sliders", &mut state.example, 1);
        clicked |= ui.radio_button("Example 2: Slider arrays", &mut state.example, 2);
        if clicked {
            state.reset();
        }
    });
}

fn example_1(ui: &Ui, state: &mut State) {
    let w = Window::new("Example 1: Basic sliders")
        .size([700.0, 340.0], Condition::Appearing)
        .position([20.0, 120.0], Condition::Appearing);
    w.build(&ui, || {
        ui.text("All of the following data types are supported:");
        ui.text("Signed:   i8 i16 i32 i64");
        ui.text("Unsigned: u8 u16 u32 u64");
        ui.text("Floats:   f32 f64");

        Slider::new("u8 value", std::u8::MIN, std::u8::MAX)
            .build(&ui, &mut state.u8_value);

        Slider::new("f32 value", -999_999_999.0, 999_999_999.0)
            .build(&ui, &mut state.f32_value);

        ui.separator();
        ui.text("Slider range can be limited:");
        Slider::new("i32 value with range", -999, 999)
            .build(&ui, &mut state.i32_value);
        ui.text("Note that for 32-bit/64-bit types, sliders are always limited to half of the natural type range!");

        ui.separator();
        ui.text("Value formatting can be customized with a C-style printf string:");
        Slider::new("f64 value with custom formatting", -999_999_999.0, 999_999_999.0)
            .display_format("%09.0f")
            .build(&ui, &mut state.f64_formatted);

        ui.separator();
        ui.text("Vertical sliders require a size parameter but otherwise work in a similar way:");
        VerticalSlider::new("vertical\nu8 value", [50.0, 50.0], std::u8::MIN, std::u8::MAX)
            .build(&ui, &mut state.u8_value);
    });
}

fn example_2(ui: &Ui, state: &mut State) {
    let w = Window::new("Example 2: Slider arrays")
        .size([700.0, 260.0], Condition::Appearing)
        .position([20.0, 120.0], Condition::Appearing);
    w.build(&ui, || {
        ui.text("You can easily build a slider group from an array of values:");
        Slider::new("[u8; 4]", std::u8::MIN, std::u8::MAX).build_array(&ui, &mut state.array);

        ui.text("You don't need to use arrays with known length; arbitrary slices can be used:");
        let slice: &mut [u8] = &mut state.array[1..=2];
        Slider::new("subslice", std::u8::MIN, std::u8::MAX).build_array(&ui, slice);
    });
}

#[derive(Default)]
struct State {
    example: u32,
    i32_value: i32,
    u8_value: u8,
    f32_value: f32,
    f64_formatted: f64,
    array: [u8; 4],
}

impl State {
    fn reset(&mut self) {}
}
