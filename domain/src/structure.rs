use crate::events::*;
use crate::time::*;

pub trait Clip: std::fmt::Debug {
    fn get_events(&self, position: Position) -> Option<&Vec<Event>>;
}

#[derive(Debug)]
pub struct Sequence {
    pub events: Vec<EventsAtPosition>
}

impl Sequence {
    pub fn new() -> Sequence {
        let mut evnts = Vec::new();
        evnts.reserve_exact(1000);
        Sequence { events: evnts }
    }

    pub fn add_events(&mut self, occurs_at: Position, events: Vec<Event>) {
        let x = EventsAtPosition::new(occurs_at, events);
        self.events.push(x);
    }
}

impl Clip for Sequence {
    fn get_events(&self, position: Position) -> Option<&Vec<Event>> {        
        let d = self.events.iter().find(|x| x.occurs_at == position);
        match d {
            Some(x) => Some(&x.events),
            None => None
        }
    }
}




#[derive(Debug)]
pub struct Repeat {
    inner: Option<Box<dyn Clip + 'static>>,
}

impl Clip for Repeat {
    fn get_events(&self, _position: Position) -> Option<&Vec<Event>> {        
        match &self.inner {
            Some(x) => x.get_events(_position),
            None => None
        }
    }
}