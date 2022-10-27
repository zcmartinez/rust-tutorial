

fn main() {

}

fn consume_s(s: String)->usize{
    s.len()
}
enum State<T, Q=i32>{
    ON(Q),
    OFF(T),
}

mod topology{
    pub  struct point{
        x:f64,
        y:f64
    }
    pub struct square{
        p_tl:point,
        p_br:point
    }

    impl point {
        pub fn new(x: f64, y:f64) -> Self{
            Self{
                x,
                y
            }
        }

        pub fn x(&self)-> f64{
            self.x
        }
        pub fn y(&self)-> f64{
            self.y
        }
    }

    fn max(a: f64, b: f64) -> f64 {
        if a > b {
            a
        } else {
            b
        }
    }
    fn min(a: f64, b: f64) -> f64 {
        if a < b {
            a
        } else {
            b
        }
    }

    impl square{
        pub fn new(p1:point, p2:point) -> Self{
            Self{
                p_tl:p1,
                p_br:p2
            }
        }

        pub fn tl(&self) -> point {
            point {
                x: self.p_tl.x,
                y: self.p_tl.y,
            }
        }

        pub fn br(&self) -> point {
            point {
                x: self.p_br.x,
                y: self.p_br.y,
            }
        }

        pub fn height (&self) -> f64 {
            self.p_tl.y - self.p_br.y
        }
        pub fn width (&self) -> f64 {
            self.p_br.x - self.p_tl.x
        }

        pub fn area (&self) -> f64 {
            self.width()*self.height()
        }

        pub fn dilat(&mut self, d:f64) -> (){
            self.p_tl.x = self.p_tl.x - d;
            self.p_tl.y = self.p_tl.y + d;
            self.p_br.x = self.p_br.x + d;
            self.p_br.y = self.p_br.y - d;
        }

        pub fn intersection(&self, other: &Self) -> Self {
            let x1 = max(self.p_tl.x, other.p_tl.x);
            let y1 = max(self.p_tl.y, other.p_tl.y);
            let x2 = min(self.p_br.x, other.p_br.x);
            let y2 = min(self.p_br.y, other.p_br.y);

            if x1 > x2 || y1 > y2 {
                square::new(point::new(0.0, 0.0), point::new(0.0, 0.0))
            } else {
                square::new(point::new(x1, y1), point::new(x2, y2))
            }
        }

        pub fn union(&self, other: &Self) -> Self {
            let x1 = min(self.p_tl.x, other.p_tl.x);
            let y1 = min(self.p_tl.y, other.p_tl.y);
            let x2 = max(self.p_br.x, other.p_br.x);
            let y2 = max(self.p_br.y, other.p_br.y);
            square::new(point::new(x1, y1), point::new(x2, y2))
        }
    }

}

#[cfg(test)]
mod test {
    use crate::topology::{point, square};

    #[test]
    fn point_test(){
        let p = point::new(10.0, 10.0);

        assert_eq!(p.y(), 10.0);
        assert_eq!(p.x(), 10.0);
    }

    #[test]
    fn sq_test(){
        let p1:point = point::new(0.0, 2.0);
        let p2:point = point::new(1.0,0.0);
        let sq:square = square::new(p2,p1);
        assert_eq!(sq.area(),2.0);
    }

    #[test]
    fn dilat_test(){
        let p1:point = point::new(0.0, 2.0);
        let p2:point = point::new(1.0,0.0);
        let mut sq:square = square::new(p1,p2);

        sq.dilat(2.0);


        assert_eq!(sq.area(), 30.0);
    }

    #[test]
    fn intersection_test_corner() {
        let s1: square = square::new(point::new(0.0, 0.0), point::new(3.0, 3.0));
        let s2: square = square::new(point::new(1.0, 1.0), point::new(4.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.tl().x(), 1.0);
        assert_eq!(s3.tl().y(), 1.0);
        assert_eq!(s3.br().x(), 3.0);
        assert_eq!(s3.br().y(), 3.0);
    }

    #[test]
    fn intersection_test_cross() {
        let s1: square = square::new(point::new(0.0, 0.0), point::new(3.0, 3.0));
        let s2: square = square::new(point::new(1.0, -1.0), point::new(2.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.tl().x(), 1.0);
        assert_eq!(s3.tl().y(), 0.0);
        assert_eq!(s3.br().x(), 2.0);
        assert_eq!(s3.br().y(), 3.0);
    }

    #[test]
    fn intersection_test_out() {
        let s1: square = square::new(point::new(0.0, 0.0), point::new(3.0, 3.0));
        let s2: square = square::new(point::new(5.0, 5.0), point::new(10.0, 10.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.tl().x(), 0.0);
        assert_eq!(s3.tl().y(), 0.0);
        assert_eq!(s3.br().x(), 0.0);
        assert_eq!(s3.br().y(), 0.0);
    }

    #[test]
    fn union_test_cross() {
        let s1: square = square::new(point::new(0.0, 0.0), point::new(3.0, 3.0));
        let s2: square = square::new(point::new(1.0, -1.0), point::new(2.0, 4.0));
        let s3 = s1.union(&s2);
        assert_eq!(s3.tl().x(), 0.0);
        assert_eq!(s3.tl().y(), -1.0);
        assert_eq!(s3.br().x(), 3.0);
        assert_eq!(s3.br().y(), 4.0);
    }
}