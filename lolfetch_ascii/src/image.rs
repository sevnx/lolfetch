use image::{GenericImageView, ImageError};
use lolfetch_color::{ColoredChar, ColoredString};
use rayon::prelude::*;
use reqwest::Client;
use std::path::Path;
use termcolor::Color;
use thiserror::Error;

pub type ColoredArt = Vec<ColoredString>;

/// Error type for the ASCII art processing
#[derive(Error, Debug)]
pub enum ArtProcessingError {
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

/// Creates an ASCII art of an image from a path
pub fn from_file_path(path: &Path, width: u32, height: u32) -> Result<ColoredArt, ImageError> {
    let image = image::open(path)?;
    Ok(image_to_ascii(&image, width, height))
}

/// Creates an ASCII art of an image from a URL
pub async fn from_url(
    url: &str,
    width: u32,
    height: u32,
) -> Result<ColoredArt, ArtProcessingError> {
    let response = Client::new().get(url).send().await?;
    let image_data = response.bytes().await?;
    let image = image::load_from_memory(&image_data)?;

    Ok(image_to_ascii(&image, width, height))
}

fn image_to_ascii(image: &image::DynamicImage, width: u32, height: u32) -> ColoredArt {
    // Based on tests, brightening the image gives better results (could be a skill issue)
    const BRIGHTNESS_FACTOR: i32 = 45;
    let brightened_image = image.brighten(BRIGHTNESS_FACTOR);

    let resized_image =
        brightened_image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);

    let (img_width, img_height) = resized_image.dimensions();

    (0..img_height)
        .into_par_iter()
        .map(|y| {
            ColoredString::from(
                (0..img_width)
                    .map(|x| ColoredChar::from_pixel(resized_image.get_pixel(x, y)))
                    .collect::<Vec<ColoredChar>>(),
            )
        })
        .collect()
}

pub trait FromPixelToColoredChar {
    fn from_pixel(pixel: image::Rgba<u8>) -> Self;
}

impl FromPixelToColoredChar for ColoredChar {
    fn from_pixel(pixel: image::Rgba<u8>) -> Self {
        const ASCII_CHARS: &[char] = &[
            '@', '#', '$', 'S', '%', '*', '+', ';', '-', ':', ',', '.', '\'', '"',
        ];
        const ALPHA_THRESHOLD: u8 = 128;
        const ASCII_CHARS_LEN: usize = ASCII_CHARS.len();

        let [r, g, b, a] = pixel.0;

        if a < ALPHA_THRESHOLD {
            return Self::new(' ', None, None);
        }

        // Brightness formula from : https://stackoverflow.com/a/596243
        let brightness = (299 * u32::from(r) + 587 * u32::from(g) + 114 * u32::from(b)) / 1000;

        let ascii_index = ((255 - brightness) * (ASCII_CHARS_LEN - 1) as u32 / 255) as usize;

        Self::new(ASCII_CHARS[ascii_index], Some(Color::Rgb(r, g, b)), None)
    }
}

#[cfg(test)]
mod tests {
    use crate::ascii_art;

    /// Test for fetching an image from a URL and converting it to ASCII art
    #[test]
    fn test_proper_fetching() {
        ascii_art::from_url(
            "https://static.wikia.nocookie.net/leagueoflegends/images/f/f8/Season_2023_-_Iron.png/revision/latest",
            50,
            25,
        )
        .then(|result| {
            assert!(result.is_ok());
            match result {
                Ok(art) => {
                    assert_eq!(art.len(), 25);
                    assert_eq!(art[0].len(), 50);
                }
                Err(_) => unreachable!(),
            }
            Ok(())
        })
    }
}
