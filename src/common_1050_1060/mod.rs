//! Definitions shared between the 1050 and 1060 MCUs.
//!
//! The 1060 pads are an extension of the 1050 pads. This module contains the
//! definitions common to both MCUs.

mod adc;
mod flexcan;
mod flexio;
mod flexpwm;
mod lpi2c;
mod lpspi;
mod lpuart;
mod sai;
mod usdhc;

use pads::*;
pub(crate) mod pads;
