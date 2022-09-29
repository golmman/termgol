use crate::common::color::Color;

#[derive(Clone)]
pub struct Cell {
    pub alive: bool,
    pub color: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            alive: false,
            color: Color::default(),
        }
    }
}
