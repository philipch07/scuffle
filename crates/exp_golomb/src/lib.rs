use std::io;

use scuffle_bytes_util::{BitReader, BitWriter};

pub fn read_exp_golomb<R: io::Read>(reader: &mut BitReader<R>) -> io::Result<u64> {
    let mut leading_zeros = 0;
    while !reader.read_bit()? {
        leading_zeros += 1;
    }

    let mut result = 1;
    for _ in 0..leading_zeros {
        result <<= 1;
        result |= reader.read_bit()? as u64;
    }

    Ok(result - 1)
}

pub fn read_signed_exp_golomb<R: io::Read>(reader: &mut BitReader<R>) -> io::Result<i64> {
    let exp_glob = read_exp_golomb(reader)?;

    if exp_glob % 2 == 0 {
        Ok(-((exp_glob / 2) as i64))
    } else {
        Ok((exp_glob / 2) as i64 + 1)
    }
}

pub fn write_exp_golomb<W: io::Write>(writer: &mut BitWriter<W>, input: u64) -> io::Result<()> {
    let mut number = input + 1;
    let mut leading_zeros = 0;
    while number > 1 {
        number >>= 1;
        leading_zeros += 1;
    }

    for _ in 0..leading_zeros {
        writer.write_bit(false)?;
    }

    writer.write_bits(input + 1, leading_zeros + 1)?;

    Ok(())
}

pub fn write_signed_exp_golomb<W: io::Write>(writer: &mut BitWriter<W>, number: i64) -> io::Result<()> {
    let number = if number <= 0 {
        -number as u64 * 2
    } else {
        number as u64 * 2 - 1
    };

    write_exp_golomb(writer, number)
}

#[cfg(test)]
mod tests {
    use bytes::Buf;
    use scuffle_bytes_util::{BitReader, BitWriter};

    use crate::{read_exp_golomb, read_signed_exp_golomb, write_exp_golomb, write_signed_exp_golomb};

    pub fn get_remaining_bits(reader: &BitReader<std::io::Cursor<Vec<u8>>>) -> usize {
        let remaining = reader.get_ref().remaining();

        if reader.is_aligned() {
            remaining * 8
        } else {
            remaining * 8 + (8 - reader.bit_pos() as usize)
        }
    }

    #[test]
    fn test_exp_glob_decode() {
        let mut bit_writer = BitWriter::<Vec<u8>>::default();

        bit_writer.write_bits(0b1, 1).unwrap(); // 0
        bit_writer.write_bits(0b010, 3).unwrap(); // 1
        bit_writer.write_bits(0b011, 3).unwrap(); // 2
        bit_writer.write_bits(0b00100, 5).unwrap(); // 3
        bit_writer.write_bits(0b00101, 5).unwrap(); // 4
        bit_writer.write_bits(0b00110, 5).unwrap(); // 5
        bit_writer.write_bits(0b00111, 5).unwrap(); // 6

        let data = bit_writer.finish().unwrap();

        let mut bit_reader = BitReader::new(std::io::Cursor::new(data));

        let remaining_bits = get_remaining_bits(&bit_reader);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 0);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 1);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 1);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 4);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 2);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 7);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 3);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 12);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 4);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 17);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 5);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 22);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 6);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 27);
    }

    #[test]
    fn test_signed_exp_glob_decode() {
        let mut bit_writer = BitWriter::<Vec<u8>>::default();

        bit_writer.write_bits(0b1, 1).unwrap(); // 0
        bit_writer.write_bits(0b010, 3).unwrap(); // 1
        bit_writer.write_bits(0b011, 3).unwrap(); // -1
        bit_writer.write_bits(0b00100, 5).unwrap(); // 2
        bit_writer.write_bits(0b00101, 5).unwrap(); // -2
        bit_writer.write_bits(0b00110, 5).unwrap(); // 3
        bit_writer.write_bits(0b00111, 5).unwrap(); // -3

        let data = bit_writer.finish().unwrap();

        let mut bit_reader = BitReader::new(std::io::Cursor::new(data));

        let remaining_bits = get_remaining_bits(&bit_reader);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 0);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 1);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 1);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 4);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, -1);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 7);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 2);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 12);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, -2);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 17);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 3);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 22);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, -3);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 27);
    }

    #[test]
    fn test_exp_glob_encode() {
        let mut bit_writer = BitWriter::<Vec<u8>>::default();

        write_exp_golomb(&mut bit_writer, 0).unwrap();
        write_exp_golomb(&mut bit_writer, 1).unwrap();
        write_exp_golomb(&mut bit_writer, 2).unwrap();
        write_exp_golomb(&mut bit_writer, 3).unwrap();
        write_exp_golomb(&mut bit_writer, 4).unwrap();
        write_exp_golomb(&mut bit_writer, 5).unwrap();
        write_exp_golomb(&mut bit_writer, 6).unwrap();
        write_exp_golomb(&mut bit_writer, u64::MAX - 1).unwrap();

        let data = bit_writer.finish().unwrap();

        let mut bit_reader = BitReader::new(std::io::Cursor::new(data));

        let remaining_bits = get_remaining_bits(&bit_reader);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 0);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 1);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 1);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 4);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 2);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 7);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 3);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 12);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 4);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 17);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 5);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 22);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 6);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 27);

        let result = read_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, u64::MAX - 1);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 154);
    }

    #[test]
    fn test_signed_exp_glob_encode() {
        let mut bit_writer = BitWriter::<Vec<u8>>::default();

        write_signed_exp_golomb(&mut bit_writer, 0).unwrap();
        write_signed_exp_golomb(&mut bit_writer, 1).unwrap();
        write_signed_exp_golomb(&mut bit_writer, -1).unwrap();
        write_signed_exp_golomb(&mut bit_writer, 2).unwrap();
        write_signed_exp_golomb(&mut bit_writer, -2).unwrap();
        write_signed_exp_golomb(&mut bit_writer, 3).unwrap();
        write_signed_exp_golomb(&mut bit_writer, -3).unwrap();
        write_signed_exp_golomb(&mut bit_writer, i64::MAX).unwrap();

        let data = bit_writer.finish().unwrap();

        let mut bit_reader = BitReader::new(std::io::Cursor::new(data));

        let remaining_bits = get_remaining_bits(&bit_reader);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 0);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 1);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 1);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 4);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, -1);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 7);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 2);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 12);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, -2);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 17);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, 3);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 22);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, -3);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 27);

        let result = read_signed_exp_golomb(&mut bit_reader).unwrap();
        assert_eq!(result, i64::MAX);
        assert_eq!(get_remaining_bits(&bit_reader), remaining_bits - 154);
    }
}
