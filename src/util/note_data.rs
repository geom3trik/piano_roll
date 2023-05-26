use crate::MusicalTime;
use vizia::prelude::*;

pub const MAX_OCTAVE: i8 = 6;
pub const MIN_OCTAVE: i8 = -2;

#[derive(Debug, Lens, Clone, Data)]
pub struct NoteData {
    pitch: Pitch,
    start: MusicalTime,
    end: MusicalTime,
}

impl NoteData {
    pub fn new(pitch: Pitch, start: MusicalTime, end: MusicalTime) -> Self {
        Self { pitch, start, end }
    }

    pub fn increment(&mut self) {
        self.pitch.increment();
    }

    pub fn decrement(&mut self) {
        self.pitch.decrement();
    }
}

#[derive(Debug, Default, Data, Clone, Copy, PartialEq)]
pub struct Pitch {
    octave: i8,
    note: Note,
    fine: f32,
}

impl Pitch {
    pub fn new(octave: i8, note: Note, fine: f32) -> Self {
        Self { octave, note, fine }
    }

    pub fn note(&self) -> Note {
        self.note
    }

    pub fn octave(&self) -> i8 {
        self.octave
    }

    pub fn set_note(&mut self, note: Note) {
        self.note = note;
    }

    pub fn set_octave(&mut self, octave: i8) {
        self.octave = octave;
    }

    pub fn increment(&mut self) {
        if self.note() == Note::B && self.octave < MAX_OCTAVE {
            self.octave += 1;
            self.note = Note::C;
        }
    }

    pub fn decrement(&mut self) {
        if self.note() == Note::C && self.octave > MIN_OCTAVE {
            self.octave -= 1;
            self.note = Note::B;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Data)]
pub enum Note {
    #[default]
    C = 0,
    CS = 1,
    D = 2,
    DS = 3,
    E = 4,
    F = 5,
    FS = 6,
    G = 7,
    GS = 8,
    A = 9,
    AS = 10,
    B = 11,
}

impl Note {
    pub fn is_black_key(&self) -> bool {
        match self {
            Note::CS | Note::DS | Note::FS | Note::GS | Note::AS => true,
            _ => false,
        }
    }

    pub fn next(&self) -> Self {
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

    pub fn prev(&self) -> Self {
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

impl From<Note> for u8 {
    fn from(value: Note) -> Self {
        match value {
            Note::C => 0,
            Note::CS => 1,
            Note::D => 2,
            Note::DS => 3,
            Note::E => 4,
            Note::F => 5,
            Note::FS => 6,
            Note::G => 7,
            Note::GS => 8,
            Note::A => 9,
            Note::AS => 10,
            Note::B => 11,
            _ => unreachable!(),
        }
    }
}

impl From<u8> for Note {
    fn from(value: u8) -> Self {
        match value {
            0 => Note::C,
            1 => Note::CS,
            2 => Note::D,
            3 => Note::DS,
            4 => Note::E,
            5 => Note::F,
            6 => Note::FS,
            7 => Note::G,
            8 => Note::GS,
            9 => Note::A,
            10 => Note::AS,
            11 => Note::B,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Note::C => "C",
            Note::CS => "C#",
            Note::D => "D",
            Note::DS => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FS => "F#",
            Note::G => "G",
            Note::GS => "G#",
            Note::A => "A",
            Note::AS => "A#",
            Note::B => "B",
            _ => unreachable!(),
        })
    }
}
