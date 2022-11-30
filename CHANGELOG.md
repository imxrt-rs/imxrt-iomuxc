# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.2.0] 2022-11-30

### Changed

- **BREAKING** Update to Rust 2021 edition.

- **BREAKING** Remove typenum dependency, and use const generics. Requires
  at least Rust 1.51.

- **BREAKING** the GPIO `Pin` trait is now generic over the GPIO module.
  A const generic replaces the associated type `Module`.

- **BREAKING** rename feature flags and module: `"imxrt106x" => "imxrt1060"`

  For rational on this change, see
  [here](https://github.com/imxrt-rs/imxrt-rs/pull/91).

- **BREAKING** add the prefix 'GPIO_' to nearly all pad types and pad modules.

  - `AD_B1_23` => `GPIO_AD_B1_23`
  - `ad_b1` => `gpio_ad_b1`

- **BREAKING** favor Rust's API naming guidelines for select traits, marker types:
  - `ADC` => `Adc`
  - `SCL` => `Scl`
  - etc.

- **BREAKING** in the ADC module, we remove the `ADC` trait. Users should
  replace usages of `adc::ADC` with constants that indicate ADC1, ADC2, etc.

  `gpio::Pin` is no longer a trait bound for the `adc::Pin` trait. Users who
  relied on this guarantee should explicitly require the bound.

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

- Deprecated the old pull / keeper configuration API. See the 0.1.4 release for
  the new API.

### Added

- i.MX RT 1010 support with the `"imxrt1010"` feature:
  - LPI2C
  - LPSPI
  - LPUART
  - FlexPWM

- Add uSDHC pin traits.
- Add select uSDHC pins for the i.MX RT 1060.

- Basic i.MX RT 1170 support with the `"imxrt1170"` feature. Includes minimal
  pad implementations for

  - LPI2C
  - LPSPI
  - LPUART
  - FlexPWM

  that are sufficient for evaluating the 1170EVK.

- Add CCM clock output pin trait with 1010, 1170 implementations.

## [0.1.5] - 2022-01-01

### Added

- Include additional SPI pins for the i.MX RT 1060.

## [0.1.4] - 2021-11-14

- Add `unsafe` static functions on every pad to set alternate, change SION, and set configurations.
  These functions do not require ownership of the pad object.

- Unify the pullup, pulldown, and keeper configurations into one enum,
  `PullKeeper`. This lets you more simply express pin configurations:

  ```rust
  // Before
  configure(
      &mut pad,
      Config::zero()
          .set_pull_keep(PullKeep::Enabled)
          .set_pull_keep_select(PullKeepSelect::Pull)
          .set_pullupdown(PullUpDown::Pullup100k),
  );

  // After
  configure(
      &mut pad,
      Config::zero().set_pull_keeper(Some(PullKeeper::Pullup100k))
  );
  ```

  This approach will replace the existing API that uses separate methods and
  enums.

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

[Unreleased]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/imxrt-rs/imxrt-iomuxc/releases/tag/v0.1.0
