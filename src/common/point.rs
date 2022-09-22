use std::ops::Add;
use std::ops::Sub;

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
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

    pub fn up_left(&self) -> Self {
        Self::new(self.x - 1, self.y - 1)
    }

    pub fn up_right(&self) -> Self {
        Self::new(self.x + 1, self.y - 1)
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y + 1)
    }

    pub fn down_left(&self) -> Self {
        Self::new(self.x - 1, self.y + 1)
    }

    pub fn down_right(&self) -> Self {
        Self::new(self.x + 1, self.y + 1)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_calc_array_bounds() {}
}
