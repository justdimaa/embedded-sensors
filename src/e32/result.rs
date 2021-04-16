use core::fmt;

#[derive(Debug)]
pub enum Error<SerialError> {
    InvalidBaudRate(u16),
    ParseConfigurationError(ParseConfigurationError),
    SerialError(SerialError),
    InvalidWriteSize(usize),
}

impl<SerialError> fmt::Display for Error<SerialError> 
where
    SerialError: fmt::Debug,{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            InvalidBaudRate(v) => write!(
                f,
                "Could not configure module because of an invalid baud rate. Value: {}.", v
            ),
            ParseConfigurationError(e) => fmt::Display::fmt(e, f),
            SerialError(e) => fmt::Debug::fmt(e, f),
            InvalidWriteSize(v) => write!(f, "To transmit data, the buffer capacity needs to be at least 4 bytes long. Current capacity: {}.", v)
        }
    }
}

#[derive(Debug)]
pub enum ParseConfigurationError {
    CouldNotParse,
    InvalidHead(u8),
    InvalidUartParity(u8),
    InvalidUartBaudRate(u8),
    InvalidAirDataRate(u8),
    InvalidTransmission(u8),
    InvalidWirelessWakeUpTime(u8),
    InvalidIoDriveMode(u8),
    InvalidTxPower(u8),
}

impl fmt::Display for ParseConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ParseConfigurationError::*;

        match *self {
            CouldNotParse => write!(f, "Could not parse configuration."),
            InvalidHead(v) => write!(f, "Invalid head {}", v),
            InvalidUartParity(v) => write!(f, "Invalid uart parity {:#010b}", v),
            InvalidUartBaudRate(v) => write!(f, "Invalid uart baud rate {:#010b}", v),
            InvalidAirDataRate(v) => write!(f, "Invalid air data rate {:#010b}", v),
            InvalidTransmission(v) => write!(f, "Invalid transmission {:#010b}", v),
            InvalidWirelessWakeUpTime(v) => write!(f, "Invalid wireless wake up time {:#010b}", v),
            InvalidIoDriveMode(v) => write!(f, "Invalid io drive mode {:#010b}", v),
            InvalidTxPower(v) => write!(f, "Invalid tx power {:#010b}", v),
        }
    }
}
