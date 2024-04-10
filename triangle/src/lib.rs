use itertools::Itertools;
use triangle_tests::*;

pub struct Triangle<T>([T; 3]);

impl<T: Into<f64> + PartialEq + PartialOrd + Clone + Copy + std::ops::Add> Triangle<T>
    where <T as std::ops::Add>::Output: PartialOrd<T>
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if !triangle_test_alternative(&sides) {
            return None;
        }

        Some(Triangle(sides))
    }

    pub fn is_equilateral(&self) -> bool {
        let mut sides = self.0.into_iter();
        let first = sides.next().unwrap();
        sides.all(|side| side == first)
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_isosceles()
    }

    pub fn is_isosceles(&self) -> bool {
        self.0
            .into_iter()
            .combinations(2)
            .any(|sides| sides[0] == sides[1])
    }
}

pub mod triangle_tests {
    pub fn triangle_test<T: Into<f64> + PartialEq + PartialOrd + Clone + Copy + std::ops::Add>(
        sides: &[T; 3]
    ) -> bool {
        let a: f64 = sides[0].into();
        let b: f64 = sides[1].into();
        let c: f64 = sides[2].into();

        let angle_a = angle(&a, &b, &c);
        let angle_b = angle(&b, &c, &a);
        let angle_c = angle(&a, &c, &b);

        let sum = (angle_a + angle_b + angle_c) as f32;
        sum == std::f32::consts::PI
    }
    fn angle(side1: &f64, side2: &f64, side3: &f64) -> f64 {
        ((side1.powf(2.0) + side2.powf(2.0) - side3.powf(2.0)) / 2.0 / side1 / side2).acos()
    }

    pub fn triangle_test_alternative<
        T: Into<f64> + PartialEq + PartialOrd + Clone + Copy + std::ops::Add
        >(sides: &[T; 3]) -> bool where <T as std::ops::Add>::Output: PartialOrd<T> {
        sides[0] + sides[1] > sides[2] &&
            sides[0] + sides[2] > sides[1] &&
            sides[1] + sides[2] > sides[0]
    }
}
