use super::*;
use camera::{Camera, Ray};
use io::PngTile;
use std::thread::JoinHandle;

/// Given a camera, a world and a render function, uses as many cores available to render the scene
pub fn render_parallel(
    image_dimensions: Dimensions,
    camera: Camera,
    world: &World,
    render_fn: fn(&Ray, &World) -> Color,
) -> PngTile {
    let mut handles = Vec::new();
    for (id, (dimensions, offset)) in threads::determine_work(image_dimensions)
        .into_iter()
        .enumerate()
    {
        let world = world.clone();
        let handle = std::thread::spawn(move || {
            let canvas = camera
                .render::<fn(&Ray, &World) -> Color>(id, dimensions, offset, &world, render_fn);

            (id, canvas)
        });

        handles.push(handle);
    }

    threads::join_canvases(handles)
}

/// Attempts to estimate the number of cores available for parallelism, defaulting to 1 should it not be
/// able to estimate said value.
pub fn estimate_cores() -> usize {
    match std::thread::available_parallelism() {
        Ok(num) => num.get(),
        Err(_) => 1,
    }
}

/// Divies up work vertically into roughly equal bands based on the estimated number of cores (difference of 1 at most).
pub fn determine_work(image_dims: Dimensions) -> Vec<(Dimensions, TileCorner)> {
    determine_work_with_cores(image_dims, estimate_cores())
}

/// Divies up work vertically into roughly equal bands based on the number of cores (difference of 1 at most).
pub fn determine_work_with_cores(
    image_dims: Dimensions,
    num_cores: usize,
) -> Vec<(Dimensions, TileCorner)> {
    let base_work = image_dims.1 / num_cores;
    let mut remainder = image_dims.1 % num_cores;
    let mut y_offset = 0;

    let mut results = Vec::new();
    for _ in 0..num_cores {
        let extra_work = if remainder != 0 {
            remainder -= 1;
            1
        } else {
            0
        };

        let new_dims = Dimensions(image_dims.0, base_work + extra_work);
        let corner = TileCorner(0, y_offset);

        y_offset += base_work + extra_work;

        results.push((new_dims, corner))
    }

    results
}

/// Joins canvases vertically based on the y_offset of each tile.
/// Assumes that each band was created with the division strategy of determine_work
pub fn join_canvases(handles: Vec<JoinHandle<(usize, PngTile)>>) -> PngTile {
    let mut canvases: Vec<_> = handles
        .into_iter()
        .map(|handle| handle.join().expect("Thread couldn't be joined"))
        .collect();

    canvases.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    canvases
        .into_iter()
        .map(|(_, canvas)| canvas)
        .reduce(|acc, elem| acc.join_vertical(elem))
        .unwrap()
}
