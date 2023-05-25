use vizia::prelude::*;

use crate::ToggleButton;

pub struct TopBar {}

impl TopBar {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}
            .build(cx, |cx| {
                // Adjustments
                // Space
                ToggleButton::new(cx, "\u{eae3}");
                ToggleButton::new(cx, "\u{eb20}");
            })
            .layout_type(LayoutType::Row)
    }
}

impl View for TopBar {
    fn element(&self) -> Option<&'static str> {
        Some("topbar")
    }
}
