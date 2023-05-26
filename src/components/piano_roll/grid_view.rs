use vizia::icons::{ICON_PENCIL, ICON_SEARCH, ICON_SLICE};
use vizia::vg;
use vizia::{icons::ICON_POINTER, prelude::*};

use crate::MusicalTime;

pub struct GridData {
    start: MusicalTime,
    end: MusicalTime,
    pixels_per_beat: f32,
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
                        self.end = self
                            .end
                            .checked_sub(
                                MusicalTime::from_128th_beats(0, 1) * (y.abs() * 10.0) as u32,
                            )
                            .unwrap_or_default();
                    } else if *y > 0.0 {
                        self.end += MusicalTime::from_128th_beats(0, 1) * (y.abs() * 10.0) as u32;
                    }

                    // Consume the event to stop the scrollview
                    meta.consume();
                } else {
                    if *x > 0.0 {
                        if let Some(new_start) = self.start.checked_sub(
                            MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32,
                        ) {
                            self.start = new_start;
                            self.end = self
                                .end
                                .checked_sub(
                                    MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32,
                                )
                                .unwrap();
                        }
                    } else if *x < 0.0 {
                        self.start += MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32;
                        self.end += MusicalTime::from_128th_beats(0, 1) * (x.abs() * 10.0) as u32;
                    }
                }

                cx.needs_redraw();
            }

            _ => {}
        });
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let scale = cx.scale_factor();

        let white_key_height1 = 24.0 * scale;
        let white_key_height2 = 23.0 * scale;

        let height1 = 14.0 * scale;
        let height2 = 13.0 * scale;
        let gap = 0.0 * scale;

        let mut path1 = vg::Path::new();
        path1.rect(bounds.x, bounds.y, bounds.w, height1);

        let mut path2 = vg::Path::new();
        path2.rect(bounds.x, bounds.y, bounds.w, height2);

        // Draw horizontal lanes
        for i in 0..8 {
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
            canvas.fill_path(&mut path1, &vg::Paint::color(color2));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color2));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color1));
            canvas.translate(0.0, height1);
            canvas.fill_path(&mut path1, &vg::Paint::color(color2));
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
        let mut lane_x =
            cx.logical_to_physical(60.0 - self.start.as_beats_f64().fract() as f32 * px_per_beat);

        for index in 0..num as u32 + 1 {
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
