# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Changed

- **BREAKING** rename feature flags and module: `"imxrt106x" => "imxrt1060"`

  For rational on this change, see
  [here](https://github.com/imxrt-rs/imxrt-rs/pull/91).

- **BREAKING** favor Rust's API naming guidelines for select traits, marker types:
  - `ADC` => `Adc`
  - `SCL` => `Scl`
  - etc.

- **BREAKING** in the ADC module, we remove the `ADC` trait. Users should
  replace usages of `adc::ADC` with `consts::Unsigned`. The `Adc1` and `Adc2`
  types are now aliases for `U1` and `U2`.

  `adc::Pin::INPUT` is now an associated `u32` constant, not a type. Cast the
  `u32` as needed for your implementation. See the before and after below for
  migration tips.

  ```rust
  // Before
  let channel: u16 = <P as Pin<ADC1>>::Input::U16;

  // After
  let channel: u16 = <P as Pin<ADC1>>::INPUT as u16;
  ```

- **BREAKING** rename modules to match i.MX RT peripherals. Specific renames include

  - `i2c` => `lpi2c`
  - `pwm` => `flexpwm`
  - `spi` => `lpspi`
  - `uart` => `lpuart`

- Add `unsafe` static functions on every pad to set alternate, change SION, and set configurations.
  These functions do not require ownership of the pad object.

### Added

- i.MX RT 1010 support with the `"imxrt1010"` feature:
  - I2C
  - SPI
  - UART

## [0.1.3] - 2021-04-24

### Added

- Support for SAI pins

## [0.1.2] - 2020-11-23

### Added

- Support for ADC pins

## [0.1.1] - 2020-09-10

### Added

- Export a `prelude` module to help others re-export APIs that are not specific
  to a crate.

## [0.1.0] - 2020-08-28

First release

- Configuration API
- Preliminary support for i.MX RT 1060 chips:
  - I2C
  - GPIO
  - PWM
  - SPI
  - UART

[Unreleased]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/imxrt-rs/imxrt-iomuxc/releases/tag/v0.1.0
