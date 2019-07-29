use image::{self, ImageBuffer, Rgb};

// borrowed from https://github.com/lord/tinyrenderer/blob/master/src/img.rs
pub struct Img {
    pub buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub w: u32,
    pub h: u32,
}

impl Img {
    pub fn new(w: u32, h: u32) -> Img {
        let mut buf = ImageBuffer::new(w, h);
        buf.put_pixel(0, 0, Rgb([0 as u8; 3]));
        Img {
            buf: buf,
            w:   w,
            h:   h,
        }
    }

    // puts a pixel using (x, y) indexing rather than (r, c)
    pub fn put(&mut self, x: u32, y: u32, color: (u8, u8, u8)) {
        if x < self.w && y < self.h {
            self.buf.put_pixel(x, self.h-y-1, Rgb([color.0, color.1, color.2]));
        }
    }

    pub fn save(self) {
        self.buf.save("../images/output.png").unwrap();
    }

}

