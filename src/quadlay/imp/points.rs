use sdl2::rect::Point;
use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::*;
#[derive(Copy, Clone)]
pub struct PointA(
    pub (i32, i32),
    pub Option<(i32, i32)>,
    pub Option<(i32, i32)>,
);

impl Into<Point> for PointA {
    fn into(self) -> Point {
        if self.1 != None && self.2 != None {
            Point::new(
                (self.0 .0 * self.1.unwrap().0) + self.2.unwrap().0,
                (self.0 .1 * self.1.unwrap().1) + self.2.unwrap().1,
            )
        } else if self.2 == None && self.1 != None {
            Point::new(self.0 .0 * self.1.unwrap().0, self.0 .1 * self.1.unwrap().1)
        } else if self.2 != None && self.1 == None {
            Point::new(self.0 .0 + self.2.unwrap().0, self.0 .1 + self.2.unwrap().1)
        } else {
            Point::new(self.0 .0, self.0 .1)
        }
    }
}

// Note: explicit lifetime parameter says that this value lives atleast aslongas the life of
pub struct PointsA<'a, T>
where
    T: Mul<Output = T> + Add<Output = T> + PartialEq + Copy + Display,
{
    pub scale: Option<(T, T)>,
    pub shift: Option<(T, T)>,
    pub values: &'a mut Vec<Point>,
    pub x_y: &'a Vec<(T, T)>,
}

impl<'a, T> Into<&'a [Point]> for PointsA<'a, T>
where
    T: Mul<Output = T> + Display + Add<Output = T> + PartialEq + Copy,
{
    fn into(self) -> &'a [Point] {
        for (x, y) in self.x_y.iter() {
            if self.scale != None && self.shift != None {
                self.values.push(Point::new(
                    conv(*x * self.scale.unwrap().0 + self.shift.unwrap().0),
                    conv(*y * self.scale.unwrap().1 + self.shift.unwrap().1),
                ));
            } else if self.scale == None && self.shift != None {
                self.values.push(Point::new(
                    conv(*x + self.shift.unwrap().0),
                    conv(*y + self.shift.unwrap().1),
                ));
            } else if self.scale != None && self.shift == None {
                self.values.push(Point::new(
                    conv(*x * self.scale.unwrap().0),
                    conv(*y * self.scale.unwrap().1),
                ));
            } else {
                self.values.push(Point::new(conv(*x), conv(*y)));
            }
        }
        self.values.as_slice()
    }
}
fn conv<T: Display>(a: T) -> i32 {
    let fted = format!("{}", a);
    fted.parse::<f32>().unwrap() as i32
}
