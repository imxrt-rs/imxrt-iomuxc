//! Pads for the i.MX RT 1060 processor family
//!
//! The module exports all of the i.MX RT 1060 processor's pads. Pads that can support peripheral
//! functions are tagged with `imxrt-iomuxc` traits.

mod adc;
mod flexcan;
mod flexio;
mod flexpwm;
mod lpi2c;
mod lpspi;
mod lpuart;
mod sai;
mod usdhc;

mod pads;
pub use pads::*;
