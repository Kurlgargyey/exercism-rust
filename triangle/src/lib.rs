pub struct Triangle([u64; 3]);

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        Some(Triangle(sides))
    }

    pub fn is_equilateral(&self) -> bool {
        let mut sides = self.0.into_iter();
        let first = sides.next().unwrap();
        sides.all(|side| side == first)
    }

    pub fn is_scalene(&self) -> bool {
        todo!("Determine if the Triangle is scalene.");
    }

    pub fn is_isosceles(&self) -> bool {
        todo!("Determine if the Triangle is isosceles.");
    }
}
