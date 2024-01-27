#[allow(dead_code)]
pub struct Wangdenticon {
    pub gridsize: u32,
}

#[allow(dead_code)]
impl Wangdenticon {
    #[allow(dead_code)]
    pub fn new(gridsize: u32) -> Self {
        Self { gridsize }
    }

    const MIDDLE_TILES: &'static [u8] = &[0, 1, 4, 5, 10, 11, 14, 15];
    const OPPOSITE_MAP: &'static [u8] = &[0, 1, 8, 9, 4, 5, 12, 13, 2, 3, 10, 11, 6, 7, 14, 15];

    fn render_tile(
        &self,
        tile: u8,
        img_buffer: &mut image::RgbImage,
        grid_idx: u32,
        fgcolor: &[u8; 3],
        bgcolor: &[u8; 3],
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
                img_buffer.put_pixel(col + j, row + i, image::Rgb(*bgcolor))
            }
        }

        if north == 1 {
            for j in 0..3 {
                img_buffer.put_pixel(col + j, row, image::Rgb(*fgcolor))
            }
        }

        if east == 2 {
            for i in 0..3 {
                img_buffer.put_pixel(col + 2, row + i, image::Rgb(*fgcolor))
            }
        }

        if south == 4 {
            for j in 0..3 {
                img_buffer.put_pixel(col + j, row + 2, image::Rgb(*fgcolor))
            }
        }

        if west == 8 {
            for i in 0..3 {
                img_buffer.put_pixel(col, row + i, image::Rgb(*fgcolor))
            }
        }
    }

    fn cell_index(&self, grid_idx: u32) -> (u32, u32) {
        let row = grid_idx / self.gridsize;
        let col = grid_idx % self.gridsize;
        (row * 3, col * 3)
    }

    pub fn generate(
        &self,
        hex_list: &[u8; 16],
        fgcolor: &[u8; 3],
        bgcolor: &[u8; 3],
    ) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let middle_tile = Self::MIDDLE_TILES[hex_list[15] as usize % Self::MIDDLE_TILES.len()];
        let width = 3 * self.gridsize;
        let height = width;
        let mut img_buffer = image::RgbImage::new(width, height);
        let xub = (self.gridsize >> 1) + (self.gridsize & 1);
        for y in 0..self.gridsize {
            for x in 0..xub {
                let left_idx = (y * self.gridsize) + x;
                let right_idx = y * self.gridsize + self.gridsize - 1 - x;
                if left_idx != right_idx {
                    let tile = hex_list[((y * xub + x) % 15) as usize];
                    self.render_tile(tile, &mut img_buffer, left_idx, fgcolor, bgcolor);
                    self.render_tile(
                        Self::OPPOSITE_MAP[(tile as usize) % Self::OPPOSITE_MAP.len()],
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
        fgcolor: &[u8; 3],
        bgcolor: &[u8; 3],
        size: u32,
        out_fmt: image::ImageOutputFormat,
    ) -> Vec<u8> {
        let img_buffer = self.generate(hex_list, fgcolor, bgcolor);
        let mut buf = vec![];
        image::DynamicImage::ImageRgb8(img_buffer)
            .resize(size, size, image::imageops::Nearest)
            .write_to(&mut buf, out_fmt)
            .expect("Writing image as png failed!");
        buf
    }

    pub fn generate_as_png(
        &self,
        hex_list: &[u8; 16],
        fgcolor: &[u8; 3],
        bgcolor: &[u8; 3],
        size: u32,
    ) -> Vec<u8> {
        self.generate_as(
            hex_list,
            fgcolor,
            bgcolor,
            size,
            image::ImageOutputFormat::Png,
        )
    }

    pub fn generate_as_jpeg(
        &self,
        hex_list: &[u8; 16],
        fgcolor: &[u8; 3],
        bgcolor: &[u8; 3],
        size: u32,
        quality: u8,
    ) -> Vec<u8> {
        self.generate_as(
            hex_list,
            fgcolor,
            bgcolor,
            size,
            image::ImageOutputFormat::Jpeg(quality),
        )
    }
}
