use super::image::GrayImage;
use super::helpers::pixel_value;

/// Cell is a square that represents size*size pixels of the original image with
/// a single number. It is used to track density of edges. The larger the cell
/// size the lower the resolution of the heat map. The lower the cell size the
/// less abstract the heat map becomes. It has to be a number that is divides
/// both image width and image hight without a rest.
pub const CELL_SIZE: u32 = 10;

type GrayImageRaw = Vec<Vec<u32>>;

/// Transforms the bricked heat map where the cells are of CELL_SIZE to a more
/// granular one where cells are CELL_SIZE / 2. This gives us better detail
/// while preserving relationships between all parts of the image rather than
/// cropping out a block and calculating the heat separately.
pub fn heat_map(image: &GrayImage) -> (GrayImageRaw, u32, u32) {
  let (width, height) = image.dimensions();
  let bricked_heat_map: GrayImageRaw = bricked_heat_map(&image);

  let mut heat_max: u32 = 1;
  let mut heat_total: u32 = 0;
  let mut heat_counter: u32 = 1;
  let mut heat_map: GrayImageRaw = Vec::new();

  for offset_y in 0..(2 * height / CELL_SIZE) {
    let mut row: Vec<u32> = vec!();

    for offset_x in 0..(2 * width / CELL_SIZE) {
      // Sums the heat of all cells that participate to given offset and divides
      // it by 4. This will result in very low heat near the edges of the image.
      let heat: u32 = {
        let x: isize = offset_x as isize;
        let y: isize = offset_y as isize;

        (pixel_value(&bricked_heat_map, x, y, 0) +
        pixel_value(&bricked_heat_map, x, y - 1, 0) +
        pixel_value(&bricked_heat_map, x - 1, y, 0) +
        pixel_value(&bricked_heat_map, x - 1, y - 1, 0)) as u32
      } / 4;

      // Updates maximum observed heat.
      heat_max = heat_max.max(heat);

      // Adds info to heat average calculations.
      if heat > 0 {
        heat_total += heat;
        heat_counter += 1;
      }

      row.push(heat);
    }

    heat_map.push(row);
  }

  (heat_map, heat_max, heat_total / heat_counter)
}

/// Calculates the heat map of overlaying cells. Most pixels therefore belong
/// to 4 cells. Pixels on the edges of the image belong to 2 cells and pixels
/// in the corners belong to one cell.
///
/// In the following diagram, there are 4 cells where each cell is of the same
/// size (e.g. cell 0x0 contains CELL_SIZE*CELL_SIZE pixels).
/// a: row 0, col 0
/// b: row 0, col 1
/// c: row 1, col 0
/// d: row 1, col 1
///
///   ____0___________1_____
/// 0 |   a    ab     b...
///   |   ac   abcd   bd...
/// 1 |   c... cd...  d...
///
fn bricked_heat_map(image: &GrayImage) -> GrayImageRaw {
  let (width, height) = image.dimensions();

  // We want the cells to overlay one another by half of their size. Therefore
  // we can fit one full stack of cells plus one on top of it, but the second
  // one starts with padding of CELL_SIZE / 2, therefore the overlay will fit
  // one cell less.
  let rows = (2 * height / CELL_SIZE) - 1;
  let columns = (2 * width / CELL_SIZE) - 1;

  let mut heat_map: GrayImageRaw = Vec::new();

  for offset_y in 0..rows {
    let mut row: Vec<u32> = Vec::new();

    for offset_x in 0..columns {
      let mut heat: u32 = 0;

      // Counts number of black pixels (in the image the pixels are black and
      // white only) in given cell.
      for cell_y in 0..CELL_SIZE {
        for cell_x in 0..CELL_SIZE {
          // Gets the value of the pixel on position that is padded by the
          // offset plus the current cell index.
          let pixel = image.get_pixel(
            (offset_x * CELL_SIZE / 2) + cell_x,
            (offset_y * CELL_SIZE / 2) + cell_y,
          );

          if pixel.data[0] == 0 {
            heat += 1;
          }
        }
      }

      row.push(heat);
    }

    heat_map.push(row);
  }

  heat_map
}
