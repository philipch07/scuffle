use std::borrow::Cow;
use std::io;

use byteorder::{BigEndian, WriteBytesExt};

use super::define::Amf0Marker;
use super::{Amf0Value, Amf0WriteError};

/// AMF0 encoder.
///
/// Allows for encoding an AMF0 to some writer.
pub struct Amf0Encoder;

impl Amf0Encoder {
    /// Encode a generic AMF0 value
    pub fn encode(writer: &mut impl io::Write, value: &Amf0Value) -> Result<(), Amf0WriteError> {
        match value {
            Amf0Value::Boolean(val) => Self::encode_bool(writer, *val),
            Amf0Value::Null => Self::encode_null(writer),
            Amf0Value::Number(val) => Self::encode_number(writer, *val),
            Amf0Value::String(val) => Self::encode_string(writer, val),
            Amf0Value::Object(val) => Self::encode_object(writer, val),
            _ => Err(Amf0WriteError::UnsupportedType(value.marker())),
        }
    }

    fn object_eof(writer: &mut impl io::Write) -> Result<(), Amf0WriteError> {
        writer.write_u24::<BigEndian>(Amf0Marker::ObjectEnd as u32)?;
        Ok(())
    }

    /// Encode an AMF0 number
    pub fn encode_number(writer: &mut impl io::Write, value: f64) -> Result<(), Amf0WriteError> {
        writer.write_u8(Amf0Marker::Number as u8)?;
        writer.write_f64::<BigEndian>(value)?;
        Ok(())
    }

    /// Encode an AMF0 boolean
    pub fn encode_bool(writer: &mut impl io::Write, value: bool) -> Result<(), Amf0WriteError> {
        writer.write_u8(Amf0Marker::Boolean as u8)?;
        writer.write_u8(value as u8)?;
        Ok(())
    }

    /// Encode an AMF0 string
    pub fn encode_string(writer: &mut impl io::Write, value: &str) -> Result<(), Amf0WriteError> {
        if value.len() > (u16::MAX as usize) {
            return Err(Amf0WriteError::NormalStringTooLong);
        }

        writer.write_u8(Amf0Marker::String as u8)?;
        writer.write_u16::<BigEndian>(value.len() as u16)?;
        writer.write_all(value.as_bytes())?;
        Ok(())
    }

    /// Encode an AMF0 null
    pub fn encode_null(writer: &mut impl io::Write) -> Result<(), Amf0WriteError> {
        writer.write_u8(Amf0Marker::Null as u8)?;
        Ok(())
    }

    /// Encode an AMF0 object
    pub fn encode_object(
        writer: &mut impl io::Write,
        properties: &[(Cow<'_, str>, Amf0Value<'_>)],
    ) -> Result<(), Amf0WriteError> {
        writer.write_u8(Amf0Marker::Object as u8)?;
        for (key, value) in properties {
            writer.write_u16::<BigEndian>(key.len() as u16)?;
            writer.write_all(key.as_bytes())?;
            Self::encode(writer, value)?;
        }

        Self::object_eof(writer)?;
        Ok(())
    }
}

#[cfg(test)]
#[cfg_attr(all(test, coverage_nightly), coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn test_write_number() {
        let mut amf0_number = vec![0x00];
        amf0_number.extend_from_slice(&772.161_f64.to_be_bytes());

        let mut vec = Vec::<u8>::new();

        Amf0Encoder::encode_number(&mut vec, 772.161).unwrap();

        assert_eq!(vec, amf0_number);
    }

    #[test]
    fn test_write_boolean() {
        let amf0_boolean = vec![0x01, 0x01];

        let mut vec = Vec::<u8>::new();

        Amf0Encoder::encode_bool(&mut vec, true).unwrap();

        assert_eq!(vec, amf0_boolean);
    }

    #[test]
    fn test_write_string() {
        let mut amf0_string = vec![0x02, 0x00, 0x0b];
        amf0_string.extend_from_slice(b"Hello World");

        let mut vec = Vec::<u8>::new();

        Amf0Encoder::encode_string(&mut vec, "Hello World").unwrap();

        assert_eq!(vec, amf0_string);
    }

    #[test]
    fn test_write_null() {
        let amf0_null = vec![0x05];

        let mut vec = Vec::<u8>::new();

        Amf0Encoder::encode_null(&mut vec).unwrap();

        assert_eq!(vec, amf0_null);
    }

    #[test]
    fn test_write_object() {
        let mut amf0_object = vec![0x03, 0x00, 0x04];
        amf0_object.extend_from_slice(b"test");
        amf0_object.extend_from_slice(&[0x05]);
        amf0_object.extend_from_slice(&[0x00, 0x00, 0x09]);

        let mut vec = Vec::<u8>::new();

        Amf0Encoder::encode_object(&mut vec, &[("test".into(), Amf0Value::Null)]).unwrap();

        assert_eq!(vec, amf0_object);
    }
}
