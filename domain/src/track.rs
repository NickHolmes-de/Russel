

pub struct Track {
    pub clips: Vec<Clip>
}

impl Track {
    pub fn new() -> Track {
        Track(Vec::new())
    }

    pub fn add_clip(&mut self, clip: Clip) {
        self.clips.push(clip);
    }
}