use crate::pitch::Pitch;
use crate::time::Duration;
use crate::time::Position;

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Note { note : Pitch, velocity: u8, duration: Duration },
}

impl Event {
    pub fn new(note: Pitch, velocity: u8, duration: Duration) -> Event { Event::Note {note, velocity, duration} }
}

#[derive(Debug)]
pub struct EventsAtPosition {
    pub occurs_at: Position,
    pub events: Vec<Event>
}

impl EventsAtPosition {
    pub fn new(occurs_at: Position, events: Vec<Event>) -> EventsAtPosition {
        EventsAtPosition { occurs_at, events }
    }
}