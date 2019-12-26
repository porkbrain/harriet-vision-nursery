use super::helpers::{pixel_value, to_point_map};

type GrayImageRaw = Vec<Vec<u32>>;

/// Runs the automaton until all cells are stabilized (positively dead or alive)
/// which corresponds to their heat values of 0 to max. The rules are based on
/// their surrounding heat within the Moore neighbourhood. The resulting vector
/// highlights important objects in the image.
pub fn cellular_automaton(mut image: GrayImageRaw, max: u32, mean: u32) -> Vec<Vec<bool>> {
    // This loop break once there has been no change in the previous cycle, which
    // means the map is stabilized.
    loop {
        // Flag for breaking the cycle.
        let mut stabilized = true;
        // New status of the map after this cycle.
        let mut step_map: GrayImageRaw = Vec::new();

        for (y, map_row) in image.iter().enumerate() {
            let mut step_map_row: Vec<u32> = Vec::new();

            for (x, heat) in map_row.iter().enumerate() {
                // If the cell is stabilized (either fully dead or alive), skip it.
                if *heat == max || *heat == 0 {
                    step_map_row.push(*heat);
                    continue;
                }

                stabilized = false;

                // Find the average heat in Moore neighbourhood.
                let surrounding_heat: u32 = neighborhood_heat(&image, x, y);

                // Rule #1:
                // If the surrounding heat is less than the smaller value of out average
                // map heat or cell heat, cell dies. Cell also dies if the surrounding
                // heat is equal mean heat.
                if surrounding_heat <= mean.min(*heat) || surrounding_heat == mean {
                    step_map_row.push(0);
                    continue;
                }

                // Rule #2:
                // If the surrounding heat is lower than the average, the cell decreases
                // its heat by that difference.
                if surrounding_heat < mean {
                    step_map_row
                        .push(0.max(*heat as i32 - ((mean + surrounding_heat) as i32) * 2) as u32);
                    continue;
                }

                // Rule #3:
                // If the surrounding heat is larger than or equal to the average heat,
                // the cell increases its heat by that difference.
                step_map_row.push(max.min(*heat + (surrounding_heat - mean) * 2));
            }

            step_map.push(step_map_row);
        }

        if stabilized {
            break;
        }

        // Updates the map to its new evolvement.
        image = step_map;
    }

    to_point_map(image)
}

/// Calculates the mean heat in Moore neighbourhood of a cell at given location.
fn neighborhood_heat(map: &GrayImageRaw, x: usize, y: usize) -> u32 {
    let x: isize = x as isize;
    let y: isize = y as isize;

    (pixel_value(map, x - 1, y - 1, 0)
        + pixel_value(map, x, y - 1, 0)
        + pixel_value(map, x + 1, y - 1, 0)
        + pixel_value(map, x - 1, y, 0)
        + pixel_value(map, x + 1, y, 0)
        + pixel_value(map, x - 1, y + 1, 0)
        + pixel_value(map, x, y + 1, 0)
        + pixel_value(map, x + 1, y + 1, 0))
        / 8
}
