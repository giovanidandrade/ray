use engine::{
    io::{PngTile, TileCorner},
    Color, Dimensions, Float,
};

fn main() {
    let image_width = 2560;
    let image_height = 2560;

    let num_workers = match std::thread::available_parallelism() {
        Ok(num) => num.get(),
        Err(_) => 1,
    };

    let base_work = image_height / num_workers;
    let mut remainder = image_height % num_workers;
    let mut y_offset = 0;

    let mut handles = Vec::new();
    for id in 0..num_workers {
        let handle = std::thread::spawn(move || {
            let dims = Dimensions(image_width, base_work + remainder);
            let offset = TileCorner(0, y_offset);

            let mut canvas = PngTile::with_offset(dims, offset);

            for j in offset.1..(offset.1 + dims.1) {
                eprintln!("Thread {id}: {j} / {} scanlines", dims.1);

                for i in offset.0..(offset.0 + dims.0) {
                    let r = (i as Float) / (image_width as Float);
                    let g = (j as Float) / (image_height as Float);

                    let color = Color::new(r, g, 0.0);
                    canvas.set(i, j, color);
                }
            }

            (offset, canvas)
        });

        y_offset += base_work + remainder;
        remainder = 0;

        handles.push(handle);
    }

    let mut canvases = Vec::new();
    for handle in handles.into_iter() {
        let canvas = handle.join().expect("Thread couldn't be joined.");
        canvases.push(canvas);
    }

    canvases.sort_unstable_by(|a, b| {
        let (TileCorner(_, ay_offset), _) = a;
        let (TileCorner(_, by_offset), _) = b;

        ay_offset.cmp(by_offset)
    });

    canvases
        .into_iter()
        .map(|(_, canvas)| canvas)
        .reduce(|acc, elem| acc.join_vertical(elem))
        .unwrap()
        .export("picture.png");
}
