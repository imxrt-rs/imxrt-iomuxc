# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Add all LPUART pads for RT1180.
- Add select LPUART5 pins for the 1060 MCUs.

## [0.3.0] 2024-12-02

- **BREAKING** Add support for additional chip select pins for `lpspi4`.

- **BREAKING** Remove the deprecated pull / keeper configuration API.

- **BREAKING** Change the type of the `lpspi::Pin::DAISY` and `lpi2c::Pin::DAISY`
  associated constant to an `Option<Daisy>`.

- Add i.MX RT 1180 support with the `"imxrt1180"` feature.

## [0.2.9] 2025-02-16

- Add select LPUART5 pins for the 1060 MCUs.

## [0.2.8] 2024-07-18

- Add remaining FlexPWM pins for the 1060 MCUs.

## [0.2.7] 2024-06-10

- Fix many 1170 GPIO implementations, most which incorrectly specified their
  offset. Introduce a few missing GPIO implementations.

## [0.2.6] 2024-01-27

- Enable an open drain when preparing I2C pins on 10xx MCUs.

## [0.2.5] 2023-12-23

- Add SAI pin implementations for the 1020.

## [0.2.4] 2023-11-17

- Add SAI pin implementations for the 1010.

## [0.2.3] 2023-09-09

- Add FlexIO pin implementations for 1010 and 1020 pads.

## [0.2.2] 2023-07-03

- Add optional `DAISY` associated constant to `flexpwm::Pin`.

- Add FlexIO pin definitions and implementations for 1060 pads.

- Ensure that `adc::prepare()` configures an alternate value for ADC inputs.

## [0.2.1] 2023-02-23

- Add FlexCAN pin trait with implementations for select 1060 pads.

- Add i.MX RT 1020 support with the `"imxrt1020"` feature. Includes select
  pad implementations for

  - LPI2C
  - LPSPI
  - LPUART
  - CCM

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

[Unreleased]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.8...v0.3.0
[0.2.9]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.8...v0.2.9
[0.2.8]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.7...v0.2.8
[0.2.7]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.6...v0.2.7
[0.2.6]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/imxrt-rs/imxrt-iomuxc/releases/tag/v0.1.0
