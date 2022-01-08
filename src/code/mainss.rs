mod dep1;
use sdl2::rect::Point;
use dep1::points::PointA;
fn main(){
let po=PointA((100,0),None,None);
let ps:Point=po.into();
println!("{:?}",ps);
}
pub fn read_points()->std::result::Result<Vec<(f32,f32)>,String>{

    let mut vals:Vec<(f32,f32)>=vec![];
// let fs= a.match_indices("-f").collect();
let mut arc=false;
let mut arc_flag=false;
let mut fill:String="".into();
let mut fill_flag=false;
let mut width=0;
let width_flag=false;

let mut w:i32=1;
    println!("please enter (x,y) values separated by commas, -w for width, -a for arc, -f to fill
    ex:(0,0),(5.5,5.5) -w 2 
    -a (0,0),(3,3) -w 2 (3,4),(7,7) -f b -w 3
    ;");    
    let stdin=std::io::stdin();
    let mut buf="".into();
    if let Ok(_)=stdin.read_line(&mut buf){
    // let mut values= buf.split(" ").collect::<Vec<&str>>();
    let mut values:Vec<(usize,char)>= buf.char_indices().collect::<Vec<(usize,char)>>();

    // std::iter::Iterator
    
    for (idx,value) in values{
match(value){
    'a' =>{
        arc_flag=true;

    },
    'f' =>{
fill_flag=true;
    },
    'w' =>{
width_flag=true;
    },
    '0'..='9'=>{

    }
    // format!("{}",0..9) =>{

    // },
    '.' =>{

    },

    'a'..='z' =>{

    }
    '(' =>{

    },

    ')'=>{

    }

    _=>{}
}
    }


}
Ok(vec![(3.0,3.0)])
}
/*
 let mut x:(f32,f32)=(0.0,0.0);
let mut i=0;
for value in buf.split(" "){
    if let Ok(val)=value.parse::<f32>(){
        if i%2==0{
x.0=val;
        }else{
         x.1=val;
vals.push(x);
        }
        i+=1;
    }}}
assert!(vals.len() >= 2);
if vals.len()>=2{
    Ok(vals)
}else{
Err("please enter again".into())
}
    }*/
    /*A => {
            if let Ok(mut vecs) = fetch_values() {
                for i in 0..2 {
                    vecs[i].0 = vecs[i].0 + 10.0;
                }
                // println!("{:?}",vecs);

                if let Ok((point, radius, mut start, mut end)) =
                    calculate(vecs.to_owned(), 180.0, true)
                {
                    if *lshift {
                        let mid = start;
                        start = end;
                        end = mid;
                        arc_vecd.push((
                            vecs,
                            (
                                (point.0 * 10.0).floor() as i16 + 100,
                                (point.1 * 10.0).ceil() as i16,
                                radius as i16 * 10,
                                end as i16,
                                start as i16,
                                Color::BLACK,
                            ),
                        ));
                        *lshift = false;
                    } else {
                        arc_veci.push((
                            vecs,
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
                        println!("{:?}",arc_veci.last().unwrap().0);
                        canvas.present();
                    }
                }
            }
        }
*/