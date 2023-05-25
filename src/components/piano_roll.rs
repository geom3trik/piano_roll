use vizia::icons::{ICON_PENCIL, ICON_SEARCH, ICON_SLICE};
use vizia::vg;
use vizia::{icons::ICON_POINTER, prelude::*};

use crate::MusicalTime;

#[derive(Debug, Lens, Clone, Data)]
pub struct NoteData {
    pitch: Pitch,
    start: MusicalTime,
    end: MusicalTime,
}

impl NoteData {
    pub fn new(pitch: Pitch, start: MusicalTime, end: MusicalTime) -> Self {
        Self {
            pitch,
            start,
            end,
        }
    }
}

#[derive(Debug, Default, Data, Clone, Copy, PartialEq, Eq)]
pub struct Pitch(u8);

const NOTE_MASK: u8 = 0b11110000;
const OCTAVE_MASK: u8 = 0b00001111;

// 0 - sharp
// 1-3 - note
// 4-7 - octave

impl Pitch {
    pub fn new(note: Note, octave: Octave) -> Self {
        let mut p = Self(0);
        p.set_note(note);
        p.set_octave(octave);
        p
    }

    pub fn note(&self) -> Note {
        Note::from_u8((self.0 & NOTE_MASK) >> 4)
    }

    pub fn octave(&self) -> Octave {
        Octave(self.0 & OCTAVE_MASK)
    }

    pub fn set_note(&mut self, note: Note) {
        let note = note as u8;
        self.0 |= note << 4
    }

    pub fn set_octave(&mut self, octave: Octave) {
        // TODO: Check that octave isn't too high
        self.0 |= octave.0
    }

    pub fn increment(&mut self) {
        let note = self.note();
        let mut octave = self.octave();

        if self.note() == Note::B {
            octave = octave.next();
        }

        *self = Pitch::new(note.next(), octave);
    }

    pub fn decrement(&mut self) {
        let note = self.note();
        let mut octave = self.octave();

        if self.note() == Note::C {
            octave = octave.prev();
        }

        *self = Pitch::new(note.prev(), octave);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Octave(pub u8);

impl Octave {
    pub fn next(&self) -> Self {
        Octave(self.0 + 1)
    }

    pub fn prev(&self) -> Self {
        Octave(self.0.saturating_sub(1))
    }
}

impl std::fmt::Display for Octave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            f.write_str("\u{00b7}\u{00b7}")
        } else {
            f.write_fmt(format_args!("{:02}", self.0))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Note {
    B = 0,
    AS,
    A,
    GS,
    G,
    FS,
    F,
    E,
    DS,
    D,
    CS,
    C,
}

impl Note {
    fn from_u8(val: u8) -> Self {
        use Note::*;
        match val {
            0 => B,
            1 => AS,
            2 => A,
            3 => GS,
            4 => G,
            5 => FS,
            6 => F,
            7 => E,
            8 => DS,
            9 => D,
            10 => CS,
            11 => C,
            _ => unreachable!(),
        }
    }

    fn next(&self) -> Self {
        match self {
            Note::B => Note::C,
            Note::AS => Note::B,
            Note::A => Note::AS,
            Note::GS => Note::A,
            Note::G => Note::GS,
            Note::FS => Note::G,
            Note::F => Note::FS,
            Note::E => Note::F,
            Note::DS => Note::E,
            Note::D => Note::DS,
            Note::CS => Note::D,
            Note::C => Note::CS,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Note::C => Note::B,
            Note::B => Note::AS,
            Note::AS => Note::A,
            Note::A => Note::GS,
            Note::GS => Note::G,
            Note::G => Note::FS,
            Note::FS => Note::F,
            Note::F => Note::E,
            Note::E => Note::DS,
            Note::DS => Note::D,
            Note::D => Note::CS,
            Note::CS => Note::C,
        }
    }
}

// impl std::fmt::Display for Note {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         use Note::*;
//         match self {
//             A => f.write_str("A-"),
//             AS => f.write_str("A#"),
//             B => f.write_str("B-"),
//             C => f.write_str("C-"),
//             CS => f.write_str("C#"),
//             D => f.write_str("D-"),
//             DS => f.write_str("D#"),
//             E => f.write_str("E-"),
//             F => f.write_str("F-"),
//             FS => f.write_str("F#"),
//             G => f.write_str("G-"),
//             GS => f.write_str("G#"),
//         }
//     }
// }

// impl std::fmt::Display for Pitch {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("{}{}", self.note(), self.octave()))
//     }
// }

pub struct GridData {
    start: MusicalTime,
    end: MusicalTime,
    pixels_per_beat: f32,
}

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

}

impl PianoRoll {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            notes: vec![NoteData::new(
                Pitch::new(Note::B, Octave(0)),
                MusicalTime::from_beats(0),
                MusicalTime::from_quarter_beats(0, 1),
            )],
        }
        .build(cx, |cx| {
            // // Context toolbar
            // HStack::new(cx, |cx| {
            //     Label::new(cx, "Context toolbar");
            // })
            // .class("context-toolbar")
            // .z_index(15)
            // .child_space(Stretch(1.0))
            // .background_color(Color::rgb(40, 40, 40))
            // .height(Pixels(40.0));

            // HStack::new(cx, |cx| {
                // // Toolbar
                // VStack::new(cx, |cx| {
                //     Button::new(
                //         cx,
                //         |cx| {},
                //         |cx| Icon::new(cx, ICON_POINTER),
                //     )
                //     .class("ghost")
                //     .class("group")
                //     .checked(true);

                //     Button::new(
                //         cx,
                //         |cx| {},
                //         |cx| Icon::new(cx, ICON_PENCIL),
                //     )
                //     .class("ghost")
                //     .class("group");

                //     Button::new(
                //         cx,
                //         |cx| {},
                //         |cx| Icon::new(cx, ICON_SLICE),
                //     )
                //     .class("ghost")
                //     .class("group");

                //     Button::new(
                //         cx,
                //         |cx| {},
                //         |cx| Icon::new(cx, ICON_SEARCH),
                //     )
                //     .class("ghost")
                //     .class("group");
                // })
                // .class("toolbar")
                // .background_color(Color::rgb(40, 40, 40))
                // .z_index(10);

                ScrollView::new(cx, 0.0, 0.0, false, false, |cx| {
                    // ZStack::new(cx, |cx|{
                    // Grid + Notes
                    GridView::new(cx, |cx| {
                        //Binding::new(cx, PianoRoll::notes, |cx, notes|{
                        for (idx, note) in PianoRoll::notes.get(cx).iter().enumerate() {
                            let note = note.clone();
                            Binding::new(cx, GridView::root, move |cx, grid|{
                                let grid = grid.get(cx);
                                if time_to_pos(
                                    note.start,
                                    note.end,
                                    grid.start,
                                    grid.end,
                                ).is_some() {
                                    NoteView::new(cx, idx).bind(
                                        PianoRoll::notes.index(idx),
                                        move |handle, note| {
                                            let note_data = note.get(&handle);
                                            let (posx, width) = time_to_pos(
                                                note_data.start,
                                                note_data.end,
                                                grid.start,
                                                grid.end,
                                            )
                                            .unwrap();
                                            let (posy, height) = pitch_to_pos(note_data.pitch);
                        
                                                handle
                                                .left(Pixels(60.0 + posx))
                                                .top(Pixels(posy))
                                                .height(Pixels(height))
                                                .width(Pixels(width));
                                        },
                                    );
                                }
                            });
                        }

                        //});
                    })
                    .height(Pixels(1230.0));

                    // Piano
                    PianoView::new(cx)
                        .height(Pixels(1230.0))
                        .width(Pixels(60.0))
                        .position_type(PositionType::SelfDirected);

                    // }).height(Auto);
                });
            // })
            // .col_between(Pixels(1.0));
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
                self.notes[*index].pitch.decrement();
            }

            PianoRollEvent::MoveUp(index) => {
                self.notes[*index].pitch.increment();
            }

            _ => {}
        });
    }
}

pub struct PianoView {}

impl PianoView {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |cx| {})
    }
}

impl View for PianoView {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();

        let mut path = cx.build_path();
        cx.draw_background(canvas, &mut path);

        let scale = cx.scale_factor();

        let white_key_height1 = 24.0 * scale * 1.5;
        let white_key_height2 = 23.0 * scale * 1.5;
        let black_key_height = 14.0 * scale * 1.5;
        let black_key_gap = 13.0 * scale * 1.5;

        let radius = 4.0 * scale;

        let white_key_fill = vg::Paint::color(
            vg::Color::hex("#C8C8C8"),
        );

        let mut white_key_stroke = vg::Paint::color(
            vg::Color::hex("#000000"),
        );

        white_key_stroke.set_line_width(0.5 * scale);

        let mut white_key_path_1 = vg::Path::new();
        white_key_path_1.rounded_rect_varying(
            bounds.x,
            bounds.y,
            bounds.w,
            white_key_height1,
            0.0,
            radius,
            radius,
            0.0,
        );

        let mut white_key_path_2 = vg::Path::new();
        white_key_path_2.rounded_rect_varying(
            bounds.x,
            bounds.y,
            bounds.w,
            white_key_height2,
            0.0,
            radius,
            radius,
            0.0,
        );

        // Draw white keys
        for i in 0..5 {
            canvas.save();
            canvas.translate(
                0.0,
                i as f32 * (3.0 * white_key_height1 + 4.0 * white_key_height2),
            );
            canvas.fill_path(&mut white_key_path_1, &white_key_fill);
            canvas.stroke_path(&mut white_key_path_1, &white_key_stroke);
            canvas.translate(0.0, white_key_height1);
            canvas.fill_path(&mut white_key_path_2, &white_key_fill);
            canvas.stroke_path(&mut white_key_path_2, &white_key_stroke);
            canvas.translate(0.0, white_key_height2);
            canvas.fill_path(&mut white_key_path_2, &white_key_fill);
            canvas.stroke_path(&mut white_key_path_2, &white_key_stroke);
            canvas.translate(0.0, white_key_height2);
            canvas.fill_path(&mut white_key_path_1, &white_key_fill);
            canvas.stroke_path(&mut white_key_path_1, &white_key_stroke);
            canvas.translate(0.0, white_key_height1);
            canvas.fill_path(&mut white_key_path_2, &white_key_fill);
            canvas.stroke_path(&mut white_key_path_2, &white_key_stroke);
            canvas.translate(0.0, white_key_height2);
            canvas.fill_path(&mut white_key_path_1, &white_key_fill);
            canvas.stroke_path(&mut white_key_path_1, &white_key_stroke);
            canvas.translate(0.0, white_key_height1);
            canvas.fill_path(&mut white_key_path_2, &white_key_fill);
            canvas.stroke_path(&mut white_key_path_2, &white_key_stroke);
            canvas.restore();
        }

        let mut black_key_path = vg::Path::new();
        black_key_path.rounded_rect_varying(
            bounds.x,
            bounds.y,
            0.64 * bounds.w,
            black_key_height,
            0.0,
            radius,
            radius,
            0.0,
        );

        let black_key_fill = vg::Paint::color(
            vg::Color::hex("#070707")
        );

        // Draw black keys
        for i in 0..5 {
            canvas.save();
            canvas.translate(
                0.0,
                i as f32 * (3.0 * white_key_height1 + 4.0 * white_key_height2),
            );
            canvas.translate(0.0, black_key_gap);
            canvas.fill_path(&mut black_key_path, &black_key_fill);
            canvas.translate(0.0, black_key_gap + black_key_height);
            canvas.fill_path(&mut black_key_path, &black_key_fill);
            canvas.translate(0.0, black_key_gap + black_key_height);
            canvas.fill_path(&mut black_key_path, &black_key_fill);
            canvas.translate(0.0, black_key_gap + black_key_height);
            canvas.translate(0.0, black_key_height);
            canvas.fill_path(&mut black_key_path, &black_key_fill);
            canvas.translate(0.0, black_key_height + black_key_height);
            canvas.fill_path(&mut black_key_path, &black_key_fill);
            canvas.translate(0.0, black_key_height + black_key_height);
            canvas.restore();
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub enum GridSpacing {
    Bars,
    Beats,
    Three,
    Four,
    Six,
    Eight,
    Sixteen,
    ThirtyTwo,
    SixtyFour,
}

#[derive(Lens, Data, Clone)]
pub struct GridView {
    // TODO: Move this up
    pub start: MusicalTime,
    pub end: MusicalTime,
    pub clip_start: MusicalTime,
    pub clip_end: MusicalTime,
}

impl GridView {
    pub fn new(cx: &mut Context, content: impl Fn(&mut Context)) -> Handle<Self> {
        Self {
            start: MusicalTime::from_beats(0),
            end: MusicalTime::from_beats(9),
            clip_start: MusicalTime::from_beats(0),
            clip_end: MusicalTime::from_beats(20),
        }
        .build(cx, |cx| {
            (content)(cx);
        })
    }
}

impl View for GridView {

    fn element(&self) -> Option<&'static str> {
        Some("gridview")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseScroll(x, y) => {
                if cx.modifiers().contains(Modifiers::CTRL) {
                    if *y < 0.0 {
                        self.end = self.end.checked_sub(MusicalTime::from_128th_beats(0, 1) * (y.abs() * 10.0) as u32).unwrap_or_default();
                    } else if *y > 0.0 {
                        self.end += MusicalTime::from_128th_beats(0, 1) * (y.abs() * 10.0) as u32;
                    }

                    // Consume the event to stop the scrollview 
                    meta.consume();
                } else {
                    if *x > 0.0 {
                        if let Some(new_start) = self.start.checked_sub(MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32) {
                            self.start = new_start;
                            self.end = self.end.checked_sub(MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32).unwrap();
                        }
                        
                    } else if *x < 0.0 {
                        self.start += MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32;
                        self.end += MusicalTime::from_128th_beats(0, 1)  * (x.abs() * 10.0) as u32;
                    }
                }

                cx.needs_redraw();
            }

            _=> {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();

        let white_key_height1 = 24.0 * scale * 1.5;
        let white_key_height2 = 23.0 * scale * 1.5;

        let height1 = 14.0 * scale * 1.5;
        let height2 = 13.0 * scale * 1.5;
        let gap = 0.0 * scale;

        let mut path1 = vg::Path::new();
        path1.rect(bounds.x, bounds.y, bounds.w, height1);

        let mut path2 = vg::Path::new();
        path2.rect(bounds.x, bounds.y, bounds.w, height2);

        // Draw horizontal lanes
        for i in 0..5 {
            let color1 = vg::Color::rgb(20, 20, 20);
            let color2 = vg::Color::rgb(25, 25, 25);
            canvas.save();
            canvas.translate(
                0.0,
                i as f32 * (3.0 * white_key_height1 + 4.0 * white_key_height2),
            );
            canvas.fill_path(&mut path2, &vg::Paint::color(color2));
            canvas.translate(0.0, height2);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path2, &vg::Paint::color(color2));
            canvas.translate(0.0, height2);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path2, &vg::Paint::color(color2));
            canvas.translate(0.0, height2);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path2, &vg::Paint::color(color2));
            canvas.translate(0.0, height2);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color2));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color2));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            let mut line_path = vg::Path::new();
            line_path.move_to(bounds.x, bounds.y);
            line_path.line_to(bounds.x + bounds.w, bounds.y);
            let mut line_paint = vg::Paint::color(vg::Color::rgb(65, 65, 65));
            line_paint.set_line_width(2.0);
            canvas.stroke_path(&mut line_path, &line_paint);

            canvas.restore();
        }

        // Draw vertical lines
        let start = self.start.as_beats_f64();
        let end = self.end.as_beats_f64();
        let duration = self.end.checked_sub(self.start).unwrap();
        let num = duration.as_beats_f64() as f32;
      
        let px_per_beat = (bounds.w - (60.0 * scale)) / (num * scale);

        // let px_per_beat = 100.0;
        let mut lane_x = cx.logical_to_physical(60.0 - self.start.as_beats_f64().fract() as f32 * px_per_beat);

        for index in 0..num as u32 +1 {
            let mut path = vg::Path::new();
            path.move_to(bounds.x + lane_x, bounds.y);
            path.line_to(bounds.x + lane_x, bounds.bottom());
            canvas.stroke_path(
                &mut path,
                &vg::Paint::color(vizia::vg::Color::rgb(255, 10, 10)),
            );
            lane_x += cx.logical_to_physical(px_per_beat);
        }
    }
}

pub struct NoteView {
    index: usize,
    is_dragging: bool,
    down_pos: f32,
}

impl NoteView {
    pub fn new(cx: &mut Context, index: usize) -> Handle<Self> {
        Self {
            index,
            is_dragging: false,
            down_pos: 0.0,
        }
        .build(cx, |cx| {})
    }
}

impl View for NoteView {
    fn element(&self) -> Option<&'static str> {
        Some("noteview")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                if meta.target == cx.current() {
                    self.is_dragging = true;
                    self.down_pos = cx.mouse().left.pos_down.1;
                    cx.capture();
                }
            }

            WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                cx.release();
                self.is_dragging = false;
            }

            WindowEvent::MouseMove(x, y) => {
                let bounds = cx.bounds();
                if self.is_dragging {
                    let dy = y - self.down_pos;
                    // println!("dy: {}", dy);
                    // if *y > bounds.bottom() {
                    //     cx.emit(PianoRollEvent::MoveDown(self.index));
                    //     self.down_pos = *y;
                    // } else if *y < bounds.top() {
                    //     cx.emit(PianoRollEvent::MoveUp(self.index));
                    //     self.down_pos = *y;
                    // }
                    if dy >= 13.0 * 1.5 {
                        cx.emit(PianoRollEvent::MoveDown(self.index));
                        self.down_pos = *y -13.0 * 1.5;
                    } else if dy <= -13.0 * 1.5 {
                        cx.emit(PianoRollEvent::MoveUp(self.index));
                        self.down_pos = *y - 13.0 * 1.5;
                    }
                }
            }

            _ => {}
        });
    }
}

// Convert the pitch of a note to a position on the grid.
fn pitch_to_pos(pitch: Pitch) -> (f32, f32) {
    let note = pitch.note();

    let lane_height1 = 14.0 * 1.5;
    let lane_height2 = 13.0 * 1.5;

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

    let offset_from_start = (start.as_beats_f64() -  grid_start.as_beats_f64()) * px_per_beat;
    let length = (end.as_beats_f64() - start.as_beats_f64()) * px_per_beat;

    Some((offset_from_start as f32, length as f32))
}
