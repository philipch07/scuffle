use std::io;

/// A reader that reads individual bits from a stream
#[derive(Debug)]
#[must_use]
pub struct BitReader<T> {
    data: T,
    bit_pos: u8,
    current_byte: u8,
}

impl<T> BitReader<T> {
    /// Create a new BitReader from a reader
    pub const fn new(data: T) -> Self {
        Self {
            data,
            bit_pos: 0,
            current_byte: 0,
        }
    }
}

impl<T: io::Read> BitReader<T> {
    /// Reads a single bit
    pub fn read_bit(&mut self) -> io::Result<bool> {
        if self.is_aligned() {
            let mut buf = [0];
            self.data.read_exact(&mut buf)?;
            self.current_byte = buf[0];
        }

        let bit = (self.current_byte >> (7 - self.bit_pos)) & 1;

        self.bit_pos = (self.bit_pos + 1) % 8;

        Ok(bit == 1)
    }

    /// Reads multiple bits
    pub fn read_bits(&mut self, count: u8) -> io::Result<u64> {
        let mut bits = 0;
        for _ in 0..count {
            let bit = self.read_bit()?;
            bits <<= 1;
            bits |= bit as u64;
        }

        Ok(bits)
    }

    /// Aligns the reader to the next byte boundary
    pub fn align(&mut self) -> io::Result<()> {
        let amount_to_read = 8 - self.bit_pos;
        self.read_bits(amount_to_read as u8)?;
        Ok(())
    }
}

impl<T> BitReader<T> {
    /// Returns the underlying reader
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Returns a reference to the underlying reader
    pub const fn get_ref(&self) -> &T {
        &self.data
    }

    /// Returns the current bit position (0-7)
    pub const fn bit_pos(&self) -> u8 {
        self.bit_pos
    }

    /// Checks if the reader is aligned to the byte boundary
    pub const fn is_aligned(&self) -> bool {
        self.bit_pos == 0
    }
}

impl<T: io::Read> io::Read for BitReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.is_aligned() {
            return self.data.read(buf);
        }

        for byte in buf.iter_mut() {
            *byte = 0;
            for _ in 0..8 {
                let bit = self.read_bit()?;
                *byte <<= 1;
                *byte |= bit as u8;
            }
        }

        Ok(buf.len())
    }
}

impl<B: AsRef<[u8]>> BitReader<std::io::Cursor<B>> {
    /// Creates a new BitReader from a slice
    pub const fn new_from_slice(data: B) -> Self {
        Self::new(std::io::Cursor::new(data))
    }
}

impl<W: io::Seek + io::Read> BitReader<W> {
    /// Seeks a number of bits forward or backward
    pub fn seek_bits(&mut self, count: i64) -> io::Result<()> {
        if count == 0 {
            return Ok(());
        }

        let abs = count.abs();
        let bit_move = abs % 8;
        let byte_move = abs / 8;

        if count > 0 {
            self.data.seek(io::SeekFrom::Current(byte_move))?;
            self.read_bits(bit_move as u8)?;
        } else {
            let bit_pos = self.bit_pos as i64 - bit_move;
            let additional_byte_move = if bit_pos < 0 { 1 } else { 0 };
            self.data.seek(io::SeekFrom::Current(-byte_move - additional_byte_move))?;
            self.read_bits(bit_move.unsigned_abs() as u8)?;
        }

        Ok(())
    }
}

impl<T: io::Seek + io::Read> io::Seek for BitReader<T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        match pos {
            io::SeekFrom::Start(_) => {
                self.bit_pos = 0;
                self.data.seek(pos)
            }
            io::SeekFrom::Current(offset) => {
                self.seek_bits(offset * 8)?;
                Ok(self.data.stream_position()?)
            }
            io::SeekFrom::End(_) => {
                self.bit_pos = 0;
                self.data.seek(pos)
            }
        }
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use io::{Read, Seek};

    use super::*;

    #[test]
    fn test_bit_reader() {
        let binary = 0b10101010110011001111000101010101u32;

        let mut reader = BitReader::new_from_slice(binary.to_be_bytes());
        for i in 0..32 {
            assert_eq!(
                reader.read_bit().unwrap(),
                (binary & (1 << (31 - i))) != 0,
                "bit {} is not correct",
                i
            );
        }

        assert!(reader.read_bit().is_err(), "there shouldnt be any bits left");
    }

    #[test]
    fn test_bit_reader_read_bits() {
        let binary = 0b10101010110011001111000101010101u32;
        let mut reader = BitReader::new_from_slice(binary.to_be_bytes());
        let cases = [
            (3, 0b101),
            (4, 0b0101),
            (3, 0b011),
            (3, 0b001),
            (3, 0b100),
            (3, 0b111),
            (5, 0b10001),
            (1, 0b0),
            (7, 0b1010101),
        ];

        for (i, (count, expected)) in cases.into_iter().enumerate() {
            assert_eq!(
                reader.read_bits(count).ok(),
                Some(expected),
                "reading {} bits ({i}) are not correct",
                count
            );
        }

        assert!(reader.read_bit().is_err(), "there shouldnt be any bits left");
    }

    #[test]
    fn test_bit_reader_align() {
        let mut reader = BitReader::new_from_slice([0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000, 0b10000000]);

        for i in 0..6 {
            let pos = reader.data.stream_position().unwrap();
            assert_eq!(pos, i, "stream pos");
            assert_eq!(reader.bit_pos(), 0, "bit pos");
            assert!(reader.read_bit().unwrap(), "bit {} is not correct", i);
            reader.align().unwrap();
            let pos = reader.data.stream_position().unwrap();
            assert_eq!(pos, i + 1, "stream pos");
            assert_eq!(reader.bit_pos(), 0, "bit pos");
        }

        assert!(reader.read_bit().is_err(), "there shouldnt be any bits left");
    }

    #[test]
    fn test_bit_reader_io_read() {
        let binary = 0b10101010110011001111000101010101u32;
        let mut reader = BitReader::new_from_slice(binary.to_be_bytes());

        // Aligned read (calls the underlying read directly (very fast))
        let mut buf = [0; 1];
        reader.read(&mut buf).unwrap();
        assert_eq!(buf, [0b10101010]);

        // Unaligned read
        assert_eq!(reader.read_bits(1).unwrap(), 0b1);
        let mut buf = [0; 1];
        reader.read(&mut buf).unwrap();
        assert_eq!(buf, [0b10011001]);
    }

    #[test]
    fn test_bit_reader_seek() {
        let binary = 0b10101010110011001111000101010101u32;
        let mut reader = BitReader::new_from_slice(binary.to_be_bytes());

        reader.seek_bits(5).unwrap();
        assert_eq!(reader.data.stream_position().unwrap(), 1);
        assert_eq!(reader.bit_pos(), 5);
        assert_eq!(reader.read_bits(1).unwrap(), 0b0);
        assert_eq!(reader.bit_pos(), 6);

        reader.seek_bits(10).unwrap();
        assert_eq!(reader.data.stream_position().unwrap(), 2);
        assert_eq!(reader.bit_pos(), 0);
        assert_eq!(reader.read_bits(1).unwrap(), 0b1);
        assert_eq!(reader.bit_pos(), 1);
        assert_eq!(reader.data.stream_position().unwrap(), 3);

        reader.seek_bits(-8).unwrap();
        assert_eq!(reader.data.stream_position().unwrap(), 2);
        assert_eq!(reader.bit_pos(), 1);
        assert_eq!(reader.read_bits(1).unwrap(), 0b1);
        assert_eq!(reader.bit_pos(), 2);
        assert_eq!(reader.data.stream_position().unwrap(), 2);
    }

    #[test]
    fn test_bit_reader_io_seek() {
        let binary = 0b10101010110011001111000101010101u32;
        let mut reader = BitReader::new_from_slice(binary.to_be_bytes());
        reader.seek(io::SeekFrom::Start(1)).unwrap();
        assert_eq!(reader.bit_pos(), 0);
        assert_eq!(reader.data.stream_position().unwrap(), 1);
        assert_eq!(reader.read_bits(1).unwrap(), 0b1);
        assert_eq!(reader.bit_pos(), 1);
        assert_eq!(reader.data.stream_position().unwrap(), 2);

        reader.seek(io::SeekFrom::Current(1)).unwrap();
        assert_eq!(reader.bit_pos(), 1);
        assert_eq!(reader.data.stream_position().unwrap(), 3);
        assert_eq!(reader.read_bits(1).unwrap(), 0b1);
        assert_eq!(reader.bit_pos(), 2);
        assert_eq!(reader.data.stream_position().unwrap(), 3);

        reader.seek(io::SeekFrom::Current(-1)).unwrap();
        assert_eq!(reader.bit_pos(), 2);
        assert_eq!(reader.data.stream_position().unwrap(), 2);
        assert_eq!(reader.read_bits(1).unwrap(), 0b0);
        assert_eq!(reader.bit_pos(), 3);
        assert_eq!(reader.data.stream_position().unwrap(), 2);

        reader.seek(io::SeekFrom::End(-1)).unwrap();
        assert_eq!(reader.bit_pos(), 0);
        assert_eq!(reader.data.stream_position().unwrap(), 3);
        assert_eq!(reader.read_bits(1).unwrap(), 0b0);
        assert_eq!(reader.bit_pos(), 1);
        assert_eq!(reader.data.stream_position().unwrap(), 4);
    }
}
