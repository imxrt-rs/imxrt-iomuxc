//! SPI pin implementations

use super::pads::{ad::*, sd::*};
use crate::{
    consts::*,
    spi::{Pcs0, Pin, Sck, Sdi, Sdo},
    Daisy,
};

//
// SPI1
//

// PCS0
spi!(module: U1, alt: 0, pad: AD_05, signal: Pcs0, daisy: DAISY_LPSPI1_PCS_0_AD_05);

spi!(module: U1, alt: 2, pad: SD_07, signal: Pcs0, daisy: DAISY_LPSPI1_PCS_0_SD_07);

// SCK
spi!(module: U1, alt: 0, pad: AD_06, signal: Sck, daisy: DAISY_LPSPI1_SCK_AD_06);

spi!(module: U1, alt: 2, pad: SD_08, signal: Sck, daisy: DAISY_LPSPI1_SCK_SD_08);

// SDI
spi!(module: U1, alt: 0, pad: AD_03, signal: Sdi, daisy: DAISY_LPSPI1_SDI_AD_03);

spi!(module: U1, alt: 2, pad: SD_05, signal: Sdi, daisy: DAISY_LPSPI1_SDI_SD_05);

// SDO
spi!(module: U1, alt: 0, pad: AD_04, signal: Sdo, daisy: DAISY_LPSPI1_SDO_AD_04);

spi!(module: U1, alt: 2, pad: SD_06, signal: Sdo, daisy: DAISY_LPSPI1_SDO_SD_06);

//
// SPI2
//

// PCS0
spi!(module: U2, alt: 0, pad: AD_11, signal: Pcs0, daisy: DAISY_LPSPI2_PCS_0_AD_11);

spi!(module: U2, alt: 1, pad: SD_12, signal: Pcs0, daisy: DAISY_LPSPI2_PCS_0_SD_12);

// SCK
spi!(module: U2, alt: 0, pad: AD_12, signal: Sck, daisy: DAISY_LPSPI2_SCK_AD_12);

spi!(module: U2, alt: 1, pad: SD_11, signal: Sck, daisy: DAISY_LPSPI2_SCK_SD_11);

// SDI
spi!(module: U2, alt: 0, pad: AD_09, signal: Sdi, daisy: DAISY_LPSPI2_SDI_AD_09);

spi!(module: U2, alt: 1, pad: SD_09, signal: Sdi, daisy: DAISY_LPSPI2_SDI_SD_09);

// SDO
spi!(module: U2, alt: 0, pad: AD_10, signal: Sdo, daisy: DAISY_LPSPI2_SDO_AD_10);

spi!(module: U2, alt: 1, pad: SD_10, signal: Sdo, daisy: DAISY_LPSPI2_SDO_SD_10);

mod daisy {
    #![allow(unused)]

    use super::Daisy;

    pub const DAISY_LPSPI1_PCS_0_AD_05: Daisy = Daisy::new(0x401f81d0 as *mut u32, 0);
    pub const DAISY_LPSPI1_PCS_0_SD_07: Daisy = Daisy::new(0x401f81d0 as *mut u32, 1);
    pub const DAISY_LPSPI1_SCK_AD_06: Daisy = Daisy::new(0x401f81d4 as *mut u32, 0);
    pub const DAISY_LPSPI1_SCK_SD_08: Daisy = Daisy::new(0x401f81d4 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDI_AD_03: Daisy = Daisy::new(0x401f81d8 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDI_SD_05: Daisy = Daisy::new(0x401f81d8 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDO_AD_04: Daisy = Daisy::new(0x401f81dc as *mut u32, 0);
    pub const DAISY_LPSPI1_SDO_SD_06: Daisy = Daisy::new(0x401f81dc as *mut u32, 1);
    pub const DAISY_LPSPI2_PCS_0_AD_11: Daisy = Daisy::new(0x401f81e0 as *mut u32, 0);
    pub const DAISY_LPSPI2_PCS_0_SD_12: Daisy = Daisy::new(0x401f81e0 as *mut u32, 1);
    pub const DAISY_LPSPI2_SCK_AD_12: Daisy = Daisy::new(0x401f81e4 as *mut u32, 0);
    pub const DAISY_LPSPI2_SCK_SD_11: Daisy = Daisy::new(0x401f81e4 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDI_AD_09: Daisy = Daisy::new(0x401f81e8 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDI_SD_09: Daisy = Daisy::new(0x401f81e8 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDO_AD_10: Daisy = Daisy::new(0x401f81ec as *mut u32, 0);
    pub const DAISY_LPSPI2_SDO_SD_10: Daisy = Daisy::new(0x401f81ec as *mut u32, 1);
}

use daisy::*;
