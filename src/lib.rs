mod display_chunks;
mod display_shm;
mod util;

use std::io;

pub enum Mode {
    Chunks,
    SharedMemory,
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
        use display_shm::*;

        util::reset_cursor()?;
        match self.mode {
            Mode::Chunks => display_chunks(buffer, self.size, bits_per_pixel)?,
            Mode::SharedMemory => display_shm(buffer, self.size, bits_per_pixel)?,
        }

        Ok(())
    }
}
