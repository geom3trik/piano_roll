use vizia::prelude::*;

use crate::{Note, PianoRollData, Pitch, MAX_OCTAVE, MIN_OCTAVE};
pub struct PianoView<SL>
where
    SL: Lens<Target = f32>,
{
    scale: SL,
}

impl<SL> PianoView<SL>
where
    SL: Lens<Target = f32>,
{
    pub fn new(cx: &mut Context, scale_lens: SL) -> Handle<Self> {
        Self {
            scale: scale_lens.clone(),
        }
        .build(cx, |cx| {
            for oct in (MIN_OCTAVE..MAX_OCTAVE).rev() {
                ZStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::B, 0.0));
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::A, 0.0));
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::G, 0.0));
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::F, 0.0));
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::E, 0.0));
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::D, 0.0));
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::C, 0.0));
                    })
                    .pointer_events(PointerEvents::None)
                    .class("pianoview-white-keys");

                    VStack::new(cx, |cx| {
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::AS, 0.0))
                            .class("black-key");
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::GS, 0.0))
                            .class("black-key");
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::FS, 0.0))
                            .class("black-key");
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::DS, 0.0))
                            .class("black-key")
                            .class("space-27");
                        PianoTile::new(cx, scale_lens.clone(), Pitch::new(oct, Note::CS, 0.0))
                            .class("black-key")
                            .class("space-14");
                    })
                    .pointer_events(PointerEvents::None)
                    .class("pianoview-black-keys");
                })
                .class("pianoview-octave");
            }
        })
        .hoverable(true)
    }
}

impl<SL> View for PianoView<SL>
where
    SL: Lens<Target = f32>,
{
    fn element(&self) -> Option<&'static str> {
        Some("pianoview")
    }
}

pub enum PianoTileEvent {}
pub struct PianoTile<SL>
where
    SL: Lens<Target = f32>,
{
    scale: SL,
    pitch: Pitch,
}

impl<SL> PianoTile<SL>
where
    SL: Lens<Target = f32>,
{
    pub fn new(cx: &mut Context, scale_lens: SL, pitch: Pitch) -> Handle<Self> {
        Self {
            scale: scale_lens.clone(),
            pitch,
        }
        .build(cx, |cx| {
            let visible = PianoRollData::key_labels.map(move |key_labels| match key_labels {
                crate::KeyLabels::None => false,
                crate::KeyLabels::All => true,
                crate::KeyLabels::Root => pitch.note() == Note::C,
                crate::KeyLabels::White => pitch.note().is_white_key(),
                crate::KeyLabels::Black => pitch.note().is_black_key(),
            });

            Label::new(cx, &format!("{}{}", pitch.note(), pitch.octave())).visibility(visible);
        })
        .pointer_events(PointerEvents::Auto)
        .height(scale_lens.map(move |scale| Pixels(note_to_key_height(pitch.note()) * *scale)))
    }
}

impl<SL> View for PianoTile<SL>
where
    SL: Lens<Target = f32>,
{
    fn element(&self) -> Option<&'static str> {
        Some("pianotile")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {}
}

pub fn note_to_key_height(note: Note) -> f32 {
    if note.is_black_key() {
        14.0
    } else {
        let mut height = 23.0;
        match note {
            Note::D | Note::F | Note::B => height += 1.0,
            _ => {}
        }
        height
    }
}
