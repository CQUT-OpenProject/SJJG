const EPS: f64 = 1e-9;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance_sq(self, other: Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    fn side_lengths_sq(&self) -> [f64; 3] {
        [
            self.a.distance_sq(self.b),
            self.b.distance_sq(self.c),
            self.c.distance_sq(self.a),
        ]
    }

    pub fn area(&self) -> f64 {
        ((self.b.x - self.a.x) * (self.c.y - self.a.y)
            - (self.b.y - self.a.y) * (self.c.x - self.a.x))
            .abs()
            * 0.5
    }

    pub fn is_valid(&self) -> bool {
        self.area() > EPS
    }

    pub fn is_equilateral(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let [ab, bc, ca] = self.side_lengths_sq();
        approx_eq(ab, bc) && approx_eq(bc, ca)
    }

    pub fn is_isosceles(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let [ab, bc, ca] = self.side_lengths_sq();
        approx_eq(ab, bc) || approx_eq(bc, ca) || approx_eq(ca, ab)
    }

    pub fn is_right(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        let mut sides = self.side_lengths_sq();
        sides.sort_by(|a, b| a.total_cmp(b));
        approx_eq(sides[0] + sides[1], sides[2])
    }

    pub fn contains_point(&self, p: Point) -> bool {
        if !self.is_valid() {
            return false;
        }

        let total_area = self.area();
        let a1 = Triangle::new(self.a, self.b, p).area();
        let a2 = Triangle::new(self.b, self.c, p).area();
        let a3 = Triangle::new(self.c, self.a, p).area();

        approx_eq(a1 + a2 + a3, total_area) && a1 > EPS && a2 > EPS && a3 > EPS
    }
}

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() <= EPS * (1.0 + a.abs().max(b.abs()))
}
