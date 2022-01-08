use sdl2;
use sdl2::gfx::primitives::DrawRenderer;
#[allow(unused)]
use sdl2::rect::Point;
// use sdl2::image::LoadSurface;
use sdl2::image::ImageRWops;
use sdl2::image::*;
use sdl2::pixels::Color;
// const WINWIDTH: i32 = 640;
const WIDTH: i32 = 600;
use std::collections::hash_map::HashMap;
// const HEIGHT:u32=480;
#[allow(unused)]
fn main() {
    let mut hmap = HashMap::new();

    let widths: [u32; 2] = [846, 856];
    let heights: [u32; 2] = [90, 136];
    let sdlctxt = sdl2::init().unwrap();
    let vidctxt = sdlctxt.video().unwrap();
    let window = vidctxt.window("myWindow", 900, 600).build().unwrap();
    let mut evpump = sdlctxt.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    // let canv:sdl2::render::Canvas<sdl2::video::Window>;
    let mut txt: Vec<sdl2::render::Texture> = vec![];
    let _img = sdl2::image::init(InitFlag::PNG).unwrap();
    let ladder = sdl2::rwops::RWops::from_file(std::path::Path::new("ladder.png"), "r").unwrap();
    let ladd_surf = ladder.load_png().unwrap();
    let shark = sdl2::rwops::RWops::from_file(std::path::Path::new("shark.png"), "r").unwrap();
    let shark_surf = shark.load_png().unwrap();
    let txtcrt = canvas.texture_creator();
    canvas.set_draw_color(sdl2::pixels::Color::WHITE);
    canvas.clear();
    let startladders: Vec<Vec<i32>> = vec![
        vec![19, 31, 78, 43],
        vec![30, 41, 91, 69],
        vec![55, 28, 37, 44],
        vec![97, 70, 67, 87],
    ];
    for i in 0..11 {
        if i != 10 {
            for j in 0..10 {
                if i % 2 == 0 {
                    for k in 0..startladders.len() {
                        if startladders[k].contains(&(10 * (10 - i) - j - 1)) {
                            hmap.entry((10 * (10 - i) - j - 1)).or_insert((
                                ((j * WIDTH / 10) + (WIDTH / 30)) as i32,
                                (i * WIDTH / 10) + (WIDTH / 20) as i32,
                            ));
                        }
                    }

                    canvas
                        .string(
                            ((j * WIDTH / 10) + (WIDTH / 30)) as i16,
                            ((i * WIDTH / 10) + (WIDTH / 20)) as i16,
                            format!("{:?}", (10 * (10 - i) - j - 1)).as_str(),
                            Color::BLACK,
                        )
                        .unwrap();
                } else {
                    for k in 0..startladders.len() {
                        if startladders[k].contains(&(10 * (10 - i) - 10 + j)) {
                            hmap.entry((10 * (10 - i) - 10 + j)).or_insert(
                                ((
                                    ((j * WIDTH / 10) + (WIDTH / 30)) as i32,
                                    (i * WIDTH / 10) + (WIDTH / 20) as i32,
                                )),
                            );
                        }
                    }
                    canvas
                        .string(
                            ((j * WIDTH / 10) + (WIDTH / 30)) as i16,
                            ((i * WIDTH / 10) + (WIDTH / 20)) as i16,
                            format!("{:?}", (10 * (10 - i) - 10 + j)).as_str(),
                            Color::BLACK,
                        )
                        .unwrap();
                }
            }
        }
        if i == 0 || i == 10 {
            canvas
                .thick_line(
                    (0 * WIDTH / 10) as i16,
                    (i * WIDTH / 10) as i16,
                    (10 * WIDTH / 10) as i16,
                    (i * WIDTH / 10) as i16,
                    3,
                    Color::BLACK,
                )
                .unwrap();
            canvas
                .thick_line(
                    (i * WIDTH / 10) as i16,
                    (0 * WIDTH / 10) as i16,
                    (i * WIDTH / 10) as i16,
                    (10 * WIDTH / 10) as i16,
                    3,
                    Color::BLACK,
                )
                .unwrap();
        } else {
            canvas
                .thick_line(
                    (0 * WIDTH / 10) as i16,
                    (i * WIDTH / 10) as i16,
                    (10 * WIDTH / 10) as i16,
                    (i * WIDTH / 10) as i16,
                    2,
                    Color::BLACK,
                )
                .unwrap();
            canvas
                .thick_line(
                    (i * WIDTH / 10) as i16,
                    (0 * WIDTH / 10) as i16,
                    (i * WIDTH / 10) as i16,
                    (10 * WIDTH / 10) as i16,
                    2,
                    Color::BLACK,
                )
                .unwrap();
        }
    }
    fn arad(a: (i32, i32), b: (i32, i32)) -> f64 {
        if (b.0 - a.0 != 0) {
            let val = (180f64 / std::f64::consts::PI)
                * (((b.1 - a.1) as f64 / (b.0 - a.0) as f64).atan());
            if val > 90f64 {
                90f64 - val
            } else {
                val
            }
        } else {
            90f64
        }
    }
    fn dfp(a: (i32, i32), b: (i32, i32)) -> u32 {
        (((a.0 - b.0) as f32).powi(2) + ((a.1 - b.1) as f32).powi(2))
            .powf(0.5)
            .ceil() as u32
    }
    let mut i = 0;
    let mut i = 0;
    let j = 0;
    /*
    startladders: Vec<Vec<i32>> = vec![
            vec![19, 39, 78, 43],
            vec![30, 60, 91, 69],
            vec![55, 28, 7, 3],

            vec![97, 70, 49, 33],
        ];
    */
    let mut txt = txtcrt.create_texture_from_surface(ladd_surf).unwrap();

    canvas
        .copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[i], heights[i])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[0][0])).unwrap().to_owned().0
                        + hmap.get(&(startladders[1][0])).unwrap().to_owned().0
                        + 3 * WIDTH / 10)
                        / 2,
                    (hmap.get(&(startladders[0][0])).unwrap().to_owned().1
                        + hmap.get(&(startladders[1][0])).unwrap().to_owned().1)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[0][0])).unwrap().to_owned(),
                    hmap.get(&(startladders[1][0])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[0][0])).unwrap().to_owned(),
                hmap.get(&(startladders[1][0])).unwrap().to_owned(),
            ),
            Point::from((
                (hmap.get(&(startladders[0][0])).unwrap().to_owned().0
                    + hmap.get(&(startladders[1][0])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[0][0])).unwrap().to_owned().1
                    + hmap.get(&(startladders[1][0])).unwrap().to_owned().1)
                    / 2,
            )),
            false,
            false,
        )
        .unwrap();
   canvas.copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[i], heights[i])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[0][1])).unwrap().to_owned().0
                        + hmap.get(&(startladders[1][1])).unwrap().to_owned().0+WIDTH/10
                        )
                        / 2,
                    (hmap.get(&(startladders[0][1])).unwrap().to_owned().1
                        + hmap.get(&(startladders[1][1])).unwrap().to_owned().1+WIDTH/35)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[0][1])).unwrap().to_owned(),
                    hmap.get(&(startladders[1][1])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[0][0])).unwrap().to_owned(),
                hmap.get(&(startladders[1][0])).unwrap().to_owned(),
            ),
            
            Point::from((
                (hmap.get(&(startladders[0][1])).unwrap().to_owned().0
                    + hmap.get(&(startladders[1][1])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[0][1])).unwrap().to_owned().1
                    + hmap.get(&(startladders[1][1])).unwrap().to_owned().1)
                    / 2,
            )),
            false,
            false,
        )
        .unwrap();
        canvas
        .copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[i], heights[i])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[0][2])).unwrap().to_owned().0
                        + hmap.get(&(startladders[1][2])).unwrap().to_owned().0
                        +   WIDTH / 10)
                        / 2,
                    (hmap.get(&(startladders[0][2])).unwrap().to_owned().1
                        + hmap.get(&(startladders[1][2])).unwrap().to_owned().1- WIDTH/20)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[0][2])).unwrap().to_owned(),
                    hmap.get(&(startladders[1][2])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[0][2])).unwrap().to_owned(),
                hmap.get(&(startladders[1][2])).unwrap().to_owned(),
            ),
            Point::from((
                (hmap.get(&(startladders[0][2])).unwrap().to_owned().0
                    + hmap.get(&(startladders[1][2])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[0][2])).unwrap().to_owned().1
                    + hmap.get(&(startladders[1][2])).unwrap().to_owned().1)
                    / 2,
            )),
            false,
            false,
        )
        .unwrap();
  
        canvas
        .copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[i], heights[i])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[0][3])).unwrap().to_owned().0
                        + hmap.get(&(startladders[1][3])).unwrap().to_owned().0
                        + 3 * WIDTH / 10)
                        / 2,
                    (hmap.get(&(startladders[0][3])).unwrap().to_owned().1
                        + hmap.get(&(startladders[1][3])).unwrap().to_owned().1 -2*WIDTH/10)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[0][3])).unwrap().to_owned(),
                    hmap.get(&(startladders[1][3])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[0][3])).unwrap().to_owned(),
                hmap.get(&(startladders[1][3])).unwrap().to_owned(),
            ),
            Point::from((
                (hmap.get(&(startladders[0][3])).unwrap().to_owned().0
                    + hmap.get(&(startladders[1][3])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[0][3])).unwrap().to_owned().1
                    + hmap.get(&(startladders[1][3])).unwrap().to_owned().1)
                    / 2,
            )),
            false,
            false,
        )
        .unwrap();
        
    txt = txtcrt.create_texture_from_surface(shark_surf).unwrap();
    canvas
        .copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[1], heights[1])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[2][0])).unwrap().to_owned().0
                        + hmap.get(&(startladders[3][0])).unwrap().to_owned().0
                        -4*WIDTH/10)
                        / 2,
                    (hmap.get(&(startladders[2][0])).unwrap().to_owned().1
                        + hmap.get(&(startladders[3][0])).unwrap().to_owned().1)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[2][0])).unwrap().to_owned(),
                    hmap.get(&(startladders[3][0])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[2][0])).unwrap().to_owned(),
                hmap.get(&(startladders[3][0])).unwrap().to_owned(),
            ),
            Point::from((
                (hmap.get(&(startladders[2][0])).unwrap().to_owned().0
                    + hmap.get(&(startladders[3][0])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[2][0])).unwrap().to_owned().1
                    + hmap.get(&(startladders[3][0])).unwrap().to_owned().1)
                    / 2,
            )),
            true,
            false,
        )
        .unwrap();
    canvas
        .copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[1], heights[1])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[2][1])).unwrap().to_owned().0
                        + hmap.get(&(startladders[3][1])).unwrap().to_owned().0
                        -WIDTH/6)
                        / 2,
                    (hmap.get(&(startladders[2][1])).unwrap().to_owned().1
                        + hmap.get(&(startladders[3][1])).unwrap().to_owned().1-2*WIDTH)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[2][1])).unwrap().to_owned(),
                    hmap.get(&(startladders[3][1])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[2][1])).unwrap().to_owned(),
                hmap.get(&(startladders[3][1])).unwrap().to_owned(),
            ),
            Point::from((
                (hmap.get(&(startladders[2][1])).unwrap().to_owned().0
                    + hmap.get(&(startladders[3][1])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[2][1])).unwrap().to_owned().1
                    + hmap.get(&(startladders[3][1])).unwrap().to_owned().1)
                    / 2,
            )),
            false,
            false,
        )
        .unwrap();
    canvas
        .copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[1], heights[1])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[2][2])).unwrap().to_owned().0
                        + hmap.get(&(startladders[3][2])).unwrap().to_owned().0
                        +6*WIDTH/10)
                        / 2,
                    (hmap.get(&(startladders[2][2])).unwrap().to_owned().1
                        + hmap.get(&(startladders[3][2])).unwrap().to_owned().1 -7*WIDTH/10)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[2][2])).unwrap().to_owned(),
                    hmap.get(&(startladders[3][2])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[2][2])).unwrap().to_owned(),
                hmap.get(&(startladders[3][2])).unwrap().to_owned(),
            ),
            Point::from((
                (hmap.get(&(startladders[2][2])).unwrap().to_owned().0
                    + hmap.get(&(startladders[3][2])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[2][2])).unwrap().to_owned().1
                    + hmap.get(&(startladders[3][2])).unwrap().to_owned().1)
                    / 2,
            )),
            false,
            false,
        ).unwrap();
        canvas
        .copy_ex(
            &txt,
            sdl2::rect::Rect::from((0, 0, widths[1], heights[1])),
            sdl2::rect::Rect::from_center(
                Point::from((
                    (hmap.get(&(startladders[2][3])).unwrap().to_owned().0
                        + hmap.get(&(startladders[3][3])).unwrap().to_owned().0-WIDTH/10
                        )
                        / 2,
                    (hmap.get(&(startladders[2][3])).unwrap().to_owned().1
                        + hmap.get(&(startladders[3][3])).unwrap().to_owned().1-2*WIDTH/7)
                        / 2,
                )),
                dfp(
                    hmap.get(&(startladders[2][3])).unwrap().to_owned(),
                    hmap.get(&(startladders[3][3])).unwrap().to_owned(),
                ),
                WIDTH as u32 / 20,
            ),
            arad(
                hmap.get(&(startladders[2][3])).unwrap().to_owned(),
                hmap.get(&(startladders[3][3])).unwrap().to_owned(),
            ),
            Point::from((
                (hmap.get(&(startladders[2][3])).unwrap().to_owned().0
                    + hmap.get(&(startladders[3][3])).unwrap().to_owned().0)
                    / 2,
                (hmap.get(&(startladders[2][3])).unwrap().to_owned().1
                    + hmap.get(&(startladders[3][3])).unwrap().to_owned().1)
                    / 2,
            )),
            false,
            false,
        )
        .unwrap();
    canvas.present();
    'eventloop: loop {
        for event in evpump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'eventloop;
                }
                _ => {}
            }
        }
    }
}
