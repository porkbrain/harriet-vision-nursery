/// Helper function for accessing values at given address in vector. If the
/// address is out of bounds, it delivers the default value instead.
pub fn pixel_value<T: Copy>(vec: &Vec<Vec<T>>, x: isize, y: isize, default: T) -> T {
  if x < 0 || y < 0 {
    return default;
  }

  match vec.get(y as usize) {
    None => default,
    Some(row) => match row.get(x as usize) {
      None => default,
      Some(value) => *value,
    },
  }
}

/// Converts multi dimensional vector of integers that represent colour into
/// a multi dimensional vector of booleans where true means given point is
/// highlighted.
pub fn to_point_map(input: Vec<Vec<u32>>) -> Vec<Vec<bool>> {
  input.iter().map(
    |row| row.iter().map(|point| *point != 0).collect()
  ).collect()
}
