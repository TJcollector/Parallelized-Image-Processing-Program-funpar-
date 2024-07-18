use image::{GenericImageView, ImageBuffer, RgbImage};
use rayon::prelude::*;
pub fn brightness(s:&str){
    // Load the image
    let img = image::open(s).unwrap();
    let (width, height) = img.dimensions();
    let mut output_img: RgbImage = ImageBuffer::new(width, height);

    // Define the brightening factor (e.g., 1.5 for 50% brighter)
    let brightening_factor = 1.5;

    // Convert image to a parallel iterator
    output_img
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let original_pixel = img.get_pixel(x, y);

            // Apply the brightening factor to each color channel
            pixel[0] = (original_pixel[0] as f32 * brightening_factor).clamp(0.0, 255.0) as u8;
            pixel[1] = (original_pixel[1] as f32 * brightening_factor).clamp(0.0, 255.0) as u8;
            pixel[2] = (original_pixel[2] as f32 * brightening_factor).clamp(0.0, 255.0) as u8;
        });

    // Save the brightened image
    output_img.save("brightimage.png").unwrap();
}