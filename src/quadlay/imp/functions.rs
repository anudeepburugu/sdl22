use crate::imp::*;
use lib1::geometry::two_dim::Geom;
use lib1::geometry::two_dim::Results;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::{pixels::Color, rect, rect::Point, render::Canvas, ttf::Font, video::Window};

pub fn function2(
    canvas: &mut Canvas<Window>,
    vec_vals: &Vec<Vec<(f32, f32)>>,
    width: &Vec<u8>,
    sequence: &mut Vec<char>,
) {
    // println!("{:?} {:?}",vec_vals,width);
    assert_eq!(width.len(), vec_vals.len());
    let mut val = true;
    let mut thickness: u8 = 1;
    let mut j = 0;
    canvas.set_draw_color(Color::BLACK);
    for first in vec_vals.iter() {
        for i in 1..first.len() {
            canvas.thick_line(
                (first[i - 1].0 * 10.0 + 100.0) as i16,
                (first[i - 1].1 * 10.0) as i16,
                (first[i].0 * 10.0 + 100.0) as i16,
                (first[i].1 * 10.0) as i16,
                width[j],
                Color::BLACK,
            );
            canvas.string(
                ((first[i - 1].0 * 10.0 + 100.0) as i16 + (first[i].0 * 10.0 + 100.0) as i16) / 2,
                ((first[i - 1].1 * 10.0) as i16 + (first[i].1 * 10.0) as i16) / 2,
                format!(
                    "{}",
                    Results::distance_from_points(
                        ((first[i - 1].0 + 10.0), (first[i - 1].1)),
                        ((first[i].0 + 10.0), (first[i].1))
                    )
                    
                )
                .as_str(),
                Color::BLACK,
            );
        }
        j += 1;
        sequence.push('l');
    }
}

pub fn function1(
    canvas: &mut Canvas<Window>,
    font10: &Font,
    font16: &Font,
    sequence: &mut Vec<char>,
) {
    //   surf:sdl2::surface::Surface<'static>)->Canvas<sdl2::surface::Surface<'static>>{
    let mut x_y = vec![];

    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    canvas.set_draw_color(Color::RED);
    canvas
        .draw_line(PointA((100, 0), None, None), PointA((100, 700), None, None))
        .unwrap();
    canvas.set_draw_color(Color::BLACK);

    for i in 0..100 {
        draw_text(
            format!("{}", i).as_str(),
            Some(canvas),
            &font10,
            10,
            10,
            i * 10 + 100 as i32,
            10,
        );
        for j in 1..71 {
            x_y.push((i, j));
            canvas.pixel((i * 10) as i16, j * 10, Color::BLACK);
        }
    }

    for i in 0..70 {
        draw_text(
            format!("{}", i).as_str(),
            Some(canvas),
            &font10,
            10,
            10,
            90,
            i * 10,
        );
    }
    let point2 = Point::new(300, 680);

    let str_txt = "n->next;,m->multipoint from cmd;e->erase;u->undo lmv;s->spng c->";

    draw_text(
        str_txt,
        Some(canvas),
        &font16,
        (10 * str_txt.len()) as u32,
        16,
        300,
        680,
    );
    sequence.clear();
}
pub fn draw_text(
    stri: &str,
    canva_wind: Option<&mut Canvas<Window>>,
    font: &sdl2::ttf::Font,
    w: u32,
    h: u32,
    x: i32,
    y: i32,
) {
    if let Some(canvas) = canva_wind {
        let surf = font.render(stri).blended(Color::BLACK).unwrap();

        let ctc = canvas.texture_creator();
        let txt = ctc.create_texture_from_surface(&surf).unwrap();
        canvas
            .copy(&txt, None, Some(rect::Rect::new(x, y, w, h)))
            .unwrap();
    }
}

pub fn function3(
    canvas: &mut Canvas<Window>,
    arc_veci: &Vec<(Vec<(f32, f32)>, (i16, i16, i16, i16, i16, Color))>,
    arc_vecd: &Vec<(Vec<(f32, f32)>, (i16, i16, i16, i16, i16, Color))>,
    sequence: &mut Vec<char>,
) {

    for value in arc_veci.iter() {
        if let Ok(_) = canvas.arc(
            value.1 .0, value.1 .1, value.1 .2, value.1 .3, value.1 .4, value.1 .5,
        ) {
            sequence.push('i');

            canvas.string(
                ((value.0[0].0 + value.0[1].0) * 10.0 + 200.0) as i16 / 2,
                ((value.0[0].1 + value.0[1].1) * 10.0) as i16 / 2,
                format!("{}", (value.1 .4 - value.1 .3)).as_str(),
                value.1 .5,
            );
        }
    }
    for value in arc_vecd.iter() {
        if let Ok(_) = canvas.arc(
            value.1 .0, value.1 .1, value.1 .2, value.1 .3, value.1 .4, value.1 .5,
        ) {
            sequence.push('a');

            canvas.string(
                ((value.0[0].0 + value.0[1].0) * 10.0 + 200.0) as i16 / 2,
                ((value.0[0].1 + value.0[1].1) * 10.0) as i16 / 2,
                format!("{}", (value.1 .4 - value.1 .3)).as_str(),
                value.1 .5,
            );
        }
    }
    // println!("{:?}",sequence);
}

pub fn funtion4(
    canvas: &mut Canvas<Window>,
    c_vec: &Vec<Vec<i16>>,
    cf_vec: &Vec<(Color, Vec<i16>)>,
    e_vec: &Vec<Vec<i16>>,
    ef_vec: &Vec<(Color, Vec<i16>)>,
    sequence: &mut Vec<char>,
) {
    for circle in c_vec.iter() {
        if let Ok(_) = canvas.circle(
            circle[0] * 10 + 100,
            circle[1] * 10,
            circle[2] * 10,
            Color::BLACK,
        ) {
            sequence.push('c');
        }
    }
    for circle in e_vec.iter() {
        if let Ok(_) = canvas.ellipse(
            circle[0] * 10 + 100,
            circle[1] * 10,
            circle[2] * 10,
            circle[3] * 10,
            Color::BLACK,
        ) {
            sequence.push('e');
        }
    }
    for (color, circle) in cf_vec.iter() {
        if let Ok(_) = canvas.filled_circle(
            circle[0] * 10 + 100,
            circle[1] * 10,
            circle[2] * 10,
            color.to_owned(),
        ) {
            sequence.push('d');
        }
    }
    for (color, circle) in ef_vec.iter() {
        if let Ok(_) = canvas.filled_ellipse(
            circle[0] * 10 + 100,
            circle[1] * 10,
            circle[2] * 10,
            circle[3] * 10,
            color.to_owned(),
        ) {
            sequence.push('f');
        }
    }

}

pub fn rm_last_push(
    last: char,
    e: bool,
    canvas: &mut Canvas<Window>,
    font_10: &Font,
    font_16: &Font,
    all_vecs: &mut Vec<Vec<(f32, f32)>>,
    width_vec: &mut Vec<u8>,
    arc_veci: &mut Vec<(Vec<(f32, f32)>, (i16, i16, i16, i16, i16, Color))>,
    arc_vecd: &mut Vec<(Vec<(f32, f32)>, (i16, i16, i16, i16, i16, Color))>,
    c_vec: &mut Vec<Vec<i16>>,
    cf_vec: &mut Vec<(Color, Vec<i16>)>,
    e_vec: &mut Vec<Vec<i16>>,
    ef_vec: &mut Vec<(Color, Vec<i16>)>,
    mut sequence: &mut Vec<char>,
) {
    match last {
        'a' => {
            arc_vecd.pop();
        }
        'i' => {
            arc_veci.pop();
        }
        'd' => {
            cf_vec.pop();
        }
        'c' => {
            c_vec.pop();
        }
        'e' => {
            e_vec.pop();
        }
        'f' => {
            ef_vec.pop();
        }
        'p' => {
            //TODO:
        }

        'l' => {
            if e {
                all_vecs.pop();
                width_vec.pop();
            } else {
                if let Some(value_r) = all_vecs.pop() {
                    let wid = width_vec.pop().unwrap();
                    if value_r.len() > 2 {
                        let mut values_r = set_modified(value_r);
                        values_r.pop();
                        /*  canvas.draw_lines(PointsA {
                            x_y: &values_r,
                            values: &mut vec![],
                            scale: Some((10.0, 10.0)),
                            shift: Some((100.0, 0.0)),
                        });*/
                        all_vecs.push(values_r);
                        width_vec.push(wid);
                        sequence.push('l');
                    }
                }
            }
        }
        _ => {}
    }
    function1(canvas, font_10, font_16, sequence);
    function2(canvas, all_vecs, width_vec, sequence);
    function3(canvas, arc_veci, arc_vecd, sequence);
    funtion4(canvas, c_vec, cf_vec, e_vec, ef_vec, sequence);
    canvas.present();
}

