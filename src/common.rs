use std::cmp::max;
use std::cmp::min;
use std::marker::PhantomData;
use std::ops::Add;
use std::ops::Sub;

pub const TILE_SIZE: ScreenPoint = ScreenPoint::new(3, 1);
pub const FRAMES_PER_SECOND: u16 = 8;

pub type ScreenPoint = Point<ScreenCoordinate>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MapCoordinate;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ScreenCoordinate;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point<W> {
    phantom: PhantomData<*const W>,
    pub x: i32,
    pub y: i32,
}

impl<W> Point<W> {
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            phantom: PhantomData,
            x,
            y,
        }
    }

    pub const fn width(&self) -> i32 {
        self.x
    }

    pub const fn height(&self) -> i32 {
        self.y
    }

    pub fn left(&self) -> Self {
        Self::new(self.x - 1, self.y)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x + 1, self.y)
    }

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y - 1)
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }
}

impl<W> std::fmt::Debug for Point<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl<W> Add for Point<W> {
    type Output = Point<W>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<W> Add for &Point<W> {
    type Output = Point<W>;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<W> Sub for Point<W> {
    type Output = Point<W>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<W> Sub for &Point<W> {
    type Output = Point<W>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug)]
pub struct RectAbsolute<W> {
    pub x1: W,
    pub y1: W,
    pub x2: W,
    pub y2: W,
}

pub fn intersect(r1: &RectAbsolute<i32>, r2: &RectAbsolute<i32>) -> RectAbsolute<i32> {
    let x1 = max(r1.x1, r2.x1);
    let y1 = max(r1.y1, r2.y1);
    let x2 = min(r1.x2, r2.x2);
    let y2 = min(r1.y2, r2.y2);

    RectAbsolute { x1, y1, x2, y2 }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_calc_array_bounds() {}
}
