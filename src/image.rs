use crate::structure::Point;

#[derive(Copy, Clone, Debug)]
pub struct Color(u8, u8, u8, u8);
impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color(r, g, b, 0xFF)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color(r, g, b, a)
    }

    pub fn distance(self, o: Color) -> f64 {
        let a = self.0 as i32 - o.0 as i32;
        let b = self.1 as i32 - o.1 as i32;
        let c = self.2 as i32 - o.2 as i32;
        let d = self.3 as i32 - o.3 as i32;

        ((a * a + b * b + c * c + d * d) as f64).sqrt()
    }

    fn blend_4(a: Color, b: Color, c: Color, d: Color) -> Color {
        let c1 = ((a.0 as u32 + b.0 as u32 + c.0 as u32 + d.0 as u32) >> 2) as u8;
        let c2 = ((a.1 as u32 + b.1 as u32 + c.1 as u32 + d.1 as u32) >> 2) as u8;
        let c3 = ((a.2 as u32 + b.2 as u32 + c.2 as u32 + d.2 as u32) >> 2) as u8;
        let c4 = ((a.3 as u32 + b.3 as u32 + c.3 as u32 + d.3 as u32) >> 2) as u8;

        Color(c1, c2, c3, c4)
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, c: Color) -> Color {
        Color(self.0 + c.0, self.1 + c.1, self.2 + c.2, self.3 + c.3)
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, c: Color) -> Color {
        Color(self.0 - c.0, self.1 - c.1, self.2 - c.2, self.3 - c.3)
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    pub dim: Point,
    pub data: Vec<Color>,
}

impl Image {
    #[inline]
    pub fn get(&self, index: Point) -> Color {
        self.data[(index.1 * self.width() + index.0) as usize]
    }

    #[inline]
    pub fn width(&self) -> i32 {
        self.dim.width()
    }

    #[inline]
    pub fn height(&self) -> i32 {
        self.dim.height()
    }

    pub fn half(&self) -> Image {
        let dim = Point(self.width() >> 1, self.height() >> 1);
        let mut data = Vec::with_capacity((dim.0 * dim.1) as usize);

        for x in (0..self.width()).step_by(2) {
            for y in (0..self.height()).step_by(2) {
                data.push(Color::blend_4(
                    // TODO one can get around bound checking here
                    self.get(Point(x, y)),
                    self.get(Point(x + 1, y)),
                    self.get(Point(x, y + 1)),
                    self.get(Point(x + 1, y + 1)),
                ))
            }
        }

        Image { dim, data }
    }
}
