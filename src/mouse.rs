/*
Copyright (c) 2015-2020 The imgui-rs Developers

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/


use std::cell::Cell;
use std::cmp::Ordering;

use baseview::MouseCursor;

/// State of a single mouse button. Used so that we can detect cases where mouse
/// press and release occur on the same frame (seems surprisingly frequent on
/// macOS now...)
#[derive(Debug, Clone, Default)]
pub(crate) struct Button {
    pressed_this_frame: Cell<bool>,
    state: Cell<bool>,
}

impl Button {
    // we can use this in an array initializer, unlike `Default::default()` or a
    // `const fn new()`.
    #[allow(clippy::declare_interior_mutable_const)]
    pub(crate) const INIT: Button = Self {
        pressed_this_frame: Cell::new(false),
        state: Cell::new(false),
    };
    pub(crate) fn set(&self, pressed: bool) {
        self.state.set(pressed);
        if pressed {
            self.pressed_this_frame.set(true);
        }
    }
    pub(crate) fn get(&self) -> bool {
        // If we got a press this frame, record it even if we got a release
        // too — this way we don't drop mouse clicks where the release comes
        // in on the same frame as the press. (This mirrors what Dear ImGUI
        // seems to do in the `imgui_impl_*`)
        self.pressed_this_frame.replace(false) || self.state.get()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) struct CursorSettings {
    pub cursor: Option<imgui::MouseCursor>,
    pub draw_cursor: bool,
}

fn to_baseview_cursor(cursor: imgui::MouseCursor) -> baseview::MouseCursor {
    match cursor {
        imgui::MouseCursor::Arrow => MouseCursor::Default,
        imgui::MouseCursor::TextInput => MouseCursor::Text,
        imgui::MouseCursor::ResizeAll => MouseCursor::Default,
        imgui::MouseCursor::ResizeNS => MouseCursor::NsResize,
        imgui::MouseCursor::ResizeEW => MouseCursor::EwResize,
        imgui::MouseCursor::ResizeNESW => MouseCursor::NeswResize,
        imgui::MouseCursor::ResizeNWSE => MouseCursor::NwseResize,
        imgui::MouseCursor::Hand => MouseCursor::Hand,
        imgui::MouseCursor::NotAllowed => MouseCursor::NotAllowed,
    }
}

impl CursorSettings {
    fn apply(&self) {

        // TODO

        match self.cursor {
            Some(mouse_cursor) if !self.draw_cursor => {
                //window.set_cursor_visible(true);
                //window.set_cursor_icon(to_winit_cursor(mouse_cursor));
            }
            _ => {
                //window.set_cursor_visible(false);
            },
        }
    }
}