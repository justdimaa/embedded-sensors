# embedded-sensors

Sensors in one place, with only one feature away.

# How to use

You will need to add the dependency to your `Cargo.toml`:
```toml
[dependencies]
embedded-sensors = "0.1.0"
```
Generally all supported sensors are enabled when you bind the crate into your project.
To disable all sensors you don't want to use, you have to set `default-features = false` and specify the sensors that you do want to use in the `features = ["..."]` field, for example:
```toml
[dependencies]
embedded-sensors = { version = "0.1.0", default-features = false, features = ["mpu925x", "ublox"] }
```

## Supported sensors

| Feature       | Sensors           |
| ------------- |:-----------------:|
| ak8963        | AK8963 |
| mpu6500       | MPU6500 |
| mpu925x       | MPU9250 |
| ublox         | NEO-6M |

Please submit an issue or a pull request for a sensor that you want to use, but is not listed in here.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
