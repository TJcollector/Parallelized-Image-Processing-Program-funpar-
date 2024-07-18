use image::{GenericImageView, ImageBuffer, RgbImage};
use rayon::prelude::*;
pub fn Sharpenimage(s:&str){
    // Load the image
    let img = image::open(s).unwrap();
    let (width, height) = img.dimensions();
    let mut output_img: RgbImage = ImageBuffer::new(width, height);
    // Define the sharpen kernel
    let kernel: [[i32; 3]; 3] = [
        [0, -1, 0],
        [-1, 5, -1],
        [0, -1, 0]
    ];
    // Convert image to a parallel iterator
    output_img
        .enumerate_pixels_mut()
        .par_bridge()
        .for_each(|(x, y, pixel)| {
            let mut r_sum = 0;
            let mut g_sum = 0;
            let mut b_sum = 0;
            // Iterate over the kernel
            for ky in 0..3 {
                for kx in 0..3 {
                    let nx = x as i32 + kx - 1;
                    let ny = y as i32 + ky - 1;

                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                        let neighbor_pixel = img.get_pixel(nx as u32, ny as u32);
                        let kernel_value = kernel[ky as usize][kx as usize];

                        r_sum += neighbor_pixel[0] as i32 * kernel_value;
                        g_sum += neighbor_pixel[1] as i32 * kernel_value;
                        b_sum += neighbor_pixel[2] as i32 * kernel_value;
                    }
                }
            }
            // Clamp the values to the valid range [0, 255]
            pixel[0] = r_sum.clamp(0, 255) as u8;
            pixel[1] = g_sum.clamp(0, 255) as u8;
            pixel[2] = b_sum.clamp(0, 255) as u8;
        });
    // Save the sharpened image
    output_img.save("sharpenimage.jpg").unwrap();
}