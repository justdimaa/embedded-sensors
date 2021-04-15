#![no_std]

#[cfg(feature = "ak8963")]
pub mod ak8963;

#[cfg(feature = "mpu925x")]
pub mod mpu925x;

#[cfg(feature = "mpu6500")]
pub mod mpu6500;

#[cfg(feature = "ublox")]
pub mod ublox;
