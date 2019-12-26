use super::helpers::pixel_value;
use super::point::Point;
use super::visual_object::VisualObject;

type PointMap = Vec<Vec<bool>>;

pub const MAX_CELLS: u32 = 40;

pub fn extract_highlights(image: PointMap, reference: Point, objects: &mut Vec<VisualObject>) {
    for mut highlight in find_highlights_in_map(image, reference) {
        let size = highlight.size();

        if size.is_none() {
            continue;
        }

        let (lower, higher) = size.unwrap();

        if lower.x == higher.x || lower.y == higher.y {
            continue;
        }

        if higher.x - lower.x < MAX_CELLS && higher.y - lower.y < MAX_CELLS {
            objects.push(highlight);
            continue;
        }

        match highlight.peeled_map() {
            None => continue,
            Some(map) => extract_highlights(map, highlight.reference + lower, objects),
        };
    }
}

/// Finds objects within given image heatmap. Uses flood fill algorithm which,
/// after finding any highlighted unvisited point within the image, selects all
/// highlighted other points in the neighbourhood. This happens recursively for
/// each highlighted unvisited point.
fn find_highlights_in_map(mut image: PointMap, reference: Point) -> Vec<VisualObject> {
    // Currently iterated point in the image.
    let mut current_point: Point = Point::new(0, 0);
    // Instantiates the return vector.
    let mut objects: Vec<VisualObject> = Vec::new();

    // Should the image be empty, return empty vector.
    if image.len() == 0 || image[0].len() == 0 {
        return objects;
    }

    // Servers as image dimensions.
    let last_point: Point = Point::new(image[0].len() as u32 - 1, image.len() as u32 - 1);

    // As long as the currently iterated point is not the last one, run the cycle.
    while current_point != last_point {
        // If the value at currently iterated point is positive, flood fill the
        // object and remove it from the original map.
        if pixel_value(
            &image,
            current_point.x as isize,
            current_point.y as isize,
            false,
        ) {
            let mut object: VisualObject = VisualObject::new(reference);
            flood_fill(current_point, &mut object, &mut image);
            objects.push(object);
        }

        // Increments the row starting from 0 if current_point reached the end of
        // the line otherwise moves to the pixel to the right.
        if current_point.x == last_point.x {
            current_point.x = 0;
            current_point.y += 1;
        } else {
            current_point.x += 1;
        }
    }

    objects
}

/// Recursively finds a single object within given image. It calls this function
/// for every new highlighted point.
fn flood_fill(point: Point, object: &mut VisualObject, image: &mut PointMap) {
    // Adds currently iterated point to the object and set that point to no
    // highlighted.
    object.push(point);
    image[point.y as usize][point.x as usize] = false;

    // Iterates over the Moore neighbourhood of currently iterated point.
    for y in (point.y as isize - 1)..(point.y as isize + 2) {
        if y < 0 {
            continue;
        }

        for x in (point.x as isize - 1)..(point.x as isize + 2) {
            // If the Moore's point is not highlighted, skips.
            if x < 0 || !pixel_value(image, x, y, false) {
                continue;
            }

            // Visits the Moore's point.
            flood_fill(Point::new(x as u32, y as u32), object, image);
        }
    }
}
