use crate::imp::*;
use crate::imp::*;
use lib1::geometry::quad::*;
use lib1::geometry::two_dim::*;
use png;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;
use sdl2::{
    self,
    keyboard::Keycode::{self, *},
    *,
};
use sdl2::{pixels::Color, rect, rect::Point, render::Canvas, ttf::Font, video::Window};
use std::convert::TryInto;
use std::fs::File;
use std::ops::Deref;

pub fn key_events(
    arc_inc: &mut i32,
    arc_dec: &mut i32,
    all_vecs: &mut Vec<Vec<(f32, f32)>>,
    font_10: &Font,
    font_16: &Font,
    width_vec: &mut Vec<u8>,
    iter_values: &mut std::iter::Cycle<std::slice::Iter<std::vec::Vec<(f32, f32)>>>,
    iter_areas: &mut std::iter::Cycle<std::slice::Iter<Vec<f32>>>,
    key: Keycode,
    lshift: &mut bool,
    canvas: &mut WindowCanvas,
    arc_veci: &mut Vec<(Vec<(f32, f32)>, (i16, i16, i16, i16, i16, Color))>,
    arc_vecd: &mut Vec<(Vec<(f32, f32)>, (i16, i16, i16, i16, i16, Color))>,
    c_vecs: &mut Vec<Vec<i16>>,
    mut cf_vecs: &mut Vec<(Color, Vec<i16>)>,
    mut e_vecs: &mut Vec<Vec<i16>>,
    ef_vecs: &mut Vec<(Color, Vec<i16>)>,
    sequence: &mut Vec<char>,
    width:&mut u8
) {
    assert_eq!(
        sequence.len(),
       cf_vecs.len()+ c_vecs.len() + arc_vecd.len() + arc_veci.len() + ef_vecs.len() + e_vecs.len()+all_vecs.len()
    );

    match key {
        LShift => {
            *lshift = true;
        }

        D => {
            if let Some(vec) = arc_vecd.pop() {
                if *arc_dec < 175 {
                    *arc_dec += 5;
                } else {
                    *arc_dec = 5;
                }
                if let Ok((point, radius, start, end)) =
                    calculate(vec.0.to_owned(), *arc_dec as f32, false)
                {
                    println!("{:?}", (point, radius, start, end));

                    arc_veci.push((
                        vec.0,
                        (
                            (point.0 * 10.0).floor() as i16 + 100,
                            (point.1 * 10.0).ceil() as i16,
                            radius as i16 * 10,
                            start as i16,
                            end as i16,
                            Color::BLACK,
                        ),
                    ));
                    function1(canvas, &font_10, &font_16, sequence);
                    function2(canvas, &all_vecs, &width_vec, sequence);
                    function3(canvas, &arc_veci, &arc_vecd, sequence);
                    funtion4(canvas, c_vecs, cf_vecs, e_vecs, ef_vecs, sequence);

                    canvas.present();
                } else {
                    arc_vecd.push(vec);

                    println!("none3");
                }
            } else {
                println!("none",);
            }
        }

        I => {
            if let Some(vec) = arc_veci.pop() {
                if *arc_inc < 175 {
                    *arc_inc += 5;
                } else {
                    *arc_inc = 5;
                }
                if let Ok((point, radius, start, end)) =
                    calculate(vec.0.to_owned(), *arc_inc as f32, true)
                {
                    println!("{:?}", (point, radius, start, end));

                    arc_veci.push((
                        vec.0,
                        (
                            (point.0 * 10.0).floor() as i16 + 100,
                            (point.1 * 10.0).ceil() as i16,
                            radius as i16 * 10,
                            start as i16,
                            end as i16,
                            Color::BLACK,
                        ),
                    ));
                    function1(canvas, &font_10, &font_16, sequence);
                    function2(canvas, &all_vecs, &width_vec, sequence);
                    function3(canvas, &arc_veci, &arc_vecd, sequence);
                    funtion4(canvas, c_vecs, cf_vecs, e_vecs, ef_vecs, sequence);

                    canvas.present();
                } else {
                    arc_veci.push(vec);

                    println!("none3");
                }
            } else {
                println!("none",);
            }
        }
        W=>{
set_width(width);
        }

        C => {
            if let Ok((mut c_vec0, mut cf_vec0, mut e_vec0, mut ef_vec0, mut pa_vec0,mut wid)) = fetch_rounds() {
               
               let mut c_vec1=vec![];
               let mut cf_vec1=vec![];
               let mut e_vec1=vec![];
               let mut ef_vec1=vec![];
                
                if let  Some(mut c_vec)=c_vec0{
                    c_vec1=c_vec.to_owned();
                    c_vecs.append(&mut c_vec);
                }
                if let  Some(mut e_vec)=e_vec0{

                    e_vec1=e_vec.to_owned();

                    e_vecs.append(&mut e_vec);
                }
                    if let  Some(mut cf_vec)=cf_vec0{

                    cf_vec1=cf_vec.to_owned();

                    cf_vecs.append(&mut cf_vec);
                }
                    if let  Some(mut ef_vec)=ef_vec0{

                    ef_vec1=ef_vec.to_owned();

                    ef_vecs.append(&mut ef_vec);
                }
                   
               
                    if let  Some(mut pa_vec)=pa_vec0{
                        let mut wids= wid.unwrap();
                        function2(canvas, &pa_vec, &wids, sequence);
                    all_vecs.append(&mut pa_vec);
                width_vec.append(&mut wids);

                }
                funtion4(canvas, &c_vec1, &cf_vec1, &e_vec1, &ef_vec1, sequence);
              
                canvas.present();

            }
        }
        S => {
            let data = canvas
                .read_pixels(
                    sdl2::rect::Rect::new(0, 0, 1010, 700),
                    // from_c(Point::new(300, 300), 400, 400),
                    sdl2::pixels::PixelFormatEnum::RGB24,
                )
                .unwrap();
            let file = File::create(std::path::Path::new("xyz.png")).unwrap();
            let buf_wr = std::io::BufWriter::new(file);
            let mut enc = png::Encoder::new(buf_wr, 1010, 700);
            enc.set_depth(png::BitDepth::Eight);
            enc.set_color(png::ColorType::RGB);
            let mut header = enc.write_header().unwrap();
            header.write_image_data(&data).unwrap();
        }
        M => {
            if let Ok((mut wvs, mut mvecs, mut wv)) = fetch_values() {
                let a: std::vec::Vec<std::vec::Vec<(f32, f32)>>;
                if !wvs.is_empty(){
                    function2(canvas, &mut wvs, &mut wv, sequence);

                    all_vecs.append(&mut wvs);
                width_vec.append(&mut wv);
                }

                for vecs in mvecs.iter_mut() {
                    
                    for i in 0..vecs.len() {
                        vecs[i].0 = vecs[i].0 + 10.0;
                    }

                    if let Ok((point, radius, mut start, mut end)) =
                        calculate(vecs.to_owned(), 180.0, true)
                    {
                        if *lshift {
                            let mid = start;
                            start = end;
                            end = mid;
                            arc_vecd.push((
                                vecs.to_owned(),
                                (
                                    (point.0 * 10.0).floor() as i16 + 100,
                                    (point.1 * 10.0).ceil() as i16,
                                    radius as i16 * 10,
                                    end as i16,
                                    start as i16,
                                    Color::BLACK,
                                ),
                            ));
                            sequence.push('d');

                            *lshift = false;
                        } else {
                            sequence.push('i');
                            arc_veci.push((
                                vecs.to_owned(),
                                (
                                    (point.0 * 10.0).floor() as i16 + 100,
                                    (point.1 * 10.0).ceil() as i16,
                                    radius as i16 * 10,
                                    end as i16,
                                    start as i16,
                                    Color::BLACK,
                                ),
                            ));
                        }
                        if let Ok(_) = canvas.arc(
                            (point.0 * 10.0).floor() as i16 + 100,
                            (point.1 * 10.0).ceil() as i16,
                            radius as i16 * 10,
                            start as i16,
                            end as i16,
                            Color::BLACK,
                        ) {
                        }
                    }
                }

                canvas.present();
            }
        }
        E => {
            if let Some(last_push) = sequence.pop() {
                rm_last_push(
                    last_push, true, canvas, &font_10, &font_16, all_vecs, width_vec, arc_veci,
                    arc_vecd, c_vecs, cf_vecs, e_vecs, ef_vecs, sequence,
                );
            }
        }
        U => {
            if let Some(last_push) = sequence.pop() {
                rm_last_push(
                    last_push,
                    false,
                    canvas,
                    &font_10,
                    &font_16,
                    all_vecs,
                    width_vec,
                    arc_veci,
                    arc_vecd,
                    c_vecs,
                    &mut cf_vecs,
                    &mut e_vecs,
                    ef_vecs,
                    sequence,
                );
            }
        }
        N => {
            all_vecs.clear();
            width_vec.clear();
            sequence.clear();
            if let Some(val) = iter_values.next() {
                let txt: String =
                    "Area,Diagonal1,Diagonal2:".to_string() + format!("{:?}", iter_areas.next().unwrap()).as_str();
                draw_text(
                    &txt,
                    Some(canvas),
                    &font_16,
                    10 * txt.len() as u32,
                    16,
                    300,
                    650,
                );
                all_vecs.push(val.to_owned());
                width_vec.push(3);
                function1(canvas, &font_10, &font_16, sequence);
                function2(canvas, &all_vecs, &width_vec, sequence);

                canvas.present();
            }
        }
        _ => {}
    }
}
