//! Pads for the 1050.

pub use crate::common_1050_1060::pads::*;

/// All of the pads.
pub struct Pads {
    pub gpio_emc: gpio_emc::Pads,
    pub gpio_ad_b0: gpio_ad_b0::Pads,
    pub gpio_ad_b1: gpio_ad_b1::Pads,
    pub gpio_b0: gpio_b0::Pads,
    pub gpio_b1: gpio_b1::Pads,
    pub gpio_sd_b0: gpio_sd_b0::Pads,
    pub gpio_sd_b1: gpio_sd_b1::Pads,
}

impl Pads {
    /// Take all pads from this group
    ///
    /// # Safety
    ///
    /// You may safely call this once to acquire all of the pads.
    /// Subsequent calls may return pads that are mutably aliased
    /// elsewhere. Consider calling new() at the start of your program.
    #[inline]
    pub const unsafe fn new() -> Self {
        Self {
            gpio_emc: gpio_emc::Pads::new(),
            gpio_ad_b0: gpio_ad_b0::Pads::new(),
            gpio_ad_b1: gpio_ad_b1::Pads::new(),
            gpio_b0: gpio_b0::Pads::new(),
            gpio_b1: gpio_b1::Pads::new(),
            gpio_sd_b0: gpio_sd_b0::Pads::new(),
            gpio_sd_b1: gpio_sd_b1::Pads::new(),
        }
    }

    /// Erase all of the pads
    ///
    /// The return type is an array, where the index indicates the
    /// pad offset from the start of the group. For example, AD_B0_03
    /// would be referenced as erased_pads\[3\].
    ///
    /// See `ErasedPads` for more information.
    #[inline]
    pub const fn erase(self) -> ErasedPads {
        ErasedPads {
            gpio_emc: self.gpio_emc.erase(),
            gpio_ad_b0: self.gpio_ad_b0.erase(),
            gpio_ad_b1: self.gpio_ad_b1.erase(),
            gpio_b0: self.gpio_b0.erase(),
            gpio_b1: self.gpio_b1.erase(),
            gpio_sd_b0: self.gpio_sd_b0.erase(),
            gpio_sd_b1: self.gpio_sd_b1.erase(),
        }
    }
}

/// All erased pads.
pub struct ErasedPads {
    pub gpio_emc: gpio_emc::ErasedPads,
    pub gpio_ad_b0: gpio_ad_b0::ErasedPads,
    pub gpio_ad_b1: gpio_ad_b1::ErasedPads,
    pub gpio_b0: gpio_b0::ErasedPads,
    pub gpio_b1: gpio_b1::ErasedPads,
    pub gpio_sd_b0: gpio_sd_b0::ErasedPads,
    pub gpio_sd_b1: gpio_sd_b1::ErasedPads,
}