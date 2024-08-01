pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height];
        Framebuffer { width, height, buffer }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn set_current_color(&mut self, _color: u32) {
        // Implement this if you want to use a current color
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = 0xFFFFFF; // Pintar de blanco
        }
    }
}
