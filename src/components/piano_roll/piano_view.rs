use vizia::prelude::*;

use crate::{Note, Pitch, MAX_OCTAVE, MIN_OCTAVE};
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
                    .class("pianoview-black-keys");
                })
                .class("pianoview-octave");
            }
        })
        .hoverable(false)
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
            if pitch.note() == Note::C {
                Label::new(cx, &format!("C{}", pitch.octave()));
            }
        })
        .hoverable(true)
        .height(Pixels(note_to_key_height(pitch.note())))
    }
}

impl<SL> View for PianoTile<SL>
where
    SL: Lens<Target = f32>,
{
    fn element(&self) -> Option<&'static str> {
        Some("pianotile")
    }
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
