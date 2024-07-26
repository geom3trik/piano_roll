use vizia::prelude::*;

use crate::{NoteData, PianoRollEvent};

pub struct NoteView {
    index: usize,
    is_dragging: bool,
    down_pos: f32,
}

impl NoteView {
    pub fn new<L: Lens<Target = NoteData>>(
        cx: &mut Context,
        index: usize,
        lens: L,
    ) -> Handle<Self> {
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
                        self.down_pos = *y;
                    } else if dy <= -13.0 * 1.5 {
                        cx.emit(PianoRollEvent::MoveUp(self.index));
                        self.down_pos = *y;
                    }
                }
            }

            _ => {}
        });
    }
}
