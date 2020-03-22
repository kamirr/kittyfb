mod display_chunks;
mod util;

use std::io;

pub enum Mode {
    Chunks,
}

pub struct Window {
    size: (usize, usize),
    mode: Mode,
}

impl Window {
    pub fn new(size: (usize, usize), mode: Mode) -> Self {
        Window { size, mode }
    }

    pub fn update_with_buffer(&mut self, buffer: &[u8], bits_per_pixel: usize) -> io::Result<()> {
        use display_chunks::*;

        util::reset_cursor()?;
        match self.mode {
            Mode::Chunks => display_chunks(buffer, self.size, bits_per_pixel)?,
        }

        Ok(())
    }
}
