#[allow(dead_code)]
pub struct Wangdenticon {
    pub gridsize: u8,
    pub invert: bool,
}

#[allow(dead_code)]
impl Wangdenticon {
    #[allow(dead_code)]
    pub fn new(gridsize: u8, invert: bool) -> Self {
        Self { gridsize, invert }
    }

    const MIDDLE_TILES: &'static [u8] = &[0, 1, 4, 5, 10, 11, 14, 15];
    const OPPOSITE_MAP: &'static [u8] = &[0, 1, 8, 9, 4, 5, 12, 13, 2, 3, 10, 11, 6, 7, 14, 15];

    fn render_tile(
        &self,
        tile: u8,
        img_buffer: &mut image::RgbImage,
        grid_idx: u16,
        fgcolor: [u8; 3],
        bgcolor: [u8; 3],
    ) {
        let (row, col) = self.cell_index(grid_idx);
        let m = tile % 16;

        let north = m & 1;
        let east = m & 2;
        let south = m & 4;
        let west = m & 8;

        // prefill with bgcolor
        for i in 0..3 {
            for j in 0..3 {
                img_buffer.put_pixel(col + j, row + i, image::Rgb(bgcolor))
            }
        }

        if north == 1 {
            for j in 0..3 {
                img_buffer.put_pixel(col + j, row, image::Rgb(fgcolor))
            }
        }

        if east == 2 {
            for i in 0..3 {
                img_buffer.put_pixel(col + 2, row + i, image::Rgb(fgcolor))
            }
        }

        if south == 4 {
            for j in 0..3 {
                img_buffer.put_pixel(col + j, row + 2, image::Rgb(fgcolor))
            }
        }

        if west == 8 {
            for i in 0..3 {
                img_buffer.put_pixel(col, row + i, image::Rgb(fgcolor))
            }
        }
    }

    fn cell_index(&self, grid_idx: u16) -> (u32, u32) {
        let row = grid_idx / (self.gridsize as u16);
        let col = grid_idx % (self.gridsize as u16);
        (row as u32 * 3, col as u32 * 3)
    }

    pub fn generate(&self, hex_list: &[u8; 16]) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let hash_color = [hex_list[0], hex_list[1], hex_list[2]];
        let black = [0, 0, 0];
        let middle_tile = Self::MIDDLE_TILES[hex_list[15] as usize % Self::MIDDLE_TILES.len()];
        let (fgcolor, bgcolor) = if self.invert {
            (black, hash_color)
        } else {
            (hash_color, black)
        };
        let width = 3 * (self.gridsize as u32);
        let height = width;
        let mut img_buffer = image::RgbImage::new(width, height);
        let xub = (self.gridsize >> 1) + (self.gridsize & 1);
        for y in 0..(self.gridsize as usize) {
            for x in 0..xub {
                let left_idx = (y as u16 * self.gridsize as u16) + x as u16;
                let right_idx =
                    y as u16 * self.gridsize as u16 + self.gridsize as u16 - 1 - x as u16;
                if left_idx != right_idx {
                    let tile = hex_list[(y as u16 * xub as u16 + x as u16) as usize % 15];
                    self.render_tile(tile, &mut img_buffer, left_idx, fgcolor, bgcolor);
                    self.render_tile(
                        Self::OPPOSITE_MAP[tile as usize % Self::OPPOSITE_MAP.len()],
                        &mut img_buffer,
                        right_idx,
                        fgcolor,
                        bgcolor,
                    );
                } else {
                    self.render_tile(middle_tile, &mut img_buffer, left_idx, fgcolor, bgcolor);
                }
            }
        }
        img_buffer
    }

    fn generate_as(
        &self,
        hex_list: &[u8; 16],
        size: usize,
        out_fmt: image::ImageOutputFormat,
    ) -> Vec<u8> {
        let img_buffer = self.generate(hex_list);
        let mut buf = vec![];
        image::DynamicImage::ImageRgb8(img_buffer)
            .resize(size as u32, size as u32, image::imageops::Nearest)
            .write_to(&mut buf, out_fmt)
            .expect("Writing image as png failed!");
        buf
    }

    pub fn generate_as_png(&self, hex_list: &[u8; 16], size: usize) -> Vec<u8> {
        self.generate_as(hex_list, size, image::ImageOutputFormat::Png)
    }

    pub fn generate_as_jpeg(&self, hex_list: &[u8; 16], size: usize, quality: u8) -> Vec<u8> {
        self.generate_as(hex_list, size, image::ImageOutputFormat::Jpeg(quality))
    }
}
