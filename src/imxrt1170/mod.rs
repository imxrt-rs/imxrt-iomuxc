//! Pads for the i.MX RT 1170 processor family.
//!
//! The module exports all of the i.MX RT 1170 processor's pads. Pads that can support
//! peripheral functions are tagged with `imxrt-iomuxc` traits.

mod pads;
pub use pads::*;

mod flexpwm;
mod lpi2c;
mod lpspi;
mod lpuart;

mod ccm {
    pub use crate::ccm::{Observable, Pin};

    impl Pin for super::pads::gpio_emc_b1::GPIO_EMC_B1_40 {
        const ALT: u32 = 9;
        type Function = Observable<1>;
    }
    impl Pin for super::pads::gpio_emc_b1::GPIO_EMC_B1_41 {
        const ALT: u32 = 9;
        type Function = Observable<2>;
    }
}
