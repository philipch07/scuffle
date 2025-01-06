use std::io;

/// A writer that allows you to write bits to a stream, this writer will buffer
/// the bits and flush the buffer to the underlying writer when the buffer is
/// full or the writer is flushed. By default the buffer size is 64 bytes.
#[derive(Debug)]
#[must_use]
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
    /// Writes a single bit to the stream
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

    /// Writes a number of bits to the stream (the most significant bit is
    /// written first)
    pub fn write_bits(&mut self, bits: u64, count: usize) -> io::Result<()> {
        for i in 0..count {
            let bit = (bits >> (count - i - 1)) & 1 == 1;
            self.write_bit(bit)?;
        }

        Ok(())
    }

    /// Flushes the buffer and returns the underlying writer
    /// This will also align the writer to the byte boundary
    pub fn finish(mut self) -> io::Result<W> {
        self.align()?;
        self.flush_buffer()?;
        Ok(self.writer)
    }

    /// Aligns the writer to the byte boundary
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

impl<W, const BUFFER_SIZE: usize> BitWriter<W, BUFFER_SIZE> {
    const _ASSERT_BUFFER_SIZE: () = {
        assert!(BUFFER_SIZE > 0, "BUFFER_SIZE must be greater than 0");
    };

    /// Creates a new BitWriter from a writer
    pub const fn new(writer: W) -> Self {
        let _: () = Self::_ASSERT_BUFFER_SIZE;

        Self {
            bit_pos: 0,
            buffer: [0; BUFFER_SIZE],
            writer,
        }
    }

    /// Returns the current bit position (0-7)
    pub const fn bit_pos(&self) -> u8 {
        (self.bit_pos % 8) as u8
    }

    /// Checks if the writer is aligned to the byte boundary
    pub const fn is_aligned(&self) -> bool {
        self.bit_pos % 8 == 0
    }

    /// Returns a reference to the underlying writer
    pub const fn get_ref(&self) -> &W {
        &self.writer
    }
}

impl<W: io::Write> io::Write for BitWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.is_aligned() {
            self.flush_buffer()?;
            return self.writer.write(buf);
        }

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

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use io::Write;

    use super::*;

    #[test]
    fn test_bit_writer() {
        let mut bit_writer = BitWriter::<Vec<u8>>::default();

        bit_writer.write_bits(0b11111111, 8).unwrap();
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());

        bit_writer.write_bits(0b0000, 4).unwrap();
        assert_eq!(bit_writer.bit_pos(), 4);
        assert!(!bit_writer.is_aligned());
        bit_writer.align().unwrap();
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());

        bit_writer.write_bits(0b1010, 4).unwrap();
        assert_eq!(bit_writer.bit_pos(), 4);
        assert!(!bit_writer.is_aligned());

        bit_writer.write_bits(0b101010101010, 12).unwrap();
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());

        bit_writer.write_bit(true).unwrap();
        assert_eq!(bit_writer.bit_pos(), 1);
        assert!(!bit_writer.is_aligned());

        assert_eq!(
            bit_writer.finish().unwrap(),
            vec![0b11111111, 0b00000000, 0b10101010, 0b10101010, 0b10000000]
        );
    }

    #[test]
    fn test_flush_buffer() {
        let mut bit_writer = BitWriter::<Vec<u8>>::default();

        bit_writer.write_bits(0b11111111, 8).unwrap();
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());
        assert_eq!(bit_writer.get_ref().len(), 0, "underlying writer should be empty");
        bit_writer.flush_buffer().unwrap();
        assert_eq!(bit_writer.get_ref(), &[0b11111111], "underlying writer should have one byte");

        bit_writer.write_bits(0b0000, 4).unwrap();
        assert_eq!(bit_writer.bit_pos(), 4);
        assert!(!bit_writer.is_aligned());
        assert_eq!(bit_writer.get_ref().len(), 1, "underlying writer should have one byte");
        bit_writer.flush_buffer().unwrap();
        assert_eq!(bit_writer.get_ref(), &[0b11111111], "underlying writer should have one bytes");

        bit_writer.write_bits(0b1010, 4).unwrap();
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());
        assert_eq!(bit_writer.get_ref().len(), 1, "underlying writer should have one byte");
        bit_writer.flush_buffer().unwrap();
        assert_eq!(
            bit_writer.get_ref(),
            &[0b11111111, 0b00001010],
            "underlying writer should have two bytes"
        );
    }

    #[test]
    fn test_io_write() {
        let mut inner = Vec::new();
        let mut bit_writer = BitWriter::<_, 64>::new(&mut inner);

        bit_writer.write_bits(0b11111111, 8).unwrap();
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());
        // We should have buffered the write
        assert_eq!(bit_writer.get_ref().as_slice(), &[]);

        bit_writer.write(&[1, 2, 3]).unwrap();
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());
        // since we did an io::Write on an aligned bit_writer
        // we should have flushed the buffer and then written directly to the underlying
        // writer
        assert_eq!(bit_writer.get_ref().as_slice(), &[255, 1, 2, 3]);

        bit_writer.write_bit(true).unwrap();

        bit_writer.write_bits(0b1010, 4).unwrap();

        bit_writer.write(&[0b11111111, 0b00000000, 0b11111111, 0b00000000]).unwrap();

        // Since the writer was not aligned we should have buffered the writes
        assert_eq!(bit_writer.get_ref().as_slice(), &[255, 1, 2, 3]);

        bit_writer.finish().unwrap();

        assert_eq!(
            inner,
            vec![255, 1, 2, 3, 0b11010111, 0b11111000, 0b00000111, 0b11111000, 0b00000000]
        );
    }

    #[test]
    fn test_flush() {
        let mut inner = Vec::new();
        let mut bit_writer = BitWriter::<_, 64>::new(&mut inner);

        bit_writer.write_bits(0b10100000, 8).unwrap();

        bit_writer.flush().unwrap();

        assert_eq!(bit_writer.get_ref().as_slice(), &[0b10100000]);
        assert_eq!(bit_writer.bit_pos(), 0);
        assert!(bit_writer.is_aligned());
    }
}
