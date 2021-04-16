#[derive(Debug, Clone)]
pub struct Config {
    pub(crate) measurement_mode: MeasurementMode,
    pub(crate) measurement_time: u8,
}

impl Config {
    pub fn measurement_mode(mut self, measurement_mode: MeasurementMode) -> Self {
        self.measurement_mode = measurement_mode;
        self
    }

    pub fn measurement_time(mut self, measurement_time: u8) -> Self {
        self.measurement_time = measurement_time;
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            measurement_mode: MeasurementMode::default(),
            measurement_time: 69,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MeasurementMode {
    ContinuouslyHighResolution = 0b0001_0000,
    ContinuouslyHighResolution2 = 0b0001_0001,
    ContinuouslyLowResolution = 0b0001_0011,
    OneTimeHighResolution = 0b0010_0000,
    OneTimeHighResolution2 = 0b0010_0001,
    OneTimeLowResolution = 0b0010_0011,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        Self::ContinuouslyHighResolution
    }
}
