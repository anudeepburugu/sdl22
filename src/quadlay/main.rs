
#[allow(unused)]
#[allow(dead_code)]
#[allow(unused_must_use)]
#[allow(unused_imports)]
#[macro_use]
mod imp;
use imp::qfsa;
use lib1::geometry::two_dim::*;
fn main(){  

let results= &mut Results{
    qfv:0.0,
    tfv:0.0,
};

   let args:Vec<String>= std::env::args().collect();
if args.len()<2 || args[1].eq("-h") || args[1].eq("--help")  {
println!(
"
usage: 
filename --qfs 10.5 12 10.5 12 for area of quadrilateral \n
filename --qfv (10,20) (12,20) (10,40) (12,40) for area of quadrilateral \n
filename --qfs-ft 10 1 12 3.5 10 5 10 11 feet followed by inches for area of quadrilateral in sft
filename --qfsd 5 6 2.2 5 6  middle value diagonal
filename --qfsa 5 6 5 6 30 to get shape of the quadrilateral
filename --qfsa-ft 105 0 123 0 105 0 123 0 12946.5   feet followed by inches for shape of the quadrilateral

--help : prints this message
");
}else {
match args[1].as_ref() {
    "--qfsa" =>{
        let value1:f32= args[2].to_owned().parse::<f32>().expect("parsing error!");
        let value2:f32=args[3].to_owned().parse::<f32>().expect("parsing error!");
         let value3:f32= args[4].to_owned().parse::<f32>().expect("parsing error!");
         let value4:f32=args[5].to_owned().parse::<f32>().expect("parsing error!");
            let  value5:f32= args[6].to_owned().parse::<f32>().expect("parsing error!");
  qfsa(value1,value2,value3,value4,value5);  
 }
    "--qfsa-ft"=>{
   let value1:f32= args[2].to_owned().parse::<f32>().expect("parsing error!")+args[3].to_owned().parse::<f32>().expect("parsing error!")/12f32;
        let value2:f32= args[4].to_owned().parse::<f32>().expect("parsing error!")+args[5].to_owned().parse::<f32>().expect("parsing error!")/12f32;
           let  value3:f32= args[6].to_owned().parse::<f32>().expect("parsing error!")+args[7].to_owned().parse::<f32>().expect("parsing error!")/12f32;
            let value4:f32= args[8].to_owned().parse::<f32>().expect("parsing error!")+args[9].to_owned().parse::<f32>().expect("parsing error!")/12f32;
 let value5=args[10].parse::<f32>().expect("parse error");
 qfsa(value1,value2,value3,value4,value5);  

    }


"--sft"=>{
   if let Ok((value1,value2))= Results::side_from_area(Sides{
        side1: args[3].to_owned().parse::<f32>().expect("parsing error!"),
        side2:args[4].to_owned().parse::<f32>().expect("parsing error!"),
        side3:0.0},args[2].to_owned().parse::<f32>().expect("parsing error!")){
 println!("sides:{} {}",value1,value2);
        }else{
 println!("try with other numbers");

        }

}

    "--qfsd"=>{
        let result=
        results.area_from_sides(
            Sides{
        side1:args[2].to_owned().parse::<f32>().expect("parsing error!"),
        side2:args[3].to_owned().parse::<f32>().expect("parsing error!"),
        side3:args[4].to_owned().parse::<f32>().expect("parsing error!")}).expect("error")+
        results.area_from_sides(
            Sides{side1:args[4].to_owned().parse::<f32>().expect("parsing error!"),
       side2:args[5].to_owned().parse::<f32>().expect("parsing error!"),
         side3:args[6].to_owned().parse::<f32>().expect("parsing error!")}).expect("error");
//  println!("area in sft:{} sqyds: {}",result,result/9f32);
 println!("area:{}",result);
    
    }


/*
    
"--tfsas" => {
let value=
 calculate(
    Areas::FromSAS(
        results.area_from_sas(
        SAS{
            side1:args[2].to_owned().parse().expect("parsing error!"),
            side2:args[3].to_owned().parse().expect("parsing error!")
            ,angle:args[4].to_owned().parse().expect("parsing error!")
        })
        ).expect("usage error");
println!("{}",value.unwrap());
}

"--qfs" => {
    sdl_qfs(
            args[2].to_owned().parse().expect("parsing error!"),
            args[3].to_owned().parse().expect("parsing error!"),
            args[4].to_owned().parse().expect("parsing error!"),
            args[5].to_owned().parse().expect("parsing error!"),);

      

}
*/
"--tfs" => {
    let value= 
    // calculate(
    // Areas::FromSides(
results.area_from_sides(
        Sides{
            side1:args[2].to_owned().parse().expect("parsing error!"),
            side2:args[3].to_owned().parse().expect("parsing error!")
            ,side3:args[4].to_owned().parse().expect("parsing error!")
        });
        // ).expect("usage error");
println!("{}",value.unwrap());

}
"--qfv" =>{
    results.quadarea_from_vertices(
        Qvertices{
            point1:(args[2].to_owned().parse().expect("parsing error!"),args[3].to_owned().parse().expect("parsing error!")),
            point2:(args[4].to_owned().parse().expect("parsing error!"),args[5].to_owned().parse().expect("parsing error!")),
            point3:(args[6].to_owned().parse().expect("parsing error!"),args[7].to_owned().parse().expect("parsing error!")),
            point4:(args[8].to_owned().parse().expect("parsing error!"),args[9].to_owned().parse().expect("parsing error!")),

        });
println!("{}",results.qfv);

}

"--tfv" =>{
    // let value= 
// calculate(
    // Areas::FromVertices(
        results.area_from_vertices(
        Vertices{
            point1:(args[2].to_owned().parse().expect("parsing error!"),args[3].to_owned().parse().expect("parsing error!")),
            point2:(args[4].to_owned().parse().expect("parsing error!"),args[5].to_owned().parse().expect("parsing error!")),
            point3:(args[6].to_owned().parse().expect("parsing error!"),args[7].to_owned().parse().expect("parsing error!")),

        });
        // ).expect("usage error");
println!("{}",results.qfv);

}



_ =>{
    println!("usage error")
}} }
}