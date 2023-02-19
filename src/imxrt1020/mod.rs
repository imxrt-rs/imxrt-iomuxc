//! Pads for the i.MX RT 1020 processor family
//!
//! The module exports all of the i.MX RT 1020 processor's pads. Pads that can support peripheral
//! functions are tagged with `imxrt-iomuxc` traits.

mod lpi2c;
mod lpspi;
mod lpuart;

mod pads;
pub use pads::*;

mod ccm {
    pub use crate::ccm::{Function, Observable, Pin};

    impl Pin for super::pads::gpio_sd_b1::GPIO_SD_B1_02 {
        const ALT: u32 = 6;
        type Function = Observable<1>;
    }
    impl Pin for super::pads::gpio_sd_b1::GPIO_SD_B1_03 {
        const ALT: u32 = 6;
        type Function = Observable<2>;
    }
}
