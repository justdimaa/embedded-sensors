#![allow(warnings)]
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

use core::{convert::TryFrom, marker::PhantomData};

use embedded_hal::{
    digital::v2::OutputPin,
    prelude::{_embedded_hal_serial_Read, _embedded_hal_serial_Write},
    serial,
};

use self::{
    config::ParameterSettings,
    op::{OperationCode, OperationMode},
    result::Error,
};

pub mod config;
pub mod op;
pub mod result;

#[derive(Debug)]
pub struct E32<SERIAL> {
    air_data_rate: u8,
    uart_data_rate: u16,
    parity_bit: u8,
    speed: u8,
    _serial: PhantomData<SERIAL>,
}

impl<SERIAL, SerialError> E32<SERIAL>
where
    SERIAL: serial::Read<u8, Error = SerialError> + serial::Write<u8, Error = SerialError>,
{
    pub fn new() -> Self {
        Self {
            air_data_rate: 0,
            uart_data_rate: 0,
            parity_bit: 0,
            speed: 0,
            _serial: PhantomData::default(),
        }
    }

    pub fn read_cfg<M0: OutputPin, M1: OutputPin>(
        &mut self,
        serial: &mut SERIAL,
        m0_pin: &mut M0,
        m1_pin: &mut M1,
    ) -> Result<(), Error<SerialError>> {
        self.set_mode(OperationMode::Sleep, m0_pin, m1_pin)?;
        self.write_op(serial, OperationCode::ReadCfg)?;

        Ok(())
    }

    pub fn read_cfg_callback(
        &mut self,
        serial: &mut SERIAL,
    ) -> Result<ParameterSettings, Error<SerialError>> {
        let mut buf = [0; 6];
        let mut i = 0;

        while i != buf.len() {
            match nb::block!(serial.read()) {
                Ok(v) => {
                    buf[i] = v;
                    i += 1;
                }
                Err(e) => return Err(Error::SerialError(e)),
            };
        }

        match ParameterSettings::try_from(buf) {
            Ok(v) => Ok(v),
            Err(e) => Err(Error::ParseConfigurationError(e)),
        }
    }

    pub fn write_cfg<M0: OutputPin, M1: OutputPin>(
        &mut self,
        _serial: &mut SERIAL,
        _cfg: ParameterSettings,
        m0_pin: &mut M0,
        m1_pin: &mut M1,
    ) -> Result<(), Error<SerialError>> {
        self.set_mode(OperationMode::Normal, m0_pin, m1_pin)?;

        Ok(())
    }

    fn set_mode<M0: OutputPin, M1: OutputPin>(
        &self,
        op_mode: OperationMode,
        m0_pin: &mut M0,
        m1_pin: &mut M1,
    ) -> Result<(), Error<SerialError>> {
        if self.uart_data_rate != 9600 {
            return Err(Error::InvalidBaudRate(self.uart_data_rate));
        }

        match op_mode {
            OperationMode::Normal => {
                m0_pin.set_low().ok();
                m1_pin.set_low().ok();
            }
            OperationMode::WakeUp => {
                m0_pin.set_high().ok();
                m1_pin.set_low().ok();
            }
            OperationMode::PowerSaving => {
                m0_pin.set_low().ok();
                m1_pin.set_high().ok();
            }
            OperationMode::Sleep => {
                m0_pin.set_high().ok();
                m1_pin.set_high().ok();
            }
        }

        Ok(())
    }

    fn write_op(
        &self,
        serial: &mut SERIAL,
        op_code: OperationCode,
    ) -> Result<(), Error<SerialError>> {
        let op_code = op_code as u8;

        for _ in 0..3 {
            match nb::block!(serial.write(op_code)) {
                Ok(()) => {}
                Err(e) => return Err(Error::SerialError(e)),
            }
        }

        Ok(())
    }

    pub fn read(&mut self, serial: &mut SERIAL) -> Result<u8, Error<SerialError>> {
        match nb::block!(serial.read()) {
            Ok(v) => Ok(v),
            Err(e) => return Err(Error::SerialError(e)),
        }
    }

    pub fn write<N>(
        &mut self,
        serial: &mut SERIAL,
        data: &[u8],
        addr: u16,
        channel: u8,
    ) -> Result<(), Error<SerialError>>
    where
        N: heapless::ArrayLength<u8>,
    {
        let mut buf = heapless::Vec::<_, N>::new();

        buf[0..1].copy_from_slice(&addr.to_be_bytes());
        buf[2] = channel;

        match buf.extend_from_slice(data) {
            Ok(()) => {}
            Err(_) => return Err(Error::InvalidWriteSize(buf.capacity())),
        }

        for v in &buf {
            match nb::block!(serial.write(*v)) {
                Ok(()) => {}
                Err(e) => return Err(Error::SerialError(e)),
            }
        }

        Ok(())
    }
}
