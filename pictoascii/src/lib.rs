use pyo3::prelude::*;


use image::{DynamicImage, GenericImageView, GrayImage};
use std::path::Path;

struct ASCIIArtGeneratorRust {
    ascii_chars: Vec<char>,
}

impl ASCIIArtGeneratorRust {
    // Constructs a new ASCIIArtGenerator with a given set of ASCII characters
    fn new(ascii_chars: &str) -> Self {
        ASCIIArtGeneratorRust {
            ascii_chars: ascii_chars.chars().collect(),
        }
    }

    // Scales the image to a specified width while maintaining aspect ratio
    fn scale_image(&self, image: &DynamicImage, new_width: u32) -> GrayImage {
        let (width, height) = image.dimensions();
        let aspect_ratio = height as f32 / width as f32;
        let new_height = (new_width as f32 * aspect_ratio) as u32;
        image.resize_exact(new_width, new_height, image::imageops::FilterType::Nearest).to_luma8()
    }

    // Converts the image pixels to ASCII characters
    fn map_pixels_to_ascii(&self, image: &GrayImage) -> String {
        let (width, height) = image.dimensions();
        let mut ascii_art = String::new();
        let ascii_len = self.ascii_chars.len() as u32;
    
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let intensity = pixel.0[0];
                let index = (intensity as u32 * ascii_len / 256) as usize;
                ascii_art.push(self.ascii_chars[index]);
            }
            ascii_art.push('\n');  // Add newline after each row
        }
    
        ascii_art
    }

    // Generates ASCII art from the specified image path
    fn generate_ascii_art(&self, image_path: &Path, width: u32) -> String {
        let img = image::open(image_path).expect("Failed to get value");
        let scaled_image = self.scale_image(&img, width);
        return self.map_pixels_to_ascii(&scaled_image);
        // Ok(self.map_pixels_to_ascii(&scaled_image))
    }
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn convert_img_to_ascii(image_path: String, width: u32) -> String {
    let ascii = ASCIIArtGeneratorRust::new("@%#*+=-:. ");
    return ascii.generate_ascii_art(Path::new(&image_path), width);
}

/// A Python module implemented in Rust.
#[pymodule]
fn pictoascii(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert_img_to_ascii, m)?)?;
    Ok(())
}
