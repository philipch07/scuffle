use std::io;

#[derive(Clone, Debug)]
pub struct BitWriter<W, const BUFFER_SIZE: usize = 64> {
    bit_pos: usize,
    writer: W,
    buffer: [u8; BUFFER_SIZE],
}

impl<W: Default> Default for BitWriter<W> {
    fn default() -> Self {
        Self {
            bit_pos: 0,
            writer: W::default(),
            buffer: [0; 64],
        }
    }
}

impl<W: io::Write, const BUFFER_SIZE: usize> BitWriter<W, BUFFER_SIZE> {
    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        let byte_index = self.bit_pos / 8;
        let bit_index = self.bit_pos % 8;

        if bit {
            self.buffer[byte_index] |= 1 << (7 - bit_index);
        } else {
            self.buffer[byte_index] &= !(1 << (7 - bit_index));
        }

        self.bit_pos += 1;

        if self.bit_pos == BUFFER_SIZE * 8 {
            self.flush_buffer()?;
        }

        Ok(())
    }

    pub fn write_bits(&mut self, bits: u64, count: usize) -> io::Result<()> {
        for i in 0..count {
            let bit = (bits >> (count - i - 1)) & 1 == 1;
            self.write_bit(bit)?;
        }

        Ok(())
    }

    pub fn finish(mut self) -> io::Result<W> {
        self.align()?;
        self.flush_buffer()?;
        Ok(self.writer)
    }

    pub fn align(&mut self) -> io::Result<()> {
        if !self.is_aligned() {
            self.write_bits(0, 8 - (self.bit_pos % 8))?;
        }

        Ok(())
    }

    fn flush_buffer(&mut self) -> io::Result<()> {
        let len = self.bit_pos / 8;
        self.writer.write_all(&self.buffer[..len])?;
        let bit_pos = self.bit_pos % 8;
        self.bit_pos = bit_pos;
        if bit_pos > 0 {
            let last_byte = self.buffer[len];
            self.buffer[0] = last_byte;
        }

        Ok(())
    }
}

impl<W: io::Write, const BUFFER_SIZE: usize> BitWriter<W, BUFFER_SIZE> {
    pub fn new(writer: W) -> Self {
        Self {
            bit_pos: 0,
            buffer: [0; BUFFER_SIZE],
            writer,
        }
    }

    pub fn get_bit_pos(&self) -> usize {
        self.bit_pos
    }

    pub fn is_aligned(&self) -> bool {
        self.bit_pos % 8 == 0
    }
}

impl<W: io::Write> io::Write for BitWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        for byte in buf {
            self.write_bits(*byte as u64, 8)?;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_buffer()?;
        self.writer.flush()
    }
}
