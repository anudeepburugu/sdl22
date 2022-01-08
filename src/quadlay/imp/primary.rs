// #[allow(unused_variables,dead_code)]
// pub mod qfs_draw{
use crate::imp::*;
use lib1::geometry::quad::*;
use lib1::geometry::two_dim::*;
use png;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::surface::Surface;
use sdl2::{self, keyboard::Keycode::*, *};
use sdl2::{pixels::Color, rect, rect::Point, render::Canvas, ttf::Font, video::Window};
use std::convert::TryInto;
use std::fs::File;
use std::ops::Deref;

pub fn qfsa(side1: f32, side2: f32, side3: f32, side4: f32, area: f32) {
    let mut all_vecs: Vec<Vec<(f32, f32)>> = vec![];
let mut width=1;
    let ttf = sdl2::ttf::init().unwrap();
    let font_10 = ttf
        .load_font("/usr/share/fonts/truetype/freefont/FreeSans.ttf", 10)
        .unwrap();
    let font_16 = ttf
        .load_font(
            "/usr/share/fonts/truetype/freefont/FreeSansBoldOblique.ttf",
            16,
        )
        .unwrap();

    let sdl = init().unwrap();
    let video = sdl.video().unwrap();
    let audio = sdl.audio().unwrap();

    let window:Window = video
        .window("custom window", 1100, 700)
        .opengl()
        .build()
        .unwrap();
    // let wind:sdl2::video::Window;
// window.gl_make_current(context: &GLContext);
let windgl:sdl2::video::GLContext= window.gl_create_context().unwrap();

// windgl.raw
    let mut canvas = window.into_canvas().build().unwrap();
// canvas.draw_color()\
    let mut event_pump = sdl.event_pump().unwrap();
    let mut arc_vecd = vec![];
    let mut arc_veci = vec![];
    let mut cf_vecs = vec![];
    let mut e_vecs = vec![];
    let mut c_vecs = vec![];
    let mut ef_vecs = vec![];
    let seq: &mut Vec<char> = &mut vec![];
    let mut arc_inc = 0;
    let mut arc_dec = 0;
    //TODO figure out how to get texture and save from window canvas;Note: surface canvas wouldn't let me draw,only texture gets copied

    function1(&mut canvas, &font_10, &font_16, seq);

    let values = get_results(side1, side2, side3, side4, area);
    let mut iter_areas = values.1.iter().cycle();
    // let mut iter_diag1 = values.1[1].iter().cycle();
    // let mut iter_diag2 = values.1[2].iter().cycle();

    let mut iter_values = values.0.iter().cycle();
    let point = Point::new(300, 650);
    let first = iter_values.next().unwrap();
    let mut width_vec = vec![3];
    function2(&mut canvas, &vec![first.to_owned()], &width_vec, seq);
    all_vecs.push(values.0[0].to_vec());
    // width_vec.push(3);
    let txt: String = "Area,Diagonal1,Diagonal2:".to_string() + format!("{:?}", iter_areas.next().unwrap()).as_str()
    // "Daigonal1:" + format!("{}", iter_diag1.next().unwrap()).as_str()
    // +"Diagonal2:" + format!("{}", iter_diag2.next().unwrap()).as_str()
    ;

    draw_text(
        &txt,
        Some(&mut canvas),
        &font_16,
        (10 * txt.len()) as u32,
        16,
        300,
        650,
    );
    let mut end: Point;
    let mut val: bool = true;
    let mut start = Point::new(0, 0);
    let mut lshift = false;
    canvas.present();
    'everem: loop {
        for events in event_pump.poll_iter() {
            match events {
                event::Event::Quit { .. } => {
                    break 'everem;
                }

                Event::MouseButtonDown {
                    which,
                    clicks,
                    mouse_btn,
                    x,
                    y,
                    ..
                } => {
                    if sdl2::mouse::MouseButton::Right == mouse_btn {
                        val = true;
                    } else if sdl2::mouse::MouseButton::Left == mouse_btn {
                        if val {
                            start = get_values(x, y);
                            all_vecs.push(vec![(
                                (start.x() as f32 - 100.0) / 10.0,
                                start.y() as f32 / 10.0,
                            )]);
                            seq.push('l');

                            width_vec.push(width);
                            val = false;
                        } else {
                            let end = get_values(x, y);
                            if all_vecs.last().unwrap().last().unwrap()
                                != &((end.x() as f32 - 100.0) / 10.0, end.y() as f32 / 10.0)
                            {
                                all_vecs
                                    .last_mut()
                                    .unwrap()
                                    .push(((end.x() as f32 - 100.0) / 10.0, end.y() as f32 / 10.0));
                                canvas.draw_line(start, end);
                                canvas.present();

                                start = end;
                            }
                        }
                        if clicks > 1 {
                            val = true;
                        }
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    function1(&mut canvas, &font_10, &font_16, seq);
                    function2(&mut canvas, &all_vecs, &width_vec, seq);
                    function3(&mut canvas, &arc_veci, &arc_vecd, seq);
                    funtion4(&mut canvas, &c_vecs, &cf_vecs, &e_vecs, &ef_vecs, seq);

                    // function4(&mut canvas);
                    canvas
                        .string(
                            x as i16,
                            y as i16,
                            format!("({},{})", (x / 10) - 10, y / 10).as_str(),
                            Color::BLUE,
                        )
                        .unwrap();
                    canvas.present();
                }

                Event::MouseButtonUp {
                    clicks,
                    mouse_btn,
                    x,
                    y,
                    ..
                } => {
                    if sdl2::mouse::MouseButton::Left == mouse_btn {
                        let end = get_values(x, y);
                        if all_vecs.last().unwrap().last().unwrap()
                            != &((end.x() as f32 - 100.0) / 10.0, end.y() as f32 / 10.0)
                        {
                            all_vecs
                                .last_mut()
                                .unwrap()
                                .push(((end.x() as f32 - 100.0) / 10.0, end.y() as f32 / 10.0));

                            canvas.draw_line(start, end);
                            // seq.push('l');
                            canvas.present();
                        }

                        if !val {
                            start = end;
                        }
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => key_events(
                    &mut arc_inc,
                    &mut arc_dec,
                    &mut all_vecs,
                    &font_10,
                    &font_16,
                    &mut width_vec,
                    &mut iter_values,
                    &mut iter_areas,
                    key,
                    &mut lshift,
                    &mut canvas,
                    &mut arc_veci,
                    &mut arc_vecd,
                    &mut c_vecs,
                    &mut cf_vecs,
                    &mut e_vecs,
                    &mut ef_vecs,
                    seq,
                    &mut width
                ),

                _ => {}
            }
        }
    }
}

// }
