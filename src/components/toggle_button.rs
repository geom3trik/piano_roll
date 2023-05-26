use vizia::prelude::*;

#[derive(Lens)]
pub struct ToggleButton {
    checked: bool,
}

pub enum ToggleButtonEvent {
    Toggle,
}

impl ToggleButton {
    pub fn new<'a>(cx: &'a mut Context, icon: &str) -> Handle<'a, Self> {
        Self { checked: false }
            .build(cx, |cx| {
                // Icon::new(cx, icon).hoverable(false);
            })
            .size(Auto)
            .checked(ToggleButton::checked)
            .on_press(|cx| cx.emit(ToggleButtonEvent::Toggle))
    }
}

impl View for ToggleButton {
    fn element(&self) -> Option<&'static str> {
        Some("toggle_button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|toggle_button_event, _| match toggle_button_event {
            ToggleButtonEvent::Toggle => {
                self.checked ^= true;
            }
        })
    }
}
