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
            ScrollView::new(cx, 0.0, 0.0, true, true, |cx| {
                // ZStack::new(cx, |cx|{
                // Grid + Notes
                GridView::new(cx, |cx| {
                    //Binding::new(cx, PianoRoll::notes, |cx, notes|{
                    // for (idx, note) in PianoRoll::notes.get(cx).iter().enumerate() {
                    //     let note = note.clone();
                    //     Binding::new(cx, GridView::root, move |cx, grid| {
                    //         let grid = grid.get(cx);
                    //         if time_to_pos(note.start, note.end, grid.start, grid.end).is_some() {
                    //         //NoteView::new(cx, idx, PianoRoll::notes.idx(idx));
                    //         // .bind(
                    //         //     PianoRoll::notes.idx(idx),
                    //         //     move |mut handle, note| {
                    //         //         let note_data = note.get(&handle);
                    //         //         let (posx, width) = time_to_pos(
                    //         //             note_data.start,
                    //         //             note_data.end,
                    //         //             grid.start,
                    //         //             grid.end,
                    //         //         )
                    //         //         .unwrap();
                    //         //         let (posy, height) = pitch_to_pos(note_data.pitch, handle.context().scale_factor());

                    //         //         handle
                    //         //             .left(Pixels(60.0 + posx))
                    //         //             .top(Pixels(posy))
                    //         //             .height(Pixels(height))
                    //         //             .width(Pixels(width));
                    //         //     },
                    //         // );
                    //         }
                    //     });
                    // }
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

fn time_to_pos(
    start: MusicalTime,
    end: MusicalTime,
    grid_start: MusicalTime,
    grid_end: MusicalTime,
) -> Option<(f32, f32)> {
    if start > grid_end || end < grid_start {
        return None;
    }

    let px_per_beat = 100.0;

    let offset_from_start = (start.as_beats_f64() - grid_start.as_beats_f64()) * px_per_beat;
    let length = (end.as_beats_f64() - start.as_beats_f64()) * px_per_beat;

    Some((offset_from_start as f32, length as f32))
}

// Convert the pitch of a note to a position on the grid.
fn pitch_to_pos(pitch: Pitch, scale: f32) -> (f32, f32) {
    let note = pitch.note();

    let lane_height1 = 14.0 * scale;
    let lane_height2 = 13.0 * scale;

    let octave_height = 164.0 * 1.5;

    let (offset, height) = match note {
        Note::B => (0.0, lane_height2),
        Note::AS => (lane_height2, lane_height1),
        Note::A => (lane_height1 + lane_height2, lane_height2),
        Note::GS => (lane_height1 + 2.0 * lane_height2, lane_height1),
        Note::G => (2.0 * lane_height1 + 2.0 * lane_height2, lane_height2),
        Note::FS => (2.0 * lane_height1 + 3.0 * lane_height2, lane_height1),
        Note::F => (3.0 * lane_height1 + 3.0 * lane_height2, lane_height2),
        Note::E => (3.0 * lane_height1 + 4.0 * lane_height2, lane_height1),
        Note::DS => (4.0 * lane_height1 + 4.0 * lane_height2, lane_height1),
        Note::D => (5.0 * lane_height1 + 4.0 * lane_height2, lane_height1),
        Note::CS => (6.0 * lane_height1 + 4.0 * lane_height2, lane_height1),
        Note::C => (7.0 * lane_height1 + 4.0 * lane_height2, lane_height1),
    };

    let octave = pitch.octave();

    // let multiplier = (octave.0 - 5) as f32 * octave_height;

    (offset, height)
}
