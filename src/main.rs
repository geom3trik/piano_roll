use vizia::prelude::*;

pub mod data;
pub use data::*;

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

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/theme.css"))
            .expect("Failed to load stylesheet");

        PianoRollData {
            key_labels: KeyLabels::Black,

            grid: Grid::F16,
            clip_duration: MusicalTime::from_beats(4),
            view_start: MusicalTime::from_beats(0),
            view_end: MusicalTime::from_beats(4),
        }
        .build(cx);

        TopBar::new(cx);
        HStack::new(cx, |cx| {
            //Element::new(cx).background_color(Color::from("#323232"));

            PianoRoll::new(cx);

            // Properties
            VStack::new(cx, |cx| {}).width(Pixels(200.0));
        });
    })
    .run()
}
