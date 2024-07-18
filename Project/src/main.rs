mod sharpen;
mod blurring;
mod Darker;
mod Bright;

use std::time::{SystemTime, UNIX_EPOCH};
use image::{GenericImageView, ImageBuffer, RgbImage};
use rayon::prelude::*;
fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    //Copy and paste your picture path here!!
    let s="src/landscape-test.jpg"; //example
    sharpen::Sharpenimage(s);
    blurring::blur(s);
    Darker::darken(s);
    Bright::brightness(s);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("{:?}", end.as_secs());

}
