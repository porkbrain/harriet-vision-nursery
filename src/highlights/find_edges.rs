use super::image::{
  Luma,
  GrayImage,
  ImageLuma8,
  ImageBuffer,
  DynamicImage,
};

/// Replaces all pixels that are darker/brighter than these thresholds, giving
/// the edge detection extra space to highlight edges.
const DARKEST_GREYSCALE_VALUE: u8 = 5;
const BRIGHTEST_GREYSCALE_VALUE: u8 = 250;

/// How strongly should edges be favored in edge detection algorithm. The larger
/// the value, the more dense the resulting image becomes.
const EDGE_COEF: f32 = 7.5_f32;

const FILTERS: [[f32; 9]; 5] = [
  // Highlights horizontal edges.
  [
    EDGE_COEF, EDGE_COEF, EDGE_COEF,
    1_f32, 1_f32, 1_f32,
    -EDGE_COEF, -EDGE_COEF, -EDGE_COEF,
  ],
  // Highlights vertical edges.
  [
    EDGE_COEF, 1_f32, -EDGE_COEF,
    EDGE_COEF, 1_f32, -EDGE_COEF,
    EDGE_COEF, 1_f32, -EDGE_COEF,
  ],
  // Highlights corners.
  [
    -EDGE_COEF, -EDGE_COEF / 2_f32, 1_f32,
    -EDGE_COEF / 2_f32, 1_f32, EDGE_COEF / 2_f32,
    1_f32, EDGE_COEF / 2_f32, EDGE_COEF,
  ],
  // Highlights diagonals.
  [
    1_f32, EDGE_COEF, EDGE_COEF,
    -EDGE_COEF, 1_f32, EDGE_COEF,
    -EDGE_COEF, -EDGE_COEF, 1_f32,
  ],
  // Highlights diagonals.
  [
    EDGE_COEF, EDGE_COEF, 1_f32,
    EDGE_COEF, 1_f32, -EDGE_COEF,
    1_f32, -EDGE_COEF, -EDGE_COEF,
  ],
];

/// Finds edges in given grayscale picture by using two 3x3 matrixes. First one
/// detects horizontal edges, the second one vertical.
pub fn find_edges(
  image: &DynamicImage
) -> GrayImage {
  let image = smooth_out_polarized_pixels(&image);

  let mut detectors: Vec<GrayImage> = Vec::new();
  for matrix in FILTERS.iter() {
    detectors.push(image.filter3x3(matrix).to_luma());
  }

  let (width, height) = detectors[0].dimensions();
  let mut edge_detector = ImageBuffer::new(width, height);

  // Merges the two edge highlighters together into a single image.
  for (x, y, pixel) in edge_detector.enumerate_pixels_mut() {
    // Finds min and max values for a pixel in each detector.
    let (max, min) = detectors.iter().fold((1, 1), |(max, min), detector| {
      // Finds an edge value at given pixel.
      let edge_value = detector.get_pixel(x, y).data[0];

      (edge_value.max(max), edge_value.min(min))
    });

    // If larger from both values equals max value (255 for white) or the lower
    // equals the min value (0 for black), this pixel has been recognized as
    // clear edge and will be coloured (therefore we do Luma([0]) for black).
    // Otherwise the pixel value is white as the edge in this pixel was not
    // that prevalent. We have to check for both max and min values (0 and 255)
    // because the 3x3 kernels work in one direction. Should we only check for
    // black, we would end up with edges where the darker colour was on
    // top or right to the brighter one.
    *pixel = if max == 255 || min == 0 {
      Luma([0])
    } else {
      Luma([255])
    };
  }

  edge_detector
}

/// Removes pixels that are too dark or bright so that the edge detection works
/// better. This is a hacky solution that works mostly for bright images.
fn smooth_out_polarized_pixels(image: &DynamicImage) -> DynamicImage {
  // Copies the image with all colours converted to Luma.
  let mut image_gray: GrayImage = image.grayscale().to_luma();

  for pixel in image_gray.pixels_mut() {
    if pixel.data[0] < DARKEST_GREYSCALE_VALUE {
      *pixel = Luma([DARKEST_GREYSCALE_VALUE]);
    } else if pixel.data[0] > BRIGHTEST_GREYSCALE_VALUE {
      *pixel = Luma([BRIGHTEST_GREYSCALE_VALUE]);
    }
  }

  ImageLuma8(image_gray)
}
