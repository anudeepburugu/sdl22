
use lib1::geometry::two_dim::{Results, *};
use sdl2::rect::Point;

pub fn get_values(x: i32, y: i32) -> Point {
    let mut xi = ((x as f32) / 10.0) as i32 * 10;
    let mut yi = ((y as f32) / 10.0) as i32 * 10;
    if x - xi >= 5 {
        xi += 10;
    }
    if y - yi >= 5 {
        yi += 10;
    }
    Point::new(xi, yi)
}

fn set(mut values: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut j = 0;
    for i in 0..values.len() - 1 {
        if values[i - j + 1..].contains(&values[i - j]) {
            values.remove(i - j);
            j += 1;
        }
    }
    values
}
pub fn set_modified(mut a: Vec<(f32, f32)>) -> Vec<(f32, f32)> {
    let mut j = 0;
    for i in 0..a.len() - 1 - j {
        if a[i - j] == a[i - j + 1] {
            a.remove(i - j);
            j += 1;
        }
    }
    a
}
//TODO:verify logic again difference in angle should be same as above;
pub fn calculate(
    points: Vec<(f32, f32)>,
    angle: f32,
    inc: bool,
) -> Result<((f32, f32), f32, f32, f32), String> {
    let center = Results::center_from_distance(
        vec![
            (points[0].0 as f32, points[0].1 as f32),
            (points[1].0 as f32, points[1].1 as f32),
        ],
        angle,
    );
    let mut a1 = Results::angle_from_points(vec![(center.0 .0, center.0 .1), points[0].to_owned()]);
    let mut a2 = Results::angle_from_points(vec![(center.0 .0, center.0 .1), points[1].to_owned()]);
    let mut a3 = Results::angle_from_points(vec![(center.1 .0, center.1 .1), points[0].to_owned()]);
    let mut a4 = Results::angle_from_points(vec![(center.1 .0, center.1 .1), points[1].to_owned()]);
    let m = Results::angle_from_points(points.to_owned());

    let r1 = Results::distance_from_points(center.0, points[0].to_owned());
    let r3 = Results::distance_from_points(center.1, points[1].to_owned());
    if a1 < 0.0 {
        a1 = a1 + 180.0;
    }
    if a2 < 0.0 {
        a2 = a2 + 180.0;
    }
    if a3 < 0.0 {
        a3 = a3 + 180.0;
    }
    if a4 < 0.0 {
        a4 = a4 + 180.0;
    }
    if inc {
        if angle == 180f32 {
            Ok((center.1, r1, m, m + 180f32))
        } else if (a1 - a2).abs() <= angle + 15.0 && (a1 - a2).abs() >= angle - 15.0 {
            let v1 = a1;
            let a2 = a2;
            Ok((center.0, r1, a2, a1))
        } else {
            Err("failed to fetch".to_owned())
        }
    } else {
        if angle == 180f32 {
            Ok((center.1, r1, m + 180f32, m))
        } else if (a3 - a4).abs() <= angle + 15.0 && (a3 - a4).abs() >= angle - 15.0 {
            Ok((center.1, r3, 180.0 + a3, 180.0 + a4))
        } else {
            Err("failed to fetch".to_owned())
        }
    }
}

use std::io::{self, Read};

pub fn fetch_values(
) -> std::result::Result<(Vec<Vec<(f32, f32)>>, Vec<Vec<(f32, f32)>>, Vec<u8>), String> {
    let mut wid_vecs: Vec<Vec<(f32, f32)>> = vec![];
    let mut arc_vecs: Vec<Vec<(f32, f32)>> = vec![];
    let mut arc_vals = false;
    // let mut arc_degs:Vec<u8>=vec![];
    // let mut vals:Vec<(f32,f32)>=vec![];
    let mut wid_vec: Vec<u8> = vec![];
    // let mut fill_vals:Vec<bool>=vec![];
    // let mut fill_cols:Vec<Option<sdl2::pixels::Color>>=vec![];

    println!(
        "please enter values 
     ex: -a (10,10) (10,10) -w 3 (10,10),(10,10),(10,10),(10,10) -a (1,1),(2,2)
     ex: (10,10) (10,10) -w 3 (10,10),(10,10),(10,10),(10,10) -a (1,1),(2,2)
     ex:  (10,10) (10,10) -a (10,10),(10,10) -a (10,10),(10,10) -a (1,1),(2,2)
     "
    );
    let stdin = io::stdin();
    let mut buf = "".into();
    if let Ok(_) = stdin.read_line(&mut buf) {
        let mut x: (f32, f32) = (0.0, 0.0);
        let mut i = 0;
        let mut dec = false;
        let mut fill = false;
        let mut arc = false;
        buf = buf
            .replace("(", "")
            .replace(")", "")
            .replace(",", " ")
            .replace("\n", " ");
        // println!("{}",buf.len());

        // buf+=" ";
        // println!("{}",buf.len());
        for value in buf.split(" ") {
            if dec || fill || arc {
                if dec {
                    if let Ok(val) = value.parse::<u8>() {
                        wid_vec.push(val);
                    } else {
                        wid_vec.push(1);
                    }
                    dec = false;
                } else if fill {
                    // fill_cols.push(Some(sdl2::pixels::Color::RGB(255, 0, 255)));
                    fill = false;
                } else if arc {
                    // println!("{}",value);
                    // if let Ok(val)= value.parse::<u8>(){
                    // arc_degs.push(val);
                    // }
                    arc = false;
                }
            } else {
                if let Ok(val) = value.parse::<f32>() {
                    if i % 2 == 0 {
                        x.0 = val;
                    } else {
                        x.1 = val;
                        if arc_vals {
                            arc_vecs.last_mut().unwrap().push(x);
                        } else {
                            if wid_vec.is_empty() {
                                wid_vec.push(1);
                                wid_vecs.push(vec![]);
                            }
                            wid_vecs.last_mut().unwrap().push(x);
                        }
                    }
                    i += 1;
                }
            }
            if value == "-w" {
                wid_vecs.push(vec![]);
                arc_vals = false;
                dec = true;
            } else if value == "-a" {
                arc_vals = true;
                arc_vecs.push(vec![]);
            // arc=true;
            } else if value == "-f" {
                fill = true;
            }
        }
    }
    assert_eq!(wid_vec.len(), wid_vecs.len());
    println!("w:{:?} a:{:?}", wid_vecs, arc_vecs);
    Ok((wid_vecs, arc_vecs, wid_vec))
}

//  use std::io::{self};
use sdl2::pixels::Color;
// use std::collections::HashMap;
fn sfn(value: &str) -> Color {
    match value.to_uppercase().as_str() {
        "WHITE" => Color::WHITE,
        "BLACK" => Color::BLACK,
        "GRAY" => Color::GRAY,
        "GREY" => Color::GREY,
        "RED" => Color::RED,
        "GREEN" => Color::GREEN,
        "BLUE" => Color::BLUE,
        "MAGENTA" => Color::MAGENTA,
        "YELLOW" => Color::YELLOW,
        "CYAN" => Color::CYAN,
        _ => Color::BLACK,
    }
}
pub fn fetch_rounds()->std::result::Result<(Option<Vec<Vec<i16>>>,Option<Vec<(Color,Vec<i16>)>>,Option<Vec<Vec<i16>>>,Option<Vec<(Color,Vec<i16>)>>,Option<Vec<Vec<(f32,f32)>>>,Option<Vec<u8>>),String>{
    println!("please enter values -cf green 20 20 10 -ef yellow 20 20 10 8  ");
    let stdin=io::stdin();
    let mut buf="".into();
    if let Ok(_)=stdin.read_line(&mut buf){
let mut circle_fill=false;
let mut ellipse_fill=false;
let mut c_vect:Vec<Vec<i16>>=vec![];
let mut cf_vect:Vec<(Color,Vec<i16>)>=vec![];
let mut e_vect:Vec<Vec<i16>>=vec![];
let mut ef_vect:Vec<(Color,Vec<i16>)>=vec![];
let mut pa_vect:Vec<Vec<i16>>=vec![];
let mut pe_vect:Vec<Vec<i16>>=vec![];
let mut parallel=false;
let mut perpendicular=false;
let mut i=0;
let mut count=0;
let mut refvec:&mut Vec<i16>= &mut vec![];
let mut dirs:Vec<char>=vec![];
let mut dirs_per=vec![];
let mut wid_vec:Vec<u8>=vec![];
let mut wid=false;
buf= buf.replace("(","").replace(")","").replace(","," ").replace("\n"," ");

for value in buf.split(" "){
    if  circle_fill  || ellipse_fill || parallel || perpendicular || wid{
        i=0;
 if circle_fill{
            cf_vect.push((Color::GREEN,vec![]));
            cf_vect.last_mut().unwrap().0=sfn(value);
         
            refvec= &mut cf_vect.last_mut().unwrap().1;

count=3;
circle_fill=false;
        }else if wid{
            wid_vec.push(value.parse::<u8>().unwrap());
            wid=false;
        }
        else if ellipse_fill{
            ef_vect.push((Color::GREEN,vec![]));
                ef_vect.last_mut().unwrap().0=sfn(value);
            refvec= &mut ef_vect.last_mut().unwrap().1;

count=4;
ellipse_fill=false;
        }else if parallel{
        parallel=false;
count=7;
// println!("{}",value);
dirs.push(value.parse::<char>().unwrap());
// println!("dirs:{:?}",dirs);

if dirs.len()!=wid_vec.len(){
wid_vec.push(1);
}
    }else if perpendicular{
perpendicular=false;
count=7;
dirs_per.push(value);
if dirs.len()!=wid_vec.len(){
    wid_vec.push(1);
    }
    }}
    
    else{
if value =="-e"{
    i=0;
    e_vect.push(vec![]);
    refvec= e_vect.last_mut().unwrap();
count=4;
    
}else if value =="-ef"{
    ellipse_fill=true;
    
}else if value == "-c"{ 
    i=0;
c_vect.push(vec![]);

refvec=  c_vect.last_mut().unwrap();
count=3;

}else if value == "-cf"{
    circle_fill=true;
    
}else if value== "-pa"{
parallel=true;
// println!("{}",parallel);
    pa_vect.push(vec![]);
refvec=pa_vect.last_mut().unwrap();

}else if value=="-pe"{
perpendicular=true;
pe_vect.push(vec![]);
refvec=pe_vect.last_mut().unwrap();
}else if value=="-w"{
wid=true;
}

else{
if i<count{
    if let Ok(val)=value.parse::<i16>(){
        refvec.push(val);
    i+=1;
    }

}
}
    }
}

let mut  pa_sort_vec:Vec<(f32,Vec<(f32,f32)>)>=vec![];
let mut pe_sort_vec:Vec<(f32,Vec<(f32,f32)>)>=vec![];
let mut pa_op1:Vec<Vec<(f32,f32)>>=vec![];
let mut pe_op1:Vec<Vec<(f32,f32)>>=vec![];

for vec in pa_vect.into_iter(){
    pa_sort_vec.push((0.0,vec![]));
assign(vec, pa_sort_vec.last_mut().unwrap());
}
for vec in pe_vect.into_iter(){
    pe_sort_vec.push((0.0,vec![]));
assign(vec, pe_sort_vec.last_mut().unwrap());
}

for (distance,po) in pa_sort_vec.into_iter(){
    let dir=dirs.pop().unwrap();
     pa_op1.push(vec![po[2]]);
pa_op1.last_mut().unwrap().push(Results::point_from_distance(po,distance,dir,true));
}
for (distance,po) in pe_sort_vec.into_iter(){
    let dir=dirs.pop().unwrap();
     pe_op1.push(vec![po[2]]);

pe_op1.last_mut().unwrap().push(Results::point_from_distance(po,distance,dir,true));
}
pa_op1.append(&mut pe_op1);

let mut op:(Option<Vec<Vec<i16>>>,Option<Vec<(Color,Vec<i16>)>>,Option<Vec<Vec<i16>>>,Option<Vec<(Color,Vec<i16>)>>,Option<Vec<Vec<(f32,f32)>>>,Option<Vec<u8>>)=(None,None,None,None,None,None);

if c_vect.is_empty(){
op.0=None;
}else{
    op.0=Some(c_vect);
}
if cf_vect.is_empty(){
op.1=None;
}else{
    op.1=Some(cf_vect);
}
if e_vect.is_empty(){
op.2=None;
}else{
    op.2=Some(e_vect);
}
if ef_vect.is_empty(){
op.3=None;
}else{
    op.3=Some(ef_vect);
}
if pa_op1.is_empty(){
    op.4=None;
}else{
op.4=Some(pa_op1);
}

if wid_vec.is_empty(){
    op.5=None;
}else{
    op.5=Some(wid_vec);
}
// println!("{:?}",wid_vec);
Ok(op)

    }else{
        Err("cannot parse".to_owned())

    }
}
pub fn assign(vals: Vec<i16>,ret:&mut (f32,Vec<(f32,f32)>)){
    let mut x:(f32,f32)=(0.0,0.0);
    let mut i=true;
    for val in vals.into_iter(){
        if i {
            x.0 = val as f32;
            i=false;
        } else {
            x.1 = val as f32;
    ret.1.push(x);
    i=true;
            }
    }
    ret.0=x.0;
    }

    pub fn set_width(width:&mut u8){
        let mut buf:String="".to_owned();
        let mut r:bool=false;
        println!("please set width for mouse operations:usage:-w 2");
        if let Ok(_)=io::stdin().read_line(&mut buf){
    for val in buf.split(" "){
if val =="-w"{
r=true;
}
if r{
    if let Ok(rw)= val.parse::<u8>(){
        *width=rw;
    }
}}}}