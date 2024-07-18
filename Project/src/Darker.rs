use image::{GenericImageView, ImageBuffer, RgbImage};
use rayon::prelude::*;
pub fn darken(s:&str){
    // Load the image
    let img = image::open(s).unwrap();
    let (width, height) = img.dimensions();
    let mut output_img: RgbImage = ImageBuffer::new(width, height);

    // Define the darkening factor (e.g., 0.5 for 50% darker)
    let darkening_factor = 0.5;

    // Convert image to a parallel iterator
    output_img
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let original_pixel = img.get_pixel(x, y);

            // Apply the darkening factor to each color channel
            pixel[0] = (original_pixel[0] as f32 * darkening_factor).clamp(0.0, 255.0) as u8;
            pixel[1] = (original_pixel[1] as f32 * darkening_factor).clamp(0.0, 255.0) as u8;
            pixel[2] = (original_pixel[2] as f32 * darkening_factor).clamp(0.0, 255.0) as u8;
        });

    // Save the darkened image
    output_img.save("Darken_image.jpg").unwrap();
}