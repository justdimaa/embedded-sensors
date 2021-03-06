#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum Register {
    POWER_DOWN = 0b0000_0000,
    POWER_ON = 0b0000_0001,
    RESET = 0b0000_0111,
    CONTINUOUSLY_H_RES_MODE = 0b0001_0000,
    CONTINUOUSLY_H_RES_MODE2 = 0b0001_0001,
    CONTINUOUSLY_L_RES_MODE = 0b0001_0011,
    ONE_TIME_H_RES_MODE = 0b0010_0000,
    ONE_TIME_H_RES_MODE2 = 0b0010_0001,
    ONE_TIME_L_RES_MODE = 0b0010_0011,
}
