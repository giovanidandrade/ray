use engine::{io::PngTile, Color, Dimensions, Float};

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;
    let mut canvas = PngTile::new(Dimensions(image_width, image_height));

    // Render
    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as Float) / (image_width as Float);
            let g = (j as Float) / (image_height as Float);

            let color = Color::new(r, g, 0.0);
            canvas.set(i, j, color);
        }
    }

    canvas.export("picture.png");
}
