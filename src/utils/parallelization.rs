use super::*;
use bounding::hierarchy::BoundingHierarchy;
use camera::Camera;
use io::PngTile;
use rayon::prelude::*;

/// Attempts to estimate the number of cores available for parallelism, defaulting to 1 should it not be
/// able to estimate said value.
pub fn estimate_cores() -> usize {
    match std::thread::available_parallelism() {
        Ok(num) => num.get(),
        Err(_) => 1,
    }
}

fn separate_lines(
    image_dimensions: Dimensions,
    division_step: usize,
) -> Vec<(Dimensions, TileCorner)> {
    let mut total_work = image_dimensions.1;
    let mut y_offset = 0;

    let mut results = Vec::new();
    while total_work > division_step {
        let tile_dimensions = Dimensions(image_dimensions.0, division_step);
        let corner = TileCorner(0, y_offset);

        y_offset += division_step;
        total_work -= division_step;

        results.push((tile_dimensions, corner))
    }

    if total_work != 0 {
        let tile_dimensions = Dimensions(image_dimensions.0, total_work);
        let corner = TileCorner(0, y_offset);
        results.push((tile_dimensions, corner));
    }

    results
}

/// Joins canvases vertically based on the y_offset of each tile.
/// Assumes that each band was created with the division strategy of determine_work
pub fn glue_canvases(mut canvases: Vec<(usize, PngTile)>) -> PngTile {
    canvases.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    canvases
        .into_iter()
        .map(|(_, canvas)| canvas)
        .reduce(|acc, elem| acc.join_vertical(elem))
        .expect("There should not be an empty list of canvases")
}

/// Renders the thread in a single thread. Useful mostly for debugging.
pub fn render_single_threaded(
    image_dimensions: Dimensions,
    camera: Camera,
    geometry: &BoundingHierarchy,
) -> PngTile {
    render(image_dimensions, camera, geometry, image_dimensions.1)
}

/// Renders the scene by dividing it so that each worker has division_step lines to render,
/// with the possible exception of the last one, who has the remainder.
///
/// Will error out if the division_step is 0
pub fn render(
    image_dimensions: Dimensions,
    camera: Camera,
    geometry: &BoundingHierarchy,
    division_step: usize,
) -> PngTile {
    assert_ne! { division_step, 0 }

    let canvases: Vec<_> = separate_lines(image_dimensions, division_step)
        .par_iter()
        .enumerate()
        .map(|(id, (dimensions, offset))| {
            let canvas = camera.render(id, *dimensions, *offset, &geometry.clone());
            (id, canvas)
        })
        .collect();

    glue_canvases(canvases)
}
