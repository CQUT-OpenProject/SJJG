use ex_3::{Point, Triangle};

fn assert_close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < 1e-9);
}

#[test]
fn detects_equilateral_triangle() {
    let t = Triangle::new(
        Point::new(0.0, 0.0),
        Point::new(1.0, 0.0),
        Point::new(0.5, (3.0_f64).sqrt() / 2.0),
    );

    assert!(t.is_equilateral());
    assert!(t.is_isosceles());
    assert!(!t.is_right());
}

#[test]
fn detects_isosceles_and_right_triangle() {
    let t = Triangle::new(
        Point::new(0.0, 0.0),
        Point::new(3.0, 0.0),
        Point::new(0.0, 4.0),
    );

    assert!(!t.is_equilateral());
    assert!(!t.is_isosceles());
    assert!(t.is_right());
    assert_close(t.area(), 6.0);
}

#[test]
fn point_inside_triangle_strict() {
    let t = Triangle::new(
        Point::new(0.0, 0.0),
        Point::new(4.0, 0.0),
        Point::new(0.0, 3.0),
    );

    assert!(t.contains_point(Point::new(1.0, 1.0)));
    assert!(!t.contains_point(Point::new(5.0, 1.0)));
    assert!(!t.contains_point(Point::new(2.0, 0.0)));
}

#[test]
fn degenerate_triangle_is_invalid() {
    let t = Triangle::new(
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 2.0),
    );

    assert!(!t.is_valid());
    assert_close(t.area(), 0.0);
    assert!(!t.is_equilateral());
    assert!(!t.is_isosceles());
    assert!(!t.is_right());
    assert!(!t.contains_point(Point::new(1.0, 1.0)));
}
