use std::fmt;

pub struct Game {
    pub title: String,
    pub id: u32,
    pub vars: Vec<String>
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}