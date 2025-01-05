use std::io;

pub struct BitReader<T> {
    data: T,
    bit_pos: usize,
    current_byte: u8,
}

impl<T> BitReader<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            bit_pos: 0,
            current_byte: 0,
        }
    }
}

impl<T: io::Read> BitReader<T> {
    pub fn read_bit(&mut self) -> io::Result<bool> {
        if self.is_aligned() {
            let mut buf = [0];
            self.data.read_exact(&mut buf)?;
            self.current_byte = buf[0];
        }

        let bit = (self.current_byte >> (7 - self.bit_pos)) & 1;

        self.bit_pos += 1;
        self.bit_pos %= 8;

        Ok(bit == 1)
    }

    pub fn read_bits(&mut self, count: u8) -> io::Result<u64> {
        let mut bits = 0;
        for _ in 0..count {
            let bit = self.read_bit()?;
            bits <<= 1;
            bits |= bit as u64;
        }

        Ok(bits)
    }

    pub fn align(&mut self) -> io::Result<()> {
        let amount_to_read = 8 - self.bit_pos;
        self.read_bits(amount_to_read as u8)?;
        Ok(())
    }
}

impl<T> BitReader<T> {
    pub fn into_inner(self) -> T {
        self.data
    }

    pub fn get_ref(&self) -> &T {
        &self.data
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn get_bit_pos(&self) -> usize {
        self.bit_pos
    }

    pub fn is_aligned(&self) -> bool {
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
    pub fn new_from_slice(data: B) -> Self {
        Self::new(std::io::Cursor::new(data))
    }
}

impl<W: io::Seek + io::Read> BitReader<W> {
    pub fn seek_bits(&mut self, count: i64) -> io::Result<u64> {
        for _ in 0..count {
            self.read_bit()?;
        }

        Ok(
            (self.data.stream_position()? * 8 + self.bit_pos as u64).saturating_sub(if self.is_aligned() {
                0
            } else {
                8
            }),
        )
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
                let pos = self.seek_bits(offset * 8)?;
                Ok(pos / 8)
            }
            io::SeekFrom::End(_) => {
                self.bit_pos = 0;
                self.data.seek(pos)
            }
        }
    }
}
