use std::io;

use bytes::Bytes;

/// A helper trait to implement zero copy reads on a `Cursor<Bytes>` type.
pub trait BytesCursor {
    /// Returns the remaining bytes in the cursor.
    fn remaining(&self) -> usize;

    /// Extracts the remaining bytes from the cursor returning.
    ///
    /// This does not do a copy of the bytes, and is O(1) time.
    ///
    /// This is the same as `BytesCursor::extract_bytes(self.remaining())`.
    fn extract_remaining(&mut self) -> Bytes;

    /// Extracts a bytes from the cursor.
    ///
    /// This does not do a copy of the bytes, and is O(1) time.
    /// Returns an error if the size is greater than the remaining bytes.
    fn extract_bytes(&mut self, size: usize) -> io::Result<Bytes>;
}

impl BytesCursor for io::Cursor<Bytes> {
    fn remaining(&self) -> usize {
        self.get_ref().len() - self.position() as usize
    }

    fn extract_remaining(&mut self) -> Bytes {
        self.extract_bytes(self.remaining())
            .expect("somehow we read past the end of the file")
    }

    fn extract_bytes(&mut self, size: usize) -> io::Result<Bytes> {
        let position = self.position() as usize;
        if position + size > self.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "not enough bytes"));
        }

        let slice = self.get_ref().slice(position..position + size);
        self.set_position((position + size) as u64);

        Ok(slice)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_cursor() {
        let mut cursor = io::Cursor::new(Bytes::from_static(&[1, 2, 3, 4, 5]));
        let remaining = cursor.extract_remaining();
        assert_eq!(remaining, Bytes::from_static(&[1, 2, 3, 4, 5]));
    }
}
