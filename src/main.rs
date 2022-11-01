fn main() {}

fn consume_s(s: String) -> usize {
    s.len()
}

enum State<T, Q = i32> {
    ON(Q),
    OFF(T),
}

mod topology {
    pub struct Point {
        x: f64,
        y: f64,
    }

    pub struct Square {
        p_tl: Point,
        p_br: Point,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn x(&self) -> f64 {
            self.x
        }
        pub fn y(&self) -> f64 {
            self.y
        }
    }

    impl Square {
        pub fn new(p1: Point, p2: Point) -> Self {
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            Self {
                p_tl: Point::new(min_x, min_y),
                p_br: Point::new(max_x, max_y),
            }
        }

        pub fn lower(&self) -> &Point {
            &self.p_tl
        }
        pub fn upper(&self) -> &Point {
            &self.p_br
        }

        pub fn height(&self) -> f64 {
            f64::abs(self.p_tl.y - self.p_br.y)
        }
        pub fn width(&self) -> f64 {
            f64::abs(self.p_br.x - self.p_tl.x)
        }

        pub fn area(&self) -> f64 {
            self.width() * self.height()
        }

        pub fn erosion(&mut self, d: f64) {
            self.p_tl.x = self.p_tl.x + d;
            self.p_tl.y = self.p_tl.y + d;
            self.p_br.x = self.p_br.x - d;
            self.p_br.y = self.p_br.y - d;
        }

        pub fn dilate(&mut self, d: f64) -> () {
            self.p_tl.x = self.p_tl.x - d;
            self.p_tl.y = self.p_tl.y - d;
            self.p_br.x = self.p_br.x + d;
            self.p_br.y = self.p_br.y + d;
        }

        pub fn dilate_x(&mut self, d: f64) -> () {
            let wth = self.width() * 0.5 * d;
            let mid_x = (self.p_br.x - self.p_tl.x) * 0.5;
            self.p_tl.x = mid_x - wth;
            self.p_br.x = mid_x + wth;
        }

        pub fn dilate_y(&mut self, d: f64) -> () {
            let wth = self.height() * 0.5 * d;
            let mid_y = (self.p_tl.y - self.p_tl.y) * 0.5;
            self.p_tl.y = mid_y + wth;
            self.p_br.y = mid_y - wth;
        }

        pub fn erosion_x(&mut self, d: f64) -> () {
            self.dilate_x(1.0 / d);
        }

        pub fn erosion_y(&mut self, d: f64) -> () {
            self.dilate_y(1.0 / d);
        }

        pub fn has_point(&self, p1: &Point) -> bool {
            todo!()
        }

        pub fn has_square(&self, sq: &Square) -> bool {
            todo!()
        }

        pub fn manhattan_distance(&self, sq: &Square) -> f64 {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::topology::{Point, Square};

    #[test]
    fn point_test() {
        let p = Point::new(10.0, 10.0);
        assert_eq!(p.y(), 10.0);
        assert_eq!(p.x(), 10.0);
    }

    #[test]
    fn sq_test() {
        let p1: Point = Point::new(0.0, 0.0);
        let p2: Point = Point::new(1.0, 2.0);
        let sq: Square = Square::new(p1, p2);
        //assert_eq!(sq.area(),2.0);
        assert!(sq.lower().x() < sq.upper().x());
        assert!(sq.lower().y() < sq.upper().y());
    }

    #[test]
    fn dilate_test() {
        let p1: Point = Point::new(0.0, 2.0);
        let p2: Point = Point::new(1.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);

        sq.dilate(2.0);

        assert_eq!(sq.area(), 30.0);
    }

    #[test]
    fn erosion_test() {
        let p1: Point = Point::new(0.0, 4.0);
        let p2: Point = Point::new(4.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);

        sq.erosion(0.5);

        assert_eq!(sq.area(), 9.0);
    }
}
