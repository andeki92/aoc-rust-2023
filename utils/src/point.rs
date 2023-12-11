use std::ops::{Add, Sub};

pub const ORIGIN: Point = Point::new(0, 0);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub const fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    pub fn manhattan(self, other: Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

#[cfg(test)]
mod test {
    use crate::point::{Point, ORIGIN};

    #[test]
    fn manhattan_distance_test() {
        assert_eq!(4, ORIGIN.manhattan(Point::new(0, 4)));
        assert_eq!(4, Point::new(0, 4).manhattan(ORIGIN));
        assert_eq!(4, ORIGIN.manhattan(Point::new(0, -4)));
        assert_eq!(3, ORIGIN.manhattan(Point::new(3, 0)));
        assert_eq!(6, ORIGIN.manhattan(Point::new(3, 3)));
        assert_eq!(6, ORIGIN.manhattan(Point::new(-3, -3)));
        assert_eq!(3, Point::new(-2, -3).manhattan(Point::new(-3, -5)));
    }

    #[test]
    fn add_test() {
        assert_eq!(ORIGIN, ORIGIN + ORIGIN);
        assert_eq!(Point::new(0, 4), ORIGIN + Point::new(0, 4));
        assert_eq!(Point::new(0, 4), Point::new(0, 4) + ORIGIN);
        assert_eq!(Point::new(3, 0), ORIGIN + Point::new(3, 0));
        assert_eq!(Point::new(-1, -1), Point::new(-1, -5) + Point::new(0, 4));
    }

    #[test]
    fn sub_test() {
        assert_eq!(ORIGIN, ORIGIN - ORIGIN);
        assert_eq!(Point::new(0, -4), ORIGIN - Point::new(0, 4));
        assert_eq!(Point::new(0, 4), Point::new(0, 4) - ORIGIN);
        assert_eq!(Point::new(-3, 0), ORIGIN - Point::new(3, 0));
        assert_eq!(Point::new(-1, -9), Point::new(-1, -5) - Point::new(0, 4));
    }
}
