use image::{GenericImageView, ImageBuffer, RgbImage};
use rayon::prelude::*;
pub fn blur(s:&str){
    // Load the image
    let img = image::open(s).unwrap();
    let (width, height) = img.dimensions();
    let mut output_img: RgbImage = ImageBuffer::new(width, height);
    // blur radius (default at 5), increase the 'blurness' of the picture
    let blur_radius = 5;
    output_img
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut r_sum = 0; let mut g_sum = 0;
            let mut b_sum = 0; let mut count = 0; //num of neighbor pixel
            // Iterate over the neighboring pix
            for dx in -(blur_radius as i32)..=(blur_radius as i32) {
                for dy in -(blur_radius as i32)..=(blur_radius as i32) {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 { //avoid accessing invalid memory locations or causing out-of-bounds errors.
                        let neighbor_pixel = img.get_pixel(nx as u32, ny as u32);
                        r_sum += neighbor_pixel[0] as u32;
                        g_sum += neighbor_pixel[1] as u32;
                        b_sum += neighbor_pixel[2] as u32;
                        count += 1;
                    }
                }
            }
            // Calculate the average color
            pixel[0] = (r_sum / count) as u8; //red
            pixel[1] = (g_sum / count) as u8; //green
            pixel[2] = (b_sum / count) as u8; //blue
        });
    // Save the blurred image
    output_img.save("blurredimage.jpg").unwrap();
    println!("Done")
}
