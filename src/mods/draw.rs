use super::types::{Config, SpeedTestResult, SpeedType};
use chrono::Local;
use image::{ImageBuffer, Rgb, RgbImage};
use imageproc::{
    drawing::{draw_hollow_rect_mut, draw_text_mut},
    rect::Rect,
};
use rusttype::{Font, Scale};
use std::path::Path;

pub fn draw<P: AsRef<Path>>(
    file_name: P,
    config: &Config,
    speed_test_result: &SpeedTestResult,
) -> Result<(), ()> {
    let font_data: &[u8] = include_bytes!("../../fonts/MSYHMONO.ttf");
    let font: Font = Font::try_from_bytes(font_data).unwrap();

    let sorted_results = speed_test_result.sort_vec();
    let mut max_server_len = 0;
    for item in &sorted_results {
        if get_str_len(&item.0)>max_server_len {
            max_server_len = get_str_len(&item.0);
        }
    }
    //println!("{}", max_server_len);
    let wide = max_server_len*12+725+10; //temp

    let mut img: RgbImage = ImageBuffer::from_fn(
        wide as u32,
        34 * ((sorted_results.len() as u32) + 4),
        |_x, _y| {
            // if y == 34 * 2 && x > 10 && x < wide as u32 - 10 {
            //     image::Rgb([0, 0, 0])
            // } else {
                image::Rgb([41, 41, 41])
            //}
        },
    );

    draw_text(
        &mut img,
        &config.title,
        true,
        &[255, 255, 255],
        wide as i32 / 2,
        10,
        25.0,
        &font,
    )
    .unwrap_or_default();
    let dt = Local::now();
    draw_text(
        &mut img,
        &dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        true,
        &[255, 255, 255],
        wide as i32 / 2,
        50,
        18.0,
        &font,
    )
    .unwrap_or_default();
    draw_hollow_rect_mut(
        &mut img,
        Rect::at(10, 34 * 2).of_size(342, 33),
        Rgb([255, 255, 255]),
    );
    draw_text(
        &mut img,
        "安卓",
        true,
        &[255, 255, 255],
        10+342/2,
        34 * 2+9,
        20.0,
        &font,
    )
    .unwrap_or_default();
    draw_hollow_rect_mut(
        &mut img,
        Rect::at(354, 34 * 2).of_size(256, 33),
        Rgb([255, 255, 255]),
    );
    draw_text(
        &mut img,
        "WEB",
        true,
        &[255, 255, 255],
        354+256/2,
        34 * 2+9,
        20.0,
        &font,
    )
    .unwrap_or_default();
    draw_hollow_rect_mut(
        &mut img,
        Rect::at(612, 34 * 2).of_size(111, 65),
        Rgb([255, 255, 255]),
    );
    draw_text(
        &mut img,
        "平均",
        true,
        &[255, 255, 255],
        612+111/2,
        34 * 2+26,
        20.0,
        &font,
    )
    .unwrap_or_default();
    let server_name_len = max_server_len*12;
    draw_hollow_rect_mut(
        &mut img,
        Rect::at(725, 34 * 2).of_size((server_name_len) as u32, 65),
        Rgb([255, 255, 255]),
    );
    draw_text(
        &mut img,
        "地址",
        true,
        &[255, 255, 255],
        (725+server_name_len/2)as i32,
        34 * 2+26,
        20.0,
        &font,
    )
    .unwrap_or_default();
    let area_name = ["CN","HK","TW","TH","CN","HK","TW"];
    for index in 0..7 {
        draw_little_hollow_rect(&mut img, &[255, 255, 255], 10+86*index, 34 * 3+1, 84, 30).unwrap_or_default();
        draw_text(
            &mut img,
            area_name[index as usize],
            true,
            &[255, 255, 255],
            10+86*index+84/2 -1,
            34 * 3+8,
            20.0,
            &font,
        )
        .unwrap_or_default();
    }

    let areas = [
        SpeedType::CnApp,
        SpeedType::HkApp,
        SpeedType::TwApp,
        SpeedType::CnWeb,
        SpeedType::HkWeb,
        SpeedType::TwWeb,
        SpeedType::ThApp,
    ];
    let mut row = 0;
    for item in sorted_results {
        for index in 0..7 {
            draw_little_hollow_rect(&mut img, &[255, 255, 255], 10+86*index, 34 * (4+row), 84, 30).unwrap_or_default();
            let context: String;
            let color: [u8;3];
            match &item.1[&areas[index as usize]] {
                Ok(value) => {
                    context = format!("{:.0}ms",value);
                    if value >= &700.0 {
                        color = [200,0,0];
                    }else if value <= &200.0{
                        color = [0,255,0];
                    }else{
                        color = ms_to_color(value);
                    }
                },
                Err(value) => {
                    context = value.clone();
                    color = [200,0,0];
                },
            }
            draw_text(
                &mut img,
                &context,
                true,
                &color,
                10+86*index+84/2 -1,
                34 * (4+row)+7,
                20.0,
                &font,
            )
            .unwrap_or_default();
        }
        draw_little_hollow_rect(&mut img, &[255, 255, 255], 612, 34 * (4+row), 111, 30).unwrap_or_default();
        let context: String;
        let color: [u8;3];
        match &item.2 {
            Ok(value) => {
                context = format!("{:.1}ms",value);
                if value >= &700.0 {
                    color = [200,0,0];
                }else if value <= &200.0{
                    color = [0,255,0];
                }else{
                    color = ms_to_color(value);
                }
            },
            Err(value) => {
                context = value.clone();
                color = [200,0,0];
            },
        }
        draw_text(
            &mut img,
            &context,
            true,
            &color,
            612+ 111/2,
            34 * (4+row)+7,
            20.0,
            &font,
        )
        .unwrap_or_default();
        draw_little_hollow_rect(&mut img, &[255, 255, 255], 725, 34 * (4+row), server_name_len as u32, 30).unwrap_or_default();
        draw_text(
            &mut img,
            &item.0,
            false,
            &[255, 255, 255],
            725+4,
            34 * (4+row)+7,
            20.0,
            &font,
        )
        .unwrap_or_default();
        row += 1;
    }

    img.save(file_name.as_ref()).unwrap();
    Ok(())
}

fn draw_text(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    content: &str,
    is_middle: bool,
    color: &[u8; 3],
    x: i32,
    y: i32,
    scale: f32,
    font: &Font,
) -> Result<(), ()> {
    if is_middle {
        draw_text_mut(
            img,
            Rgb::from(color.to_owned()),
            x - ((get_str_len(content) as f32) * scale / 4.0) as i32,
            y,
            Scale::uniform(scale),
            &font,
            content,
        );
    }else{
        draw_text_mut(
            img,
            Rgb::from(color.to_owned()),
            x,
            y,
            Scale::uniform(scale),
            &font,
            content,
        );
    }
    Ok(())
}

fn draw_little_hollow_rect(
    img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>,
    color: &[u8; 3],
    x: i32,
    y: i32,
    wide: u32,
    high: u32,
)-> Result<(), ()> {
    draw_hollow_rect_mut(
        img,
        Rect::at(x, y).of_size(wide, high),
        Rgb(color.to_owned()),
    );
    Ok(())
}

fn get_str_len(text: &str) -> usize {
    let mut len = 0;
    for one_char in text.chars() {
        if one_char.is_ascii() {
            len += 1;
            // println!("{} is ascii",one_char);
        } else {
            len += 2;
            // println!("{} is None",one_char);
        }
    }
    len
}

fn ms_to_color(time: &f64)-> [u8;3] { 
    [(((time-200.0)/500.0) *200.0) as u8, ((1.0-(time-200.0)/500.0)*256.0) as u8  ,0]
}
