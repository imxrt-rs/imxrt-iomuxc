# imxrt-iomuxc

i.MX RT pad definitions and pin configuration. Use this library to

- configure a pad for a peripheral, and specify its electrical properties.
- manage pad objects as ownable resources.
- statically constrain pads to work with peripherals.

`imxrt-iomuxc` targets the latest stable Rust compiler.

## Getting started

If you're already using [`imxrt-hal`](https://github.com/imxrt-rs/imxrt-hal) as
you're hardware abstraction layer, there's nothing more to do. `imxrt-hal`
re-exports this API and ensures compatibility with your i.MX RT processor. See
the `imxrt-hal` documentation for more information on acquiring pads and
instantiating drivers.

If you're defining a new i.MX RT hardware driver, consider using `imxrt-iomuxc`
to constrain the pads that are compatible with your driver. Depend on the latest
`imxrt-iomuxc` package, and do not enable any package features. Then, implement
your APIs with the various `imxrt-iomuxc` pin traits. See the `imxrt-iomuxc` API
documentation for examples. For even more examples, study the `imxrt-hal`
package.

If you're defining a board support package, consider using `imxrt-iomuxc` to
rename the pads that are supported by your board. Depend on the latest
`imxrt-iomuxc` package, and enable the feature for your i.MX RT processor. Then,
expose type aliases and / or objects that rename i.MX RT processor pads
according to your board. If you're looking for an example of this pattern,
consult the [`teensy4-pins`](https://docs.rs/teensy4-pins) package.

## Supported chip

The table below shows the chips and their corresponding crate feature. Note that
chips are denoted by reference manuals and may support different variants. For
instance, the 1060 feature supports 1061 and 1062 chip variants.

| Chip | Feature       |
|------|---------------|
| 1010 | `"imxrt1010"` |
| 1020 | `"imxrt1020"` |
| 1060 | `"imxrt1060"` |
| 1170 | `"imxrt1170"` |
| 1180 | `"imxrt1180"` |

Read on if you're interested in adding support for another i.MX RT
microcontroller, or if you want to expand existing support.

## Contributing

We appreciate your contributions. After discussing the development basics, this
section describes how you can

- add support for new i.MX RT chips.
- implement pin traits on pads.
- define new pin traits for peripherals.

Generally, open an issue if you're interested in any type of contribution, or if
you find a defect.

### Development basics

Build the package with `cargo`. The first command only builds the pin traits and
configuration API; it does not include any chip support. The second command
includes 1060 pad definitions and pin implementations. The third command
includes all supported pad definitions and pin implementations for an ARM
target.

    cargo build
    cargo build --features=imxrt1060
    cargo build --all-features --target=thumbv7em-none-eabihf

All code must build for your host and for an embedded target. Notice how a
target like `thumbv7em-none-eabihf` is optional.

Run tests with

    cargo test --all-features

In particular, documementation tests may that the crate is built with the 1060
feature. Please follow this precedent when adding documentation examples.

Generate documentation with `cargo doc`. You can optionally enable (all) chip
features.

    cargo doc
    cargo doc --features=imxrt1060
    cargo doc --all-features

If you're interested in generating the docs.rs documentation, use a nightly
toolchain, and supply the `docsrs` configuration. This will include the
documentation tags that highlight build configurations.

    cargo +nightly rustdoc --all-features -- --cfg=docsrs

### Adding support for new chips

Use the [`iomuxc.py` script](./iomuxc.py) to generate all pads for a new i.MX RT
chip. The script extracts pad definitions from a system view description (SVD)
file. Consider using the SVDs maintained in [the `imxrt-ral`
repository](https://github.com/imxrt-rs/imxrt-ral).

Once you have an SVD and can generate the pads module, integrate the pads module
into the package. Use the existing 1010, 1060, and 1170 support as your guide.
As of now, the process roughly follows

1.  Conditionally include the module in `lib.rs`.
2.  Create a new directory for your chip, and add a `mod.rs` file. Include and
    re-export your script-generated pads module.
3.  Add a `Cargo.toml` feature for your chip.

By the end of this process, you have definitions for your i.MX RT chip's pads.
You should also have GPIO pin implementations. However, you do not have pin
implementations for other peripherals. Read on to learn about that contribution
process.

### Extending pin implementations for existing pads

This section assumes that a pin trait exists for your peripheral. If this
doesn't exist, read the next section for guidance.

When you implement a pin trait for a processor pad, you signal to users that
this pad supports a particular peripheral function. As of this writing, **we
write these implementations by hand** using the reference manual (RM) or SVD as
a source of truth. The general [imxrt-rs project
documentation](https://imxrt-rs.github.io/book) has tips for obtaining a RM.
Once you have an RM, the table in the "External Signals and Pin Multiplexing"
chapter should reveal the pads that support peripheral functions.

To extend pin implementations, either append to the existing pin module for your
chip, or create a new pin module for your chip. If your pin implementation
requires daisy registers, consider using [the `daisy.py` script](./daisy.py) to
extract them from a SVD file. (If you don't have SVD files, see the previous
section.) Again, use the existing 1010, 1060, and 1170 support as your guide.

We welcome any contribution to automate this process. See [the tracking
issue](https://github.com/imxrt-rs/imxrt-iomuxc/issues/14) for more information.

### Define new pin traits

When you define a new pin trait, you specify all of the information needed for a
pad to be configured for a particular peripheral function. For example, an
LPUART pin trait needs to be concerned with specifing

- the peripheral instance (LPUART1 or LPUART2?).
- the function (transmit, receive?).
- the alternate value for muxing.
- any required electrical settings (no pulls, slow slew, etc.).

The trait describes these parameters, and functions designed to those traits
prepare the pads for the chosen configuration.

The trait complexity may vary depending on the peripheral. If you're not sure
what a trait might look like, start by studying the pad multiplexing options,
then translate that into a trait. Consult the existing pin traits for different
design techniques.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
  <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
