use std::ops::{Add, Mul, Sub};
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd)]
pub struct Point(pub i32, pub i32);
impl Add for Point {
    type Output = Self;
    fn add(self, p: Point) -> Self::Output {
        Point(self.0 + p.0, self.1 + p.1)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, p: Point) -> Self::Output {
        Point(self.0 - p.0, self.1 - p.1)
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, s: i32) -> Self::Output {
        Point(self.0 * s, self.1 * s)
    }
}

impl Point {
    #[inline]
    pub fn half(self) -> Point {
        Point(self.0 >> 1, self.1 >> 1)
    }

    #[inline]
    pub fn width(self) -> i32 {
        self.0
    }

    #[inline]
    pub fn height(self) -> i32 {
        self.1
    }

    #[inline]
    pub fn distance(self, p: Point) -> f64 {
        let abs = self - p;

        ((abs.0 * abs.0 + abs.1 * abs.1) as f64).sqrt()
    }

    #[inline]
    pub fn min_bounded(self, p: Point) -> Point {
        Point(self.0.min(p.0), self.1.min(p.1))
    }

    #[inline]
    pub fn max_bounded(self, p: Point) -> Point {
        Point(self.0.max(p.0), self.1.max(p.1))
    }
}

pub struct VectorField {
    pub dim: Point,
    pub field: Vec<Point>,
}

impl VectorField {
    pub fn get(&self, index: Point) -> Point {
        self.field[(index.1 * self.width() + index.0) as usize]
    }
    #[inline]
    pub fn width(&self) -> i32 {
        self.dim.width()
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.dim.height()
    }
}
