use crate::vectors::Color;
use ::image::{ImageBuffer, RgbImage};
use anyhow::Result;
use image::Rgb;

pub struct Image {
    pub image: RgbImage,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            image: ImageBuffer::new(width, height),
        }
    }
    pub fn width(&self) -> u32 {
        self.image.width()
    }
    pub fn height(&self) -> u32 {
        self.image.height()
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        self.image.put_pixel(
            x,
            (self.height() - 1) - y,
            Rgb([
                (color.x * 255.999) as u8,
                (color.y * 255.999) as u8,
                (color.z * 255.999) as u8,
            ]),
        );
    }
    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let pixel = self.image.get_pixel(x, y);
        Color::new(
            (pixel[0] as f32) / 255.0,
            (pixel[1] as f32) / 255.0,
            (pixel[2] as f32) / 255.0,
        )
    }
    pub fn save(&self, path: &str) -> Result<()> {
        self.image.save(path)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use anyhow::Context;

    use super::*;

    #[test]
    fn test_image_new() {
        let image = Image::new(100, 100);
        assert_eq!(image.width(), 100);
        assert_eq!(image.height(), 100);
    }

    #[test]
    fn test_image_set_pixel() {
        let mut image = Image::new(100, 100);
        image.set_pixel(50, 50, Color::new(1.0, 0.0, 0.0));
        assert_eq!(image.get_pixel(50, 50), Color::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_image_save() -> Result<()> {
        let mut image = Image::new(100, 100);
        image.set_pixel(50, 50, Color::new(1.0, 0.0, 0.0));
        image.save("test.png")?;
        std::fs::remove_file("test.png")
            .with_context(|| "Should never fail unless the previous line failed")?;
        Ok(())
    }
}
