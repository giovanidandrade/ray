use std::thread::JoinHandle;

use super::*;
use io::{PngTile, TileCorner};

pub fn estimate_cores() -> usize {
    match std::thread::available_parallelism() {
        Ok(num) => num.get(),
        Err(_) => 1,
    }
}

/// Divies up work vertically into roughly equal bands based on the number of cores (difference of 1 at most).
pub fn determine_work(image_dims: Dimensions, num_cores: usize) -> Vec<(Dimensions, TileCorner)> {
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
