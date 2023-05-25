use vizia::{prelude::*, image::Pixel};

pub mod util;
pub use util::*;

pub mod components;
pub use components::*;


#[derive(Lens)]
pub struct AppData {
    // Start time of the clip being edited by the piano roll.
    pub clip_start: MusicalTime,
    // End time of the clip being edited by the piano roll.
    pub clip_end: MusicalTime,
}

fn main() {
    Application::new(|cx|{

        cx.add_stylesheet(include_style!("src/theme.css"))
            .expect("Failed to load stylesheet");
        TopBar::new(cx);
        ZStack::new(cx, |cx|{
            Element::new(cx).background_color(Color::from("#323232"));
            PianoRoll::new(cx).space(Pixels(4.0));
            Element::new(cx).border_width(Pixels(4.0)).border_color(Color::from("#323232")).border_radius(Pixels(8.0)).hoverable(false);
        });
    }).run();
}
