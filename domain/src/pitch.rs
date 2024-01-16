use std::ops::Add;

// Status: Complete.

#[derive(Copy, Clone, Eq, Debug)]
pub enum PitchClass {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B
}

impl PitchClass {
    pub fn from_pitch(pitch: Pitch) -> PitchClass {
        match pitch.get_midi_note() % 12 {
            0 => PitchClass::C,
            1 => PitchClass::CSharp,
            2 => PitchClass::D,
            3 => PitchClass::DSharp,
            4 => PitchClass::E,
            5 => PitchClass::F,
            6 => PitchClass::FSharp,
            7 => PitchClass::G,
            8 => PitchClass::GSharp,
            9 => PitchClass::A,
            10 => PitchClass::ASharp,
            11 => PitchClass::B,
            _ => panic!("Invalid pitch class"), // this case can't actually happen (mod 12 will always be 0-11), but the compiler doesn't know that
        }
    }
}

impl PartialEq for PitchClass {
    fn eq(&self, other: &PitchClass) -> bool {
        *self as u8 == *other as u8
    }
}

#[derive(Copy, Clone, Eq, Debug)]
pub enum Interval {
    Unison, MinorSecond, MajorSecond, MinorThird, MajorThird, PerfectFourth, Tritone, PerfectFifth, MinorSixth, MajorSixth, MinorSeventh, MajorSeventh, Octave
}

impl PartialEq for Interval {
    fn eq(&self, other: &Interval) -> bool {
        *self as u8 == *other as u8
    }
}

#[derive(Copy, Clone, Eq, Debug)]
pub struct Pitch(u8);

impl Pitch {
    pub fn new(pitch_class: PitchClass, octave: u8) -> Self {
        Pitch(pitch_class as u8 + octave * 12)
    }

    pub fn get_midi_note(self) -> u8 {
        self.0
    }
}

impl PartialEq for Pitch {
    fn eq(&self, other: &Pitch) -> bool {
        self.0 == other.0
    }
}

impl Add<Interval> for Pitch {
    type Output = Pitch;

    fn add(self, interval: Interval) -> Pitch {
        Pitch(self.0 + interval as u8)
    }
}

mod tests {
    #[test]
    pub fn test_pitch_class_from_pitch() {
        use super::*;
        let actual = PitchClass::from_pitch(Pitch::new(PitchClass::C, 4));
        assert_eq!(actual, PitchClass::C);
    }
}