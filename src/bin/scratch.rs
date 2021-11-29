use std::fs::File;
use std::io::Write;
use qrcode::QrCode;
use image::{EncodableLayout, Luma, DynamicImage};

fn main() {
    // Encode some data into bits.
    let code = QrCode::new(b"01234567").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();
    let image = DynamicImage::ImageLuma8(image);
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut bytes, image::ImageOutputFormat::Png);

    File::create("qrcode.png").unwrap().write_all(&bytes).unwrap();
    // Save the image.
    // image.save("qrcode.png").unwrap();
}