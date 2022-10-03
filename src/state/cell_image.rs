use crate::common::point::Point;

use super::cell_setup::CellSetup;

const DEAD_CELL_CHARS: &str = ". ";

#[derive(Debug)]
pub struct CellImage {
    pub living_points: Vec<Point>,
    pub size: Point,
}

impl From<&str> for CellImage {
    fn from(s: &str) -> Self {
        let mut living_points = Vec::new();
        let mut width = 0;

        let mut y = 0;
        for line in s.split("\n") {
            if line.starts_with("!") {
                continue;
            }

            if width < line.len() as i32 {
                width = line.len() as i32;
            }

            let mut x = 0;
            for c in line.chars() {
                if DEAD_CELL_CHARS.find(c).is_none() {
                    living_points.push(Point::new(x, y));
                }
                x += 1;
            }
            y += 1;
        }

        CellImage {
            living_points,
            size: Point::new(width, y),
        }
    }
}

impl From<CellSetup> for CellImage {
    fn from(cell_setup: CellSetup) -> Self {
        let s: String = cell_setup.into();
        CellImage::from(s.as_str())
    }
}
