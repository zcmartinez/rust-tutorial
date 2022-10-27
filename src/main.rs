

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
}