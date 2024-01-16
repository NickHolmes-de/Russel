use std::ops::{Add, Sub, Mul, Div};

// Status: Complete.

const PPQN : u32 = 960;

#[derive(Copy, Clone, Eq, Debug)]
pub struct Position(u32);

impl Position {
    pub fn from_ticks(ticks: u32) -> Self {
        Position(ticks)
    }

    pub fn from_bars(bars: u32, beats: u32, ticks: u32) -> Self {
        Position(ticks + beats * PPQN + bars * PPQN * 4)
    }

    pub fn get_total_ticks(self) -> u32 {
        self.0
    }

    pub fn get_bars(self) -> u32 {
        self.0 / (PPQN * 4)
    }

    pub fn get_beats(self) -> u32 {
        (self.0 % (PPQN * 4)) / PPQN
    }

    pub fn get_ticks(self) -> u32 {
        self.0 % PPQN
    }
}

impl Add<Duration> for Position {
    type Output = Position;

    #[inline(always)]
    fn add(self, other: Duration) -> Position {
        Position(self.0 + other.0)
    }
}

impl Sub<Duration> for Position {
    type Output = Position;

    fn sub(self, other: Duration) -> Position {
        Position(self.0 - other.0)
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.0 == other.0
    }
}

#[derive(Copy, Clone, Eq, Debug)]
pub struct Duration(u32);

impl Duration {
    // add methods to return common music durations, e.g. quarter note, half note, etc.
    pub fn new(duration: u32) -> Self { Duration(duration) }
    pub fn half() -> Self{ Duration(PPQN * 2) }
    pub fn whole() -> Self { Duration(PPQN * 4) }
    pub fn quarter() -> Self { Duration(PPQN) }    
    pub fn eighth() -> Self { Duration(PPQN / 2) }
    pub fn sixteenth() -> Self { Duration(PPQN / 4) }
    pub fn thirtysecond() -> Self { Duration(PPQN / 8) }
    pub fn sixtyfourth() -> Duration { Duration(PPQN / 16) }
/*     pub fn dotted() -> Self { Duration {  PPQN * 3 / 2 } }
    pub fn triplet() -> Self { Duration {  PPQN * 2 / 3 } }
    pub fn quintuplet() -> Self { Duration {  PPQN * 2 / 5 } }
    pub fn septuplet() -> Self { Duration {  PPQN * 2 / 7 } }
    pub fn thirtysecond_triplet() -> Self { Duration {  PPQN / 12 } }
    pub fn sixtyfourth_triplet() -> Self { Duration {  PPQN / 24 } }
    pub fn thirtysecond_quintuplet() -> Self { Duration {  PPQN / 20 } }
    pub fn sixtyfourth_quintuplet() -> Self { Duration {  PPQN / 40 } }
    pub fn thirtysecond_septuplet() -> Self { Duration {  PPQN / 28 } }
    pub fn sixtyfourth_septuplet() -> Self { Duration {  PPQN / 56 } }  */   
}

impl Add<Duration> for Duration {
    type Output = Duration;

    fn add(self, other: Duration) -> Duration {
        Duration(self.0 + other.0)
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;

    fn mul(self, other: u32) -> Duration {
        Duration(self.0 * other)
    }
}

impl Div<u32> for Duration {
    type Output = Duration;

    fn div(self, other: u32) -> Duration {
        Duration(self.0 / other)
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        self.0 == other.0
    }
}

mod tests {
    #[test]
    fn test_position_from_ticks() {
        use super::*;
        let actual = Position::from_ticks(123);
        assert_eq!(actual.get_total_ticks(), 123);
    }
    
    #[test]
    fn test_position_from_bars() {
        use super::*;
        let actual = Position::from_bars(1, 2, 3);
        assert_eq!(actual.get_total_ticks(), 3 + 6 * PPQN);
    }

    #[test]
    fn test_position_get_bars_beat_ticks() {
        use super::*;
        let actual = Position::from_bars(3, 3, 400);
        assert_eq!(actual.get_bars(), 3);
        assert_eq!(actual.get_beats(), 3);
        assert_eq!(actual.get_ticks(), 400);
    }    

    #[test]
    fn test_position_add_duration() {
        use super::*;
        let pos = Position::from_ticks(0);
        let dur = Duration::quarter();
        let actual = pos + dur;
        assert_eq!(actual.get_total_ticks(), 960);
    }

    #[test]
    fn test_position_sub_duration() {
        use super::*;
        let pos = Position::from_ticks(960);
        let dur = Duration::quarter();
        let actual = pos - dur;
        assert_eq!(actual.get_total_ticks(), 0);
    }

    #[test]
    fn test_duration_new() {
        use super::*;
        let actual = Duration::new(123);
        assert_eq!(actual.0, 123);
    }

    #[test]
    fn test_duration_half() {
        use super::*;
        let actual = Duration::half();
        assert_eq!(actual.0, PPQN * 2);
    }

    #[test]
    fn test_duration_whole() {
        use super::*;
        let actual = Duration::whole();
        assert_eq!(actual.0, PPQN * 4);
    }

    #[test]
    fn test_duration_quarter() {
        use super::*;
        let actual = Duration::quarter();
        assert_eq!(actual.0, PPQN);
    }
}
