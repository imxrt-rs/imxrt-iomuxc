//! Ensure that the `prelude` re-exports remain backwards compatible.
//!
//! If these tests do not compile, consider the API broken.

#![allow(unused)]

mod iomuxc {
    #[cfg(feature = "imxrt1060")]
    pub use imxrt_iomuxc::imxrt1060::*;
    pub use imxrt_iomuxc::prelude::*;
}

/// Ensure that prelude modules are re-exported as expected
#[test]
fn use_prelude() {
    use iomuxc::{
        consts, flexpwm, gpio, lpi2c, lpspi, lpuart, Daisy, ErasedPad, Pad, WrongPadError,
    };
}

/// Ensure that the imxrt1060 modules are re-exported
#[cfg(feature = "imxrt1060")]
#[test]
fn use_imxrt1060() {
    use iomuxc::{
        gpio_ad_b0, gpio_ad_b1, gpio_b0, gpio_b1, gpio_emc, gpio_sd_b0, gpio_sd_b1, ErasedPads,
        Pads,
    };
}
