use super::*;

#[derive(Debug, Clone)]
pub struct PngTile {
    dimensions: Dimensions,
    upper_left: TileCorner,
    buffer: Vec<u8>,
}

const COLOR_CHANNELS: usize = 3;

impl PngTile {
    /// Creates a tile with the upper left corner as the origin
    pub fn new(dimensions: Dimensions) -> Self {
        Self::with_offset(dimensions, TileCorner::default())
    }

    /// Creates a tile with a specific upper left corner
    pub fn with_offset(dimensions: Dimensions, offset: TileCorner) -> Self {
        let Dimensions(width, height) = dimensions;

        Self {
            dimensions,
            upper_left: offset,
            buffer: vec![0; width * height * COLOR_CHANNELS],
        }
    }

    /// Convenience function that does the offset math to return the correct index
    /// assuming the tile is laid flat
    fn index(&self, x: usize, y: usize) -> usize {
        let Dimensions(width, _) = self.dimensions;
        let TileCorner(x0, y0) = self.upper_left;

        ((x - x0) + width * (y - y0)) * COLOR_CHANNELS
    }

    /// Given a color in the [0, 1] RGB space, does the appropriate conversions and
    /// writes it to the tile.
    ///
    /// Will panic if x or y are smaller than the tile offsets in a debug build.
    pub fn set(&mut self, x: usize, y: usize, value: Color) {
        let TileCorner(x0, y0) = self.upper_left;
        debug_assert! { x >= x0 && y >= y0 };

        let index = self.index(x, y);

        self.buffer[index..index + 3].copy_from_slice(&value.to_gamma_corrected_bytes());
    }

    /// Glues two tiles together vertically, such that self is on top.
    /// The offset is the minimum of both values.
    pub fn join_vertical(self, other: Self) -> Self {
        let new_dimensions = Dimensions(self.dimensions.0, self.dimensions.1 + other.dimensions.1);
        let new_offset = TileCorner(
            self.upper_left.0.min(other.upper_left.0),
            self.upper_left.1.min(other.upper_left.1),
        );

        let mut buffer = self.buffer;
        buffer.extend(other.buffer);

        Self {
            dimensions: new_dimensions,
            upper_left: new_offset,
            buffer,
        }
    }

    /// Exports the tile to a png file.
    /// Code based on the png crate documentation
    pub fn export(&self, filename: &str) {
        let path = std::path::Path::new(filename);
        let file = std::fs::File::create(path).expect("File could not be created");
        let writer = &mut std::io::BufWriter::new(file);

        let Dimensions(width, height) = self.dimensions;
        let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));

        encoder.set_source_chromaticities(png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        ));

        encoder
            .write_header()
            .expect("Header could not be written to file")
            .write_image_data(&self.buffer)
            .expect("Data could not be written to file");
    }
}
