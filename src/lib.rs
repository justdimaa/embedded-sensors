#![no_std]

#[cfg(feature = "ak8963")]
pub mod ak8963;

#[cfg(feature = "bh1750")]
pub mod bh1750;

#[cfg(feature = "e32")]
pub mod e32;

#[cfg(feature = "mpu925x")]
pub mod mpu925x;

#[cfg(feature = "mpu6500")]
pub mod mpu6500;

#[cfg(feature = "ublox")]
pub mod ublox;
