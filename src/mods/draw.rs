use std::path::Path;
use image::{RgbImage, ImageBuffer};
use rusttype::Font;
use super::types::SpeedTestResult;


pub fn draw<P: AsRef<Path>>(file_name: P,speed_test_result:&SpeedTestResult) -> Result<(),()> {
    let font_data: &[u8] = include_bytes!("../../fonts/MSYHMONO.ttf");
    let font: Font = Font::try_from_bytes(font_data).unwrap();

    let sorted_results = speed_test_result.sort_vec();
    let mut img: RgbImage = ImageBuffer::from_fn(1024, 34 * ((sorted_results.len() as u32)+3),|x,y|{
        image::Rgb([0,0,0])
    });

    // // println!("{}", 512 - get_str_len(&text) * 100 / 2);
    // // let x = 512 - get_str_len(&text) * 50 / 2;
    // // draw_text_mut(
    // //     &mut img,
    // //     Rgb::from([255, 0, 0]),
    // //     x,
    // //     10,
    // //     Scale::uniform(100.0),
    // //     &font,
    // //     &text,
    // // );

    img.save(file_name.as_ref()).unwrap();
    Ok(())
}

fn get_str_len(text: &str) -> i32 {
    let mut len = 0;
    for one_char in text.chars() {
        if one_char.is_ascii() {
            len += 1;
            // println!("{} is ascii",one_char);
        }else {
            len += 2;
            // println!("{} is None",one_char);
        }
    }
    len
}