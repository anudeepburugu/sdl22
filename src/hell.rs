fn arad(a: (i32, i32), b: (i32, i32)) -> f64 {
    if b.0 - a.0 != 0 {
      let val=  (180f64 / std::f64::consts::PI) * ((((b.1 - a.1)as f64 / (b.0 - a.0) as f64)).atan());
      println!("{}",val);
      if val>90f64{
          90f64-val
      }else{
          val
      }
    } else {
        90f64
    }
}
fn main(){
    arad((56, 39), (2, 45));
    println!("{}",(-1f32/9f32).atan());
}