//! Pads for the 1060.
//!
//! Most pads are shared with the 1050. This module defines pads that are specific to the 1060.

pub use crate::common_1050_1060::pads::*;

/// Pads with the prefix GPIO_SPI_B0.
pub mod gpio_spi_b0 {
    #![allow(non_camel_case_types)]

    const GPIO_SPI_B0_00_MUX_ADDR: u32 = 0x401f865c;
    const GPIO_SPI_B0_00_PAD_ADDR: u32 = 0x401f86b4;
    pub type GPIO_SPI_B0_00 = crate::Pad<GPIO_SPI_B0_00_MUX_ADDR, GPIO_SPI_B0_00_PAD_ADDR>;
    // GPIO_SPI_B0_00 does not have any GPIO alternates.

    const GPIO_SPI_B0_01_MUX_ADDR: u32 = 0x401f8660;
    const GPIO_SPI_B0_01_PAD_ADDR: u32 = 0x401f86b8;
    pub type GPIO_SPI_B0_01 = crate::Pad<GPIO_SPI_B0_01_MUX_ADDR, GPIO_SPI_B0_01_PAD_ADDR>;
    // GPIO_SPI_B0_01 does not have any GPIO alternates.

    const GPIO_SPI_B0_02_MUX_ADDR: u32 = 0x401f8664;
    const GPIO_SPI_B0_02_PAD_ADDR: u32 = 0x401f86bc;
    pub type GPIO_SPI_B0_02 = crate::Pad<GPIO_SPI_B0_02_MUX_ADDR, GPIO_SPI_B0_02_PAD_ADDR>;
    // GPIO_SPI_B0_02 does not have any GPIO alternates.

    const GPIO_SPI_B0_03_MUX_ADDR: u32 = 0x401f8668;
    const GPIO_SPI_B0_03_PAD_ADDR: u32 = 0x401f86c0;
    pub type GPIO_SPI_B0_03 = crate::Pad<GPIO_SPI_B0_03_MUX_ADDR, GPIO_SPI_B0_03_PAD_ADDR>;
    // GPIO_SPI_B0_03 does not have any GPIO alternates.

    const GPIO_SPI_B0_04_MUX_ADDR: u32 = 0x401f866c;
    const GPIO_SPI_B0_04_PAD_ADDR: u32 = 0x401f86c4;
    pub type GPIO_SPI_B0_04 = crate::Pad<GPIO_SPI_B0_04_MUX_ADDR, GPIO_SPI_B0_04_PAD_ADDR>;
    // GPIO_SPI_B0_04 does not have any GPIO alternates.

    const GPIO_SPI_B0_05_MUX_ADDR: u32 = 0x401f8670;
    const GPIO_SPI_B0_05_PAD_ADDR: u32 = 0x401f86c8;
    pub type GPIO_SPI_B0_05 = crate::Pad<GPIO_SPI_B0_05_MUX_ADDR, GPIO_SPI_B0_05_PAD_ADDR>;
    // GPIO_SPI_B0_05 does not have any GPIO alternates.

    const GPIO_SPI_B0_06_MUX_ADDR: u32 = 0x401f8674;
    const GPIO_SPI_B0_06_PAD_ADDR: u32 = 0x401f86cc;
    pub type GPIO_SPI_B0_06 = crate::Pad<GPIO_SPI_B0_06_MUX_ADDR, GPIO_SPI_B0_06_PAD_ADDR>;
    // GPIO_SPI_B0_06 does not have any GPIO alternates.

    const GPIO_SPI_B0_07_MUX_ADDR: u32 = 0x401f8678;
    const GPIO_SPI_B0_07_PAD_ADDR: u32 = 0x401f86d0;
    pub type GPIO_SPI_B0_07 = crate::Pad<GPIO_SPI_B0_07_MUX_ADDR, GPIO_SPI_B0_07_PAD_ADDR>;
    // GPIO_SPI_B0_07 does not have any GPIO alternates.

    const GPIO_SPI_B0_08_MUX_ADDR: u32 = 0x401f867c;
    const GPIO_SPI_B0_08_PAD_ADDR: u32 = 0x401f86d4;
    pub type GPIO_SPI_B0_08 = crate::Pad<GPIO_SPI_B0_08_MUX_ADDR, GPIO_SPI_B0_08_PAD_ADDR>;
    // GPIO_SPI_B0_08 does not have any GPIO alternates.

    const GPIO_SPI_B0_09_MUX_ADDR: u32 = 0x401f8680;
    const GPIO_SPI_B0_09_PAD_ADDR: u32 = 0x401f86d8;
    pub type GPIO_SPI_B0_09 = crate::Pad<GPIO_SPI_B0_09_MUX_ADDR, GPIO_SPI_B0_09_PAD_ADDR>;
    // GPIO_SPI_B0_09 does not have any GPIO alternates.

    const GPIO_SPI_B0_10_MUX_ADDR: u32 = 0x401f8684;
    const GPIO_SPI_B0_10_PAD_ADDR: u32 = 0x401f86dc;
    pub type GPIO_SPI_B0_10 = crate::Pad<GPIO_SPI_B0_10_MUX_ADDR, GPIO_SPI_B0_10_PAD_ADDR>;
    // GPIO_SPI_B0_10 does not have any GPIO alternates.

    const GPIO_SPI_B0_11_MUX_ADDR: u32 = 0x401f8688;
    const GPIO_SPI_B0_11_PAD_ADDR: u32 = 0x401f86e0;
    pub type GPIO_SPI_B0_11 = crate::Pad<GPIO_SPI_B0_11_MUX_ADDR, GPIO_SPI_B0_11_PAD_ADDR>;
    // GPIO_SPI_B0_11 does not have any GPIO alternates.

    const GPIO_SPI_B0_12_MUX_ADDR: u32 = 0x401f868c;
    const GPIO_SPI_B0_12_PAD_ADDR: u32 = 0x401f86e4;
    pub type GPIO_SPI_B0_12 = crate::Pad<GPIO_SPI_B0_12_MUX_ADDR, GPIO_SPI_B0_12_PAD_ADDR>;
    // GPIO_SPI_B0_12 does not have any GPIO alternates.

    const GPIO_SPI_B0_13_MUX_ADDR: u32 = 0x401f8690;
    const GPIO_SPI_B0_13_PAD_ADDR: u32 = 0x401f86e8;
    pub type GPIO_SPI_B0_13 = crate::Pad<GPIO_SPI_B0_13_MUX_ADDR, GPIO_SPI_B0_13_PAD_ADDR>;
    // GPIO_SPI_B0_13 does not have any GPIO alternates.

    /// All pads with prefix GPIO_SPI_B0.
    pub struct Pads {
        pub p00: GPIO_SPI_B0_00,
        pub p01: GPIO_SPI_B0_01,
        pub p02: GPIO_SPI_B0_02,
        pub p03: GPIO_SPI_B0_03,
        pub p04: GPIO_SPI_B0_04,
        pub p05: GPIO_SPI_B0_05,
        pub p06: GPIO_SPI_B0_06,
        pub p07: GPIO_SPI_B0_07,
        pub p08: GPIO_SPI_B0_08,
        pub p09: GPIO_SPI_B0_09,
        pub p10: GPIO_SPI_B0_10,
        pub p11: GPIO_SPI_B0_11,
        pub p12: GPIO_SPI_B0_12,
        pub p13: GPIO_SPI_B0_13,
    }
    /// Erased pads with prefix GPIO_SPI_B0.
    ///
    /// Use [`Pads::erase()`] to get an `ErasedPads` instance.
    pub type ErasedPads = [crate::ErasedPad; 14];
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
                p00: GPIO_SPI_B0_00::new(),
                p01: GPIO_SPI_B0_01::new(),
                p02: GPIO_SPI_B0_02::new(),
                p03: GPIO_SPI_B0_03::new(),
                p04: GPIO_SPI_B0_04::new(),
                p05: GPIO_SPI_B0_05::new(),
                p06: GPIO_SPI_B0_06::new(),
                p07: GPIO_SPI_B0_07::new(),
                p08: GPIO_SPI_B0_08::new(),
                p09: GPIO_SPI_B0_09::new(),
                p10: GPIO_SPI_B0_10::new(),
                p11: GPIO_SPI_B0_11::new(),
                p12: GPIO_SPI_B0_12::new(),
                p13: GPIO_SPI_B0_13::new(),
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
            [
                self.p00.erase(),
                self.p01.erase(),
                self.p02.erase(),
                self.p03.erase(),
                self.p04.erase(),
                self.p05.erase(),
                self.p06.erase(),
                self.p07.erase(),
                self.p08.erase(),
                self.p09.erase(),
                self.p10.erase(),
                self.p11.erase(),
                self.p12.erase(),
                self.p13.erase(),
            ]
        }
    }
}

/// Pads with the prefix GPIO_SPI_B1.
pub mod gpio_spi_b1 {
    #![allow(non_camel_case_types)]

    const GPIO_SPI_B1_00_MUX_ADDR: u32 = 0x401f8694;
    const GPIO_SPI_B1_00_PAD_ADDR: u32 = 0x401f86ec;
    pub type GPIO_SPI_B1_00 = crate::Pad<GPIO_SPI_B1_00_MUX_ADDR, GPIO_SPI_B1_00_PAD_ADDR>;
    // GPIO_SPI_B1_00 does not have any GPIO alternates.

    const GPIO_SPI_B1_01_MUX_ADDR: u32 = 0x401f8698;
    const GPIO_SPI_B1_01_PAD_ADDR: u32 = 0x401f86f0;
    pub type GPIO_SPI_B1_01 = crate::Pad<GPIO_SPI_B1_01_MUX_ADDR, GPIO_SPI_B1_01_PAD_ADDR>;
    // GPIO_SPI_B1_01 does not have any GPIO alternates.

    const GPIO_SPI_B1_02_MUX_ADDR: u32 = 0x401f869c;
    const GPIO_SPI_B1_02_PAD_ADDR: u32 = 0x401f86f4;
    pub type GPIO_SPI_B1_02 = crate::Pad<GPIO_SPI_B1_02_MUX_ADDR, GPIO_SPI_B1_02_PAD_ADDR>;
    // GPIO_SPI_B1_02 does not have any GPIO alternates.

    const GPIO_SPI_B1_03_MUX_ADDR: u32 = 0x401f86a0;
    const GPIO_SPI_B1_03_PAD_ADDR: u32 = 0x401f86f8;
    pub type GPIO_SPI_B1_03 = crate::Pad<GPIO_SPI_B1_03_MUX_ADDR, GPIO_SPI_B1_03_PAD_ADDR>;
    // GPIO_SPI_B1_03 does not have any GPIO alternates.

    const GPIO_SPI_B1_04_MUX_ADDR: u32 = 0x401f86a4;
    const GPIO_SPI_B1_04_PAD_ADDR: u32 = 0x401f86fc;
    pub type GPIO_SPI_B1_04 = crate::Pad<GPIO_SPI_B1_04_MUX_ADDR, GPIO_SPI_B1_04_PAD_ADDR>;
    // GPIO_SPI_B1_04 does not have any GPIO alternates.

    const GPIO_SPI_B1_05_MUX_ADDR: u32 = 0x401f86a8;
    const GPIO_SPI_B1_05_PAD_ADDR: u32 = 0x401f8700;
    pub type GPIO_SPI_B1_05 = crate::Pad<GPIO_SPI_B1_05_MUX_ADDR, GPIO_SPI_B1_05_PAD_ADDR>;
    // GPIO_SPI_B1_05 does not have any GPIO alternates.

    const GPIO_SPI_B1_06_MUX_ADDR: u32 = 0x401f86ac;
    const GPIO_SPI_B1_06_PAD_ADDR: u32 = 0x401f8704;
    pub type GPIO_SPI_B1_06 = crate::Pad<GPIO_SPI_B1_06_MUX_ADDR, GPIO_SPI_B1_06_PAD_ADDR>;
    // GPIO_SPI_B1_06 does not have any GPIO alternates.

    const GPIO_SPI_B1_07_MUX_ADDR: u32 = 0x401f86b0;
    const GPIO_SPI_B1_07_PAD_ADDR: u32 = 0x401f8708;
    pub type GPIO_SPI_B1_07 = crate::Pad<GPIO_SPI_B1_07_MUX_ADDR, GPIO_SPI_B1_07_PAD_ADDR>;
    // GPIO_SPI_B1_07 does not have any GPIO alternates.

    /// All pads with prefix GPIO_SPI_B1.
    pub struct Pads {
        pub p00: GPIO_SPI_B1_00,
        pub p01: GPIO_SPI_B1_01,
        pub p02: GPIO_SPI_B1_02,
        pub p03: GPIO_SPI_B1_03,
        pub p04: GPIO_SPI_B1_04,
        pub p05: GPIO_SPI_B1_05,
        pub p06: GPIO_SPI_B1_06,
        pub p07: GPIO_SPI_B1_07,
    }
    /// Erased pads with prefix GPIO_SPI_B1.
    ///
    /// Use [`Pads::erase()`] to get an `ErasedPads` instance.
    pub type ErasedPads = [crate::ErasedPad; 8];
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
                p00: GPIO_SPI_B1_00::new(),
                p01: GPIO_SPI_B1_01::new(),
                p02: GPIO_SPI_B1_02::new(),
                p03: GPIO_SPI_B1_03::new(),
                p04: GPIO_SPI_B1_04::new(),
                p05: GPIO_SPI_B1_05::new(),
                p06: GPIO_SPI_B1_06::new(),
                p07: GPIO_SPI_B1_07::new(),
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
            [
                self.p00.erase(),
                self.p01.erase(),
                self.p02.erase(),
                self.p03.erase(),
                self.p04.erase(),
                self.p05.erase(),
                self.p06.erase(),
                self.p07.erase(),
            ]
        }
    }
}

/// All of the pads.
pub struct Pads {
    pub gpio_emc: gpio_emc::Pads,
    pub gpio_ad_b0: gpio_ad_b0::Pads,
    pub gpio_ad_b1: gpio_ad_b1::Pads,
    pub gpio_b0: gpio_b0::Pads,
    pub gpio_b1: gpio_b1::Pads,
    pub gpio_sd_b0: gpio_sd_b0::Pads,
    pub gpio_sd_b1: gpio_sd_b1::Pads,
    pub gpio_spi_b0: gpio_spi_b0::Pads,
    pub gpio_spi_b1: gpio_spi_b1::Pads,
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
            gpio_spi_b0: gpio_spi_b0::Pads::new(),
            gpio_spi_b1: gpio_spi_b1::Pads::new(),
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
            gpio_spi_b0: self.gpio_spi_b0.erase(),
            gpio_spi_b1: self.gpio_spi_b1.erase(),
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
    pub gpio_spi_b0: gpio_spi_b0::ErasedPads,
    pub gpio_spi_b1: gpio_spi_b1::ErasedPads,
}
