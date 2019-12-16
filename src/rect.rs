use crate::Point;

pub struct RectIter {
    start: Point,
    end: Point,
    current: Point,
}

impl Iterator for RectIter {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.current.width() < self.end.width() {
            let p = self.current;
            self.current.0 += 1;
            return Some(p);
        }
        self.current.1 += 1;
        self.current.0 = self.start.0;

        if self.current.height() < self.end.height() {
            return self.next();
        }

        None
    }
}

impl RectIter {
    pub fn new(start: Point, end: Point) -> RectIter {
        RectIter {
            start,
            end,
            current: start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Point;
    use super::RectIter;

    #[test]
    fn rect_iter() {
        let data = RectIter::new(Point(1, 1), Point(4, 3)).collect::<Vec<Point>>();
        let verification = vec![
            Point(1, 1),
            Point(2, 1),
            Point(3, 1),
            Point(1, 2),
            Point(2, 2),
            Point(3, 2),
        ];
        assert_eq!(data, verification);
    }
}
