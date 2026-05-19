use std::ops::Mul;

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point <T> {
    pub fn new(x: &T, y: &T) -> Option<Self>
    where T: Copy + Default + PartialEq {
        //lame framework signature if new may not produce a valid value based on inputs
        //panic or return option?
        if *x == T::default() && *y == T::default() {
            None
        } else {
            Some(Point{x: *x, y: *y})
        }
    }
}

impl<T: Mul<Output = T>> Mul for Point<T> {
    type Output = Point<T>;

    //note: not a dot or cross product in the slightest
    fn mul(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

