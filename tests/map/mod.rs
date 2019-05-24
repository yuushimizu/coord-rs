use coord::prelude::*;

#[test]
fn map_test() {
    assert_eq!(Point::new(6, 14), Point::new(3, 7).map(|n| n * 2));
}

#[test]
fn map_to_other_type() {
    assert_eq!(Size::new(6, 14), Point::new(3, 7).map(|n| n * 2));
}

#[test]
fn map_to_rect() {
    assert_eq!(
        Rect::new(Point::new(1, 2), Size::new(10, 20)),
        Point::new(1, 2).map(|n| (n, n * 10))
    );
}

#[test]
fn map_pair() {
    assert_eq!(
        Point::new(103, 208),
        (Point::new(100, 200), Vector::new(3, 8)).map(|(p, v)| p + v)
    );
}

#[test]
fn map_triple() {
    assert_eq!(
        Vector::new(91, 182),
        (
            Vector::new(1, 2),
            Vector::new(10, 20),
            Vector::new(100, 200)
        )
            .map(|(a, b, c)| a - b + c)
    );
}

#[test]
fn map_to_tuple() {
    assert_eq!(
        (Point::new(11, 21), Size::new(20, 40)),
        Vector::new(10, 20).map(|n| (n + 1, n * 2))
    );
}
