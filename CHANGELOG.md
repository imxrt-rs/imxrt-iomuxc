# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Add uSDHC pin traits.
- Add select uSDHC pins for the i.MX RT 1060.
- Add FlexCAN pin traits.
- Add FlexCAN1 and 2 pins for the i.MX RT 1060.

## [0.1.5] - 2022-01-01

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

[Unreleased]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.5...v0.1
[0.1.5]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/imxrt-rs/imxrt-iomuxc/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/imxrt-rs/imxrt-iomuxc/releases/tag/v0.1.0
