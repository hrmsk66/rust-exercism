use num::Num;

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: Num + Copy + PartialOrd> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|x| x.is_zero())
        || sides[0] + sides[1] < sides[2]
        || sides[0] + sides[2] < sides[1] 
        || sides[1] + sides[2] < sides[0] {
            return None
        }
        Some(Triangle {
            a: sides[0],
            b: sides[1],
            c: sides[2],
        })
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.a == self.c || self.b == self.c
    }
}