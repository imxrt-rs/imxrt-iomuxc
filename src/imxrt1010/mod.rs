//! Pads for the i.MX RT 1010 processor family
//!
//! The module exports all of the i.MX RT 1010 processor's pads. Pads that can support peripheral
//! functions are tagged with `imxrt-iomuxc` traits.

mod flexpwm;
mod lpi2c;
mod lpspi;
mod lpuart;

mod pads;
pub use pads::*;

mod ccm {
    pub use crate::ccm::{Function, Observable, Pin};

    impl Pin for super::pads::gpio_sd::GPIO_SD_02 {
        const ALT: u32 = 3;
        type Function = Observable<1>;
    }
    impl Pin for super::pads::gpio_sd::GPIO_SD_01 {
        const ALT: u32 = 3;
        type Function = Observable<2>;
    }
}
