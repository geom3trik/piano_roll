use crate::{MusicalTime, Note, NoteData, Pitch};
use vizia::prelude::*;

pub mod piano_view;
pub use piano_view::*;

pub mod grid_view;
pub use grid_view::*;

pub mod note_view;
pub use note_view::*;

pub enum PianoRollEvent {
    SelectNote(usize),
    MoveDown(usize),
    MoveUp(usize),
    StartDrag(usize),
    EndDrag(usize),
}

#[derive(Lens)]
pub struct PianoRoll {
    notes: Vec<NoteData>,
    scale: f32,
}

impl PianoRoll {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            notes: vec![NoteData::new(
                Pitch::new(0, Note::B, 0.0),
                MusicalTime::from_beats(0),
                MusicalTime::from_quarter_beats(0, 1),
            )],
            scale: 1.0,
        }
        .build(cx, |cx| {
            ScrollView::new(cx, 0.0, 0.5, false, false, |cx| {
                // ZStack::new(cx, |cx|{
                // Grid + Notes
                GridView::new(cx, |cx| {
                    //Binding::new(cx, PianoRoll::notes, |cx, notes|{
                    for (idx, note) in PianoRoll::notes.get(cx).iter().enumerate() {
                        let note = note.clone();
                        Binding::new(cx, GridView::root, move |cx, grid| {
                            let grid = grid.get(cx);
                            // if time_to_pos(note.start, note.end, grid.start, grid.end).is_some() {
                            // NoteView::new(cx, idx).bind(
                            //     PianoRoll::notes.index(idx),
                            //     move |handle, note| {
                            //         let note_data = note.get(&handle);
                            //         let (posx, width) = time_to_pos(
                            //             note_data.start,
                            //             note_data.end,
                            //             grid.start,
                            //             grid.end,
                            //         )
                            //         .unwrap();
                            //         let (posy, height) = pitch_to_pos(note_data.pitch);

                            //         handle
                            //             .left(Pixels(60.0 + posx))
                            //             .top(Pixels(posy))
                            //             .height(Pixels(height))
                            //             .width(Pixels(width));
                            //     },
                            // );
                            // }
                        });
                    }
                });

                // Piano
                PianoView::new(cx, PianoRoll::scale);
            });
        })
        .row_between(Pixels(1.0))
    }
}

impl View for PianoRoll {
    fn element(&self) -> Option<&'static str> {
        Some("piano-roll")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|piano_roll_event, meta| match piano_roll_event {
            PianoRollEvent::MoveDown(index) => {
                self.notes[*index].decrement();
            }

            PianoRollEvent::MoveUp(index) => {
                self.notes[*index].increment();
            }

            _ => {}
        });
    }
}
