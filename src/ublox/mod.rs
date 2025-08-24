use core::marker::PhantomData;

pub use nmea0183 as parser;

pub mod result;

use self::result::{Error, Result};

pub struct Ublox<SERIAL>
where
    SERIAL: embedded_io::Read + embedded_io::Write,
{
    parser: parser::Parser,
    _serial: PhantomData<SERIAL>,
}

impl<SERIAL> Ublox<SERIAL>
where
    SERIAL: embedded_io::Read + embedded_io::Write,
{
    pub fn new() -> Self {
        Self {
            parser: parser::Parser::new(),
            _serial: PhantomData::default(),
        }
    }

    pub fn read(&mut self, serial: &mut SERIAL) -> Result<parser::ParseResult> {
        let mut read_word = || {
            let mut buf: [u8; 1] = [0; 1];
            match serial.read_exact(&mut buf) {
                Ok(()) => nb::Result::Ok(buf[0]),
                Err(e) => nb::Result::Err(e.into()),
            }
        };
        match nb::block!(read_word()) {
            Ok(v) => match self.parser.parse_from_byte(v) {
                Some(v) => match v {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Error::ParserError(e)),
                },
                None => Err(Error::NotEnoughData),
            },
            Err(_) => Err(Error::SerialError),
        }
    }
}
