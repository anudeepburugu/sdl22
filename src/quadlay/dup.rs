
// pub mod circ_ext{
  
    /*
    pub fn calculate(p1:(i16,i16),p2:(i16,i16), angle:i16)->Result<((i16,i16),i16,i16,i16),String>{
        let center= Results::center_from_distance(vec![(p1.0 as i16,p1.1 as i16),(p2.0 as i16,p2.1 as i16)],angle);
        // let points= Results::shift_axis(vec![p1,p2]);
        // let (p1,p2)= (points[0],points[1]);
        let mut a1=Results::angle_from_points(vec![center.0.to_owned(),(p1.0 as i16,p1.1 as i16)]);
        let mut a2=Results::angle_from_points(vec![(center.0.0,center.0.1),(p2.0 as i16,p2.1 as i16)]);
        let mut a3=Results::angle_from_points(vec![(center.1.0,center.1.1),(p1.0 as i16,p1.1 as i16)]);
        let mut a4=Results::angle_from_points(vec![(center.1.0,center.1.1),(p2.0 as i16,p2.1 as i16)]);
        let m= Results::angle_from_points(vec![p1,p2]);
let r1= Results::distance_from_points(center.0, p1);
let r2= Results::distance_from_points(center.0, p2);
let r3= Results::distance_from_points(center.1, p2);
let r4= Results::distance_from_points(center.1, p1);

// println!("{:?} {} {}",center, -m, -m+180i16);
if a1<0.0{
a1=180.0+a1;
} 
if a2<0.0{
    a2=180.0+a2;
}
if a3<0.0{
    a3=a3+180.0;
}
if a4<0.0{
    a4=a4+180.0;
}
println!("{:?}{:?} {} {} {} {} {} {} {} {} {}",center.0,center.1, r3,r4,r1,r2,a1,a2,a3,a4,m);

if angle==180i16{
    Ok((center.1,r1,m,m+180i16))
}else if (a1-a2).abs() <=angle+15.0 && (a1-a2).abs() >= angle-15.0 {
let v1=a1;
let v2=a2;
if v1<v2{
    Ok((center.0,r1,v1,v2))
}else{
    Ok((center.0,r1,v2,v1))
}
        }
        else{
            Err("failed to fetch".to_owned())
        }
    }

}
*/
use std::io::{self};
use sdl2::pixels::Color;
// use std::collections::HashMap;
fn sfn(value:&str)->Color{
    // println!("{:?}",value.to_uppercase().as_str());

match value.to_uppercase().as_str() {
        "WHITE" =>{Color::WHITE}
        "BLACK" =>{ Color::BLACK}
        "GRAY"=> {Color::GRAY}
        "GREY"=> {Color::GREY}
        "RED"=> {Color::RED}
        "GREEN"=> {Color::GREEN}
        "BLUE"=> {Color::BLUE}
        "MAGENTA"=> {Color::MAGENTA}
        "YELLOW"=> {Color::YELLOW}
        "CYAN"=>{Color::CYAN}
        _ => {Color::BLACK}
    }}
use lib1::geometry::two_dim::Results;
use lib1::geometry::two_dim::Geom;
pub fn fetch_rounds(
    e_vect:Option<&mut Vec<Vec<i16>>>,
    ef_vect:Option<&mut Vec<(Color,Vec<i16>)>>,
    c_vect:Option<&mut Vec<Vec<i16>>>,
    cf_vect:Option<&mut Vec<(Color,Vec<i16>)>>,
    vecs:Option<&mut Vec<Vec<(f32,f32)>>>,
    wids:Option<&mut Vec<f32>>){
    println!("please enter values -
    ex: -cf green 200 200 10 -ef yellow 200 200 10 18 -w 2 -pa + 0 0 2 2 2 0 5 -pe 0 0 2 2 1 1 5  -c 200 200 10 -e 200 200 10 18");
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
let mut wid_vec:Vec<i8>=vec![];
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
            wid_vec.push(value.parse::<i8>().unwrap());
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
let mut op:(Option<Vec<Vec<i16>>>,Option<Vec<(Color,Vec<i16>)>>,Option<Vec<Vec<i16>>>,Option<Vec<(Color,Vec<i16>)>>,Option<Vec<Vec<(f32,f32)>>>,Option<Vec<Vec<(f32,f32)>>>)=(None,None,None,None,None,None,);
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
if pe_op1.is_empty(){
op.5=None;
}else{
    op.5=Some(pe_op1);
}
// println!("{:?}",wid_vec);
// Ok(op)

    // }else{
        // Err("cannot parse".to_owned())

    // }
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

fn main(){
let vals= fetch_rounds().unwrap();
    // println!("{:?}",vals);
}