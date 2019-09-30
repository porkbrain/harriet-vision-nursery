use std::fmt;

#[derive(Copy, Clone)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

impl Point {

  pub fn new(x: u32, y: u32) -> Point {
    Point { x, y }
  }

}

impl PartialEq for Point {

  fn eq(&self, other: &Point) -> bool {
    self.x == other.x && self.y == other.y
  }

}

impl std::ops::Add for Point {

  type Output = Point;

  fn add(self, other: Point) -> Point {
      Point {
        x: self.x + other.x,
        y: self.y + other.y,
      }
  }

}

impl std::fmt::Debug for Point {

  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_fmt(
      format_args!("P({};{})", self.x, self.y)
    )
  }

}
