use vizia::prelude::*;

use crate::MusicalTime;

#[derive(Lens)]
pub struct PianoRollData {
    pub key_labels: KeyLabels,

    pub clip_duration: MusicalTime,

    pub view_start: MusicalTime,
    pub view_end: MusicalTime,

    pub grid: Grid,
}

impl Model for PianoRollData {}

#[derive(Debug, Default, Data, Clone, Copy, PartialEq, Eq)]
pub enum KeyLabels {
    // Show no labels on keys.
    None,
    // Show labels on every key.
    All,
    // Show labels on just the root note keys.
    #[default]
    Root,
    // Show labels on every white key.
    White,
    // Show labels on every black key.
    Black,
}

#[derive(Debug, Default, Data, Clone, Copy, PartialEq, Eq)]
pub enum Grid {
    Adaptive,
    None,
    B128,
    B64,
    B32,
    B16,
    B8,
    B4,
    B2,
    B1,
    F2,
    F4,
    F8,
    #[default]
    F16,
    F32,
    F64,
    F128,
}

impl Grid {
    pub fn to_musical_time(&self) -> MusicalTime {
        match self {
            Grid::Adaptive => todo!(),
            Grid::None => todo!(),
            Grid::B128 => MusicalTime::from_beats(128),
            Grid::B64 => MusicalTime::from_beats(64),
            Grid::B32 => MusicalTime::from_beats(32),
            Grid::B16 => MusicalTime::from_beats(16),
            Grid::B8 => MusicalTime::from_beats(8),
            Grid::B4 => MusicalTime::from_beats(4),
            Grid::B2 => MusicalTime::from_beats(2),
            Grid::B1 => MusicalTime::from_beats(1),
            Grid::F2 => MusicalTime::from_half_beats(0, 1),
            Grid::F4 => MusicalTime::from_quarter_beats(0, 1),
            Grid::F8 => MusicalTime::from_eighth_beats(0, 1),
            Grid::F16 => MusicalTime::from_sixteenth_beats(0, 1),
            Grid::F32 => MusicalTime::from_32nd_beats(0, 1),
            Grid::F64 => MusicalTime::from_64th_beats(0, 1),
            Grid::F128 => MusicalTime::from_128th_beats(0, 1),
        }
    }
}
