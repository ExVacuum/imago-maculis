use std::io::{self, Read, Cursor};
use image::io::Reader;
use image::imageops::colorops::{dither, BiLevel};
use image::DynamicImage;

// Minimum and maximum code points
const CAECUS_MIN: u16 = 0x2800;
const CAECUS_MAX: u16 = 0x28FF;

// Positions of bits on braille characters
const CAECUS_POS: [u8; 8] = [0, 1, 2, 6, 3, 4, 5, 7];

fn main() {

    // Load image from stdin
    let mut image_bytes = Vec::<u8>::new();
    match io::stdin().read_to_end(&mut image_bytes) {
        Ok(_) => (),
        Err(error) => panic!("Failed to get image bytes from stdin: {}", error)
    }
    let image_reader = match Reader::new(Cursor::new(image_bytes)).with_guessed_format() {
        Ok(reader) => reader,
        Err(error) => panic!("Failed to create image reader: {}", error)
    };
    let mut image = match image_reader.decode() {
        Ok(image) => image,
        Err(error) => panic!("Failed to decode image: {}", error)
    };

    // Convert image to grayscale
    image = image.grayscale();
    image = DynamicImage::ImageLuma8(image.to_luma8());
    let mut image_luma = match image.as_mut_luma8() {
        Some(image) => image,
        None => panic!("Failed to get luma image!")
    };

    // Dither image to pure b&w
    dither(&mut image_luma, &BiLevel);

    // Iterate through 2x4 chunks of an image to convert 
    let luma_copy = image_luma.to_owned();
    for py in (0..image.height()).step_by(4) {
        for px in (0..image.width()).step_by(2) { 
            let mut mask = 0u8;
            for ox in 0..2 {
                for oy in 0..4 {
                    if (px + ox) < image.width() && (py + oy) < image.height() {
                        if luma_copy[(px+ox, py+oy)].0[0] == 0xFF {
                            mask |= 1 << (CAECUS_POS[((ox * 4) + oy) as usize]);
                        }
                    }
                }
            }
            print!("{}", caecus_char(mask));
        }
        println!();
    }
}

// Converts a 8-bit value into a braille equivalent
fn caecus_char(mask:u8) -> char {
    return match char::from_u32((CAECUS_MIN + mask as u16) as u32) {
        Some(character) => {
            if character as u16 > CAECUS_MAX {
                panic!("Number too big!")
            }
            character
        },
        None => panic!("Error converting character!")
    }
}
