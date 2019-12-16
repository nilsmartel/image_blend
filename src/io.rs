use crate::Color;
use crate::Image;
use crate::Point;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub fn read_image(path: &str) -> Image {
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::EXPAND
    // | Transformations::STRIP_ALPHA`.
    let decoder = png::Decoder::new(File::open(path).unwrap());
    let (info, mut reader) = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut data = vec![0; info.buffer_size()];
    // Read the next frame. Currently this function should only called once.
    // The default options
    reader.next_frame(&mut data).unwrap();

    let dim = Point(info.width as i32, info.height as i32);

    let data = data
        .chunks(4)
        .map(|c| Color::rgba(c[0], c[1], c[2], c[3]))
        .collect();
    Image { dim, data }
}

pub fn save_image(path: &str, img: &Image) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, img.width() as u32, img.height() as u32);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(&data).unwrap(); // Save
}
