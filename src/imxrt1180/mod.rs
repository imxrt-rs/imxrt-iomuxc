//! Pads for the i.MX RT 1180 processor family.
//!
//! The module exports all of the i.MX RT 1180 processor's pads. Pads that can support
//! peripheral functions are tagged with `imxrt-iomuxc` traits.

mod pads;
pub use pads::*;

mod lpuart;
