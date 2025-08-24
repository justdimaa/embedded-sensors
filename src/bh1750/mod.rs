use core::marker::PhantomData;

use embedded_hal::i2c;

use self::{
    config::{Config, MeasurementMode},
    result::Error,
};

pub mod config;
mod register;
pub mod result;

pub struct Bh1750<I2C>
where
    I2C: i2c::I2c,
{
    addr: u8,
    cfg: Config,
    light_level: f32,
    _i2c: PhantomData<I2C>,
}

impl<I2C> Bh1750<I2C>
where
    I2C: i2c::I2c,
{
    pub fn new(addr: u8, i2c: &mut I2C) -> Result<Self, Error<I2C::Error>> {
        Self::with_configuration(addr, i2c, Config::default())
    }

    pub fn with_configuration(
        addr: u8,
        i2c: &mut I2C,
        cfg: Config,
    ) -> Result<Self, Error<I2C::Error>> {
        let mut bh = Self {
            addr,
            cfg,
            light_level: 0.0,
            _i2c: PhantomData::default(),
        };

        bh.init(i2c)?;

        Ok(bh)
    }

    fn init(&mut self, i2c: &mut I2C) -> Result<(), Error<I2C::Error>> {
        self.set_measurement_mode(i2c, self.cfg.measurement_mode)?;
        self.set_measurement_time(i2c, self.cfg.measurement_time)?;
        Ok(())
    }

    pub fn measurement_mode(&self) -> MeasurementMode {
        self.cfg.measurement_mode
    }

    pub fn set_measurement_mode(
        &mut self,
        i2c: &mut I2C,
        measurement_mode: MeasurementMode,
    ) -> Result<(), Error<I2C::Error>> {
        Self::write_register(self.addr, i2c, measurement_mode as u8)?;
        self.cfg.measurement_mode = measurement_mode;
        Ok(())
    }

    pub fn measurement_time(&self) -> MeasurementMode {
        self.cfg.measurement_mode
    }

    pub fn set_measurement_time(
        &mut self,
        i2c: &mut I2C,
        measurement_time: u8,
    ) -> Result<(), Error<I2C::Error>> {
        if measurement_time < 31 || measurement_time > 254 {
            return Err(Error::InvalidMeasurementTime(measurement_time));
        }

        Self::write_register(
            self.addr,
            i2c,
            (0b01000 << 3) | (measurement_time as u8 >> 5),
        )?;
        Self::write_register(self.addr, i2c, (0b011 << 5) | (measurement_time & 0b11111))?;
        self.cfg.measurement_time = measurement_time;
        Ok(())
    }

    pub fn light_level(&self) -> f32 {
        self.light_level
    }

    pub fn read(&mut self, i2c: &mut I2C) -> Result<(), Error<I2C::Error>> {
        let mut buf = [0; 2];

        Self::read_register(self.addr, i2c, self.cfg.measurement_mode as u8, &mut buf)?;

        let light_level = u16::from_be_bytes(buf);
        self.light_level = match self.cfg.measurement_mode {
            MeasurementMode::ContinuouslyHighResolution
            | MeasurementMode::OneTimeHighResolution => {
                light_level as f32 / 1.2 * (69.0 / self.cfg.measurement_time as f32)
            }
            MeasurementMode::ContinuouslyHighResolution2
            | MeasurementMode::OneTimeHighResolution2 => {
                light_level as f32 / 1.2 * (69.0 / self.cfg.measurement_time as f32) / 2.0
            }
            MeasurementMode::ContinuouslyLowResolution | MeasurementMode::OneTimeLowResolution => {
                light_level as f32 / 1.2
            }
        };

        Ok(())
    }

    fn read_register(
        addr: u8,
        i2c: &mut I2C,
        reg: u8,
        buf: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        match i2c.write(addr, &[reg as u8]) {
            Ok(()) => {}
            Err(e) => return Err(Error::I2cError(e)),
        }

        match i2c.read(addr, buf) {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::I2cError(e)),
        }
    }

    fn write_register(addr: u8, i2c: &mut I2C, reg: u8) -> Result<(), Error<I2C::Error>> {
        match i2c.write(addr, &[reg]) {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::I2cError(e)),
        }
    }
}
