use vizia::icons::{ICON_PENCIL, ICON_SEARCH, ICON_SLICE};
use vizia::vg;
use vizia::{icons::ICON_POINTER, prelude::*};

use crate::{MusicalTime, PianoRollData};

#[derive(Lens, Data, Clone)]
pub struct GridView {
    // TODO: Move this up
    // pub start: MusicalTime,
    // pub end: MusicalTime,
    // pub clip_start: MusicalTime,
    // pub clip_end: MusicalTime,
}

impl GridView {
    pub fn new(cx: &mut Context, content: impl Fn(&mut Context)) -> Handle<Self> {
        Self {
            // start: MusicalTime::from_beats(0),
            // end: MusicalTime::from_beats(9),
            // clip_start: MusicalTime::from_beats(0),
            // clip_end: MusicalTime::from_beats(20),
        }
        .build(cx, |cx| {
            (content)(cx);
        })
        // .bind(Self::start, |handle, start|{
        //     handle.bind(Self::end, move |handle, end|{
        //         let s = start.get(&handle).as_beats_f64();
        //         let e = end.get(&handle).as_beats_f64();
        //         let duration = end.get(&handle).checked_sub(start.get(&handle)).unwrap();
        //         let num = duration.as_beats_f64() as f32;

        //         let scale = 1.0;

        //         let width = (100.0 * (num * scale) ) + (70.0 * scale);

        //         handle.width(Pixels(width));
        //     });
        // })
    }
}

impl View for GridView {
    fn element(&self) -> Option<&'static str> {
        Some("gridview")
    }

    // fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
    //     event.map(|window_event, meta| match window_event {
    //         WindowEvent::MouseScroll(x, y) => {
    //             if cx.modifiers().contains(Modifiers::CTRL) {
    //                 if *y < 0.0 {
    //                     self.end = self
    //                         .end
    //                         .checked_sub(
    //                             MusicalTime::from_128th_beats(0, 1) * (y.abs() * 10.0) as u32,
    //                         )
    //                         .unwrap_or_default();
    //                 } else if *y > 0.0 {
    //                     self.end += MusicalTime::from_128th_beats(0, 1) * (y.abs() * 10.0) as u32;
    //                 }

    //                 // Consume the event to stop the scrollview
    //                 meta.consume();
    //             } else {
    //                 if *x > 0.0 {
    //                     if let Some(new_start) = self.start.checked_sub(
    //                         MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32,
    //                     ) {
    //                         self.start = new_start;
    //                         self.end = self
    //                             .end
    //                             .checked_sub(
    //                                 MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32,
    //                             )
    //                             .unwrap();
    //                     }
    //                 } else if *x < 0.0 {
    //                     self.start += MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32;
    //                     self.end += MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32;
    //                 }
    //             }

    //             cx.needs_redraw();
    //         }

    //         _ => {}
    //     });
    // }

    fn draw(&self, cx: &mut DrawContext, canvas: &Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();

        let white_key_height1 = 24.0 * scale;
        let white_key_height2 = 23.0 * scale;

        let height1 = 14.0 * scale;
        let height2 = 13.0 * scale;
        let gap = 0.0 * scale;

        let mut path1 = vg::Path::new();
        path1.add_rect(
            vg::Rect::from_point_and_size((bounds.x, bounds.y), (bounds.w, height1)),
            None,
        );

        let mut path2 = vg::Path::new();
        path2.add_rect(
            vg::Rect::from_point_and_size((bounds.x, bounds.y), (bounds.w, height2)),
            None,
        );

        // Draw horizontal lanes
        for i in 0..8 {
            let color1 = Color::rgb(20, 20, 20);
            let color2 = Color::rgb(25, 25, 25);

            canvas.save();
            canvas.translate((
                0.0,
                i as f32 * (3.0 * white_key_height1 + 4.0 * white_key_height2),
            ));
            let mut paint = vg::Paint::default();
            paint.set_color(color2);
            canvas.draw_path(&path2, &paint);
            canvas.translate((0.0, height2));
            let mut paint = vg::Paint::default();
            paint.set_color(color1);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut paint = vg::Paint::default();
            paint.set_color(color2);
            canvas.draw_path(&path2, &paint);
            canvas.translate((0.0, height2));
            let mut paint = vg::Paint::default();
            paint.set_color(color1);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut paint = vg::Paint::default();
            paint.set_color(color2);
            canvas.draw_path(&path2, &paint);
            canvas.translate((0.0, height2));
            let mut paint = vg::Paint::default();
            paint.set_color(color1);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut paint = vg::Paint::default();
            paint.set_color(color2);
            canvas.draw_path(&path2, &paint);
            canvas.translate((0.0, height2));
            let mut paint = vg::Paint::default();
            paint.set_color(color2);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut paint = vg::Paint::default();
            paint.set_color(color1);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut paint = vg::Paint::default();
            paint.set_color(color2);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut paint = vg::Paint::default();
            paint.set_color(color1);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut paint = vg::Paint::default();
            paint.set_color(color2);
            canvas.draw_path(&path1, &paint);
            canvas.translate((0.0, height1));
            let mut line_path = vg::Path::new();
            line_path.move_to((bounds.x, bounds.y));
            line_path.line_to((bounds.x + bounds.w, bounds.y));
            let mut line_paint = vg::Paint::default();
            line_paint.set_color(Color::rgb(65, 65, 65));
            line_paint.set_stroke_width(2.0);
            line_paint.set_style(vg::PaintStyle::Stroke);
            canvas.draw_path(&mut line_path, &line_paint);

            canvas.restore();
        }

        // Draw vertical lines
        let start = PianoRollData::view_start.get(cx);
        let end = PianoRollData::view_end.get(cx);
        let grid_spacing = PianoRollData::grid.get(cx);
        let duration = end.checked_sub(start).unwrap();
        let duration = duration.as_beats_f64();
        let num = (duration / grid_spacing.to_musical_time().as_beats_f64()).round() as u32;

        let px_per_beat = (bounds.w - (70.0 * scale)) / (duration as f32 * scale);

        // let px_per_beat = 100.0;

        // let px_per_beat = 100.0;

        for index in 0..num + 1 {
            let lane_x = cx.logical_to_physical(
                70.0 + grid_spacing.to_musical_time().as_beats_f64() as f32
                    * px_per_beat
                    * index as f32,
            );
            let mut path = vg::Path::new();
            path.move_to((bounds.x + lane_x, bounds.y));
            path.line_to((bounds.x + lane_x, bounds.bottom()));
            let mut paint = vg::Paint::default();
            paint.set_color(Color::rgb(255, 10, 10));
            paint.set_style(vg::PaintStyle::Stroke);
            canvas.draw_path(&path, &paint);
            // lane_x += cx.logical_to_physical(px_per_beat);
        }
    }
}
