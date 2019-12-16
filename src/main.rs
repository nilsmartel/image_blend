mod image;
mod io;
mod rect;
mod structure;

use image::{Color, Image};
use io::*;
use rect::RectIter;
use structure::{Point, VectorField};

struct Config {
    from: String,
    to: String,
    output: String,
}

impl Config {
    fn from_args() -> Config {
        let mut args = std::env::args();
        args.next();
        Config {
            from: args.next().unwrap(),
            to: args.next().unwrap(),
            output: args.next().unwrap(),
        }
    }

    fn to(mut self, value: String) -> Config {
        self.to = value;
        self
    }
    fn output(mut self, value: String) -> Config {
        self.output = value;
        self
    }
    fn from(mut self, value: String) -> Config {
        self.from = value;
        self
    }
}

fn main() {
    let config = Config::from_args();
    let source = read_image(&config.from);
    let dest = read_image(&config.to);

    println!("Images read");

    let vf = get_img_vectors(&source, &dest);

    println!("Vectors calculatedImage");

    let data: Vec<Color> = vf
        .field
        .iter()
        .map(|Point(x, y)| Color::rgb(255.min((x / 2) as u8), 255.min((y / 2) as u8), 0))
        .collect();

    let img = Image { dim: vf.dim, data };

    save_image(&config.output, &img);
}

fn get_img_vectors(source: &Image, dest: &Image) -> VectorField {
    if source.dim == Point(2, 2) {
        let field: Vec<Point> = RectIter::new(Point(0, 0), Point(2, 2))
            .map(|pos| {
                let color = source.get(pos);
                similiar_color_direction(color, RectIter::new(Point(0, 0), Point(2, 2)), dest)
            })
            .collect();

        return VectorField {
            dim: Point(2, 2),
            field,
        };
    }

    let field = get_img_vectors(&source.half(), &dest.half());
    let min = Point(0, 0);
    let max = source.dim;
    let field: Vec<Point> = RectIter::new(min, max)
        .map(|p| {
            let p = p + field.get(p.half()) * 2;
            let check_rect = RectIter::new(
                min.max_bounded(p - Point(1, 1)),
                max.min_bounded(p + Point(1, 1)),
            );
            similiar_color_direction(source.get(p), check_rect, dest)
        })
        .collect();

    VectorField { dim: max, field }
}

fn similiar_color_direction(color: Color, path: impl Iterator<Item = Point>, img: &Image) -> Point {
    path.map(|pos| (pos, color.distance(img.get(pos))))
        .fold((Point(0, 0), std::f64::INFINITY), |a, b| {
            if a.1 < b.1 {
                a
            } else {
                b
            }
        })
        .0
}
