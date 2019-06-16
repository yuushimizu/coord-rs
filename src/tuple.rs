use crate::coord::Coord;

impl<C: Coord> Coord for (C,) {
    type Item = (<C as Coord>::Item,);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        (<C as Coord>::from_x_y(x.0, y.0),)
    }

    /// # Examples
    /// ```
    /// # use coord::Coord;
    /// # use coord::Point;
    /// assert_eq!((10,), (Point::new(10, 20),).x());
    /// ```
    fn x(&self) -> Self::Item {
        (self.0.x(),)
    }

    /// # Examples
    /// ```
    /// # use coord::Coord;
    /// # use coord::Point;
    /// assert_eq!((20,), (Point::new(10, 20),).y());
    /// ```
    fn y(&self) -> Self::Item {
        (self.0.y(),)
    }
}

impl<C0: Coord, C1: Coord> Coord for (C0, C1) {
    type Item = (<C0 as Coord>::Item, <C1 as Coord>::Item);

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        (C0::from_x_y(x.0, y.0), C1::from_x_y(x.1, y.1))
    }

    /// # Examples
    /// ```
    /// # use coord::Coord;
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!((1, 2), (Vector::new(1, 10), Point::new(2, 20)).x());
    /// ```
    fn x(&self) -> Self::Item {
        (self.0.x(), self.1.x())
    }

    /// # Examples
    /// ```
    /// # use coord::Coord;
    /// # use coord::Point;
    /// # use coord::Vector;
    /// assert_eq!((10, 20), (Vector::new(1, 10), Point::new(2, 20)).y());
    /// ```
    fn y(&self) -> Self::Item {
        (self.0.y(), self.1.y())
    }
}

impl<C0: Coord, C1: Coord, C2: Coord> Coord for (C0, C1, C2) {
    type Item = (
        <C0 as Coord>::Item,
        <C1 as Coord>::Item,
        <C2 as Coord>::Item,
    );

    fn from_x_y(x: Self::Item, y: Self::Item) -> Self {
        (
            C0::from_x_y(x.0, y.0),
            C1::from_x_y(x.1, y.1),
            C2::from_x_y(x.2, y.2),
        )
    }

    /// # Examples
    /// ```
    /// # use coord::Coord;
    /// # use coord::Point;
    /// # use coord::Vector;
    /// # use coord::Size;
    /// assert_eq!((1, 2, 3), (Vector::new(1, 10), Point::new(2, 20), Size::new(3, 30)).x());
    /// ```
    fn x(&self) -> Self::Item {
        (self.0.x(), self.1.x(), self.2.x())
    }

    /// # Examples
    /// ```
    /// # use coord::Coord;
    /// # use coord::Point;
    /// # use coord::Vector;
    /// # use coord::Size;
    /// assert_eq!((10, 20, 30), (Vector::new(1, 10), Point::new(2, 20), Size::new(3, 30)).y());
    /// ```
    fn y(&self) -> Self::Item {
        (self.0.y(), self.1.y(), self.2.y())
    }
}
