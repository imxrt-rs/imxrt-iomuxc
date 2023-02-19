//! SPI pin implementations

use super::pads::{gpio_ad::*, gpio_sd::*};
use crate::{
    consts::*,
    lpspi::{Pcs0, Pin, Sck, Sdi, Sdo},
    Daisy,
};

//
// SPI1
//

// PCS0
spi!(module: U1, alt: 0, pad: GPIO_AD_05, signal: Pcs0, daisy: Some(DAISY_LPSPI1_PCS_0_GPIO_AD_05));
spi!(module: U1, alt: 2, pad: GPIO_SD_07, signal: Pcs0, daisy: Some(DAISY_LPSPI1_PCS_0_GPIO_SD_07));

// SCK
spi!(module: U1, alt: 0, pad: GPIO_AD_06, signal: Sck, daisy: Some(DAISY_LPSPI1_SCK_GPIO_AD_06));
spi!(module: U1, alt: 2, pad: GPIO_SD_08, signal: Sck, daisy: Some(DAISY_LPSPI1_SCK_GPIO_SD_08));

// SDI
spi!(module: U1, alt: 0, pad: GPIO_AD_03, signal: Sdi, daisy: Some(DAISY_LPSPI1_SDI_GPIO_AD_03));
spi!(module: U1, alt: 2, pad: GPIO_SD_05, signal: Sdi, daisy: Some(DAISY_LPSPI1_SDI_GPIO_SD_05));

// SDO
spi!(module: U1, alt: 0, pad: GPIO_AD_04, signal: Sdo, daisy: Some(DAISY_LPSPI1_SDO_GPIO_AD_04));
spi!(module: U1, alt: 2, pad: GPIO_SD_06, signal: Sdo, daisy: Some(DAISY_LPSPI1_SDO_GPIO_SD_06));

//
// SPI2
//

// PCS0
spi!(module: U2, alt: 0, pad: GPIO_AD_11, signal: Pcs0, daisy: Some(DAISY_LPSPI2_PCS_0_GPIO_AD_11));
spi!(module: U2, alt: 1, pad: GPIO_SD_12, signal: Pcs0, daisy: Some(DAISY_LPSPI2_PCS_0_GPIO_SD_12));

// SCK
spi!(module: U2, alt: 0, pad: GPIO_AD_12, signal: Sck, daisy: Some(DAISY_LPSPI2_SCK_GPIO_AD_12));
spi!(module: U2, alt: 1, pad: GPIO_SD_11, signal: Sck, daisy: Some(DAISY_LPSPI2_SCK_GPIO_SD_11));

// SDI
spi!(module: U2, alt: 0, pad: GPIO_AD_09, signal: Sdi, daisy: Some(DAISY_LPSPI2_SDI_GPIO_AD_09));
spi!(module: U2, alt: 1, pad: GPIO_SD_09, signal: Sdi, daisy: Some(DAISY_LPSPI2_SDI_GPIO_SD_09));

// SDO
spi!(module: U2, alt: 0, pad: GPIO_AD_10, signal: Sdo, daisy: Some(DAISY_LPSPI2_SDO_GPIO_AD_10));
spi!(module: U2, alt: 1, pad: GPIO_SD_10, signal: Sdo, daisy: Some(DAISY_LPSPI2_SDO_GPIO_SD_10));

mod daisy {
    use super::Daisy;

    pub const DAISY_LPSPI1_PCS_0_GPIO_AD_05: Daisy = Daisy::new(0x401f81d0 as *mut u32, 0);
    pub const DAISY_LPSPI1_PCS_0_GPIO_SD_07: Daisy = Daisy::new(0x401f81d0 as *mut u32, 1);
    pub const DAISY_LPSPI1_SCK_GPIO_AD_06: Daisy = Daisy::new(0x401f81d4 as *mut u32, 0);
    pub const DAISY_LPSPI1_SCK_GPIO_SD_08: Daisy = Daisy::new(0x401f81d4 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDI_GPIO_AD_03: Daisy = Daisy::new(0x401f81d8 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDI_GPIO_SD_05: Daisy = Daisy::new(0x401f81d8 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDO_GPIO_AD_04: Daisy = Daisy::new(0x401f81dc as *mut u32, 0);
    pub const DAISY_LPSPI1_SDO_GPIO_SD_06: Daisy = Daisy::new(0x401f81dc as *mut u32, 1);
    pub const DAISY_LPSPI2_PCS_0_GPIO_AD_11: Daisy = Daisy::new(0x401f81e0 as *mut u32, 0);
    pub const DAISY_LPSPI2_PCS_0_GPIO_SD_12: Daisy = Daisy::new(0x401f81e0 as *mut u32, 1);
    pub const DAISY_LPSPI2_SCK_GPIO_AD_12: Daisy = Daisy::new(0x401f81e4 as *mut u32, 0);
    pub const DAISY_LPSPI2_SCK_GPIO_SD_11: Daisy = Daisy::new(0x401f81e4 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDI_GPIO_AD_09: Daisy = Daisy::new(0x401f81e8 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDI_GPIO_SD_09: Daisy = Daisy::new(0x401f81e8 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDO_GPIO_AD_10: Daisy = Daisy::new(0x401f81ec as *mut u32, 0);
    pub const DAISY_LPSPI2_SDO_GPIO_SD_10: Daisy = Daisy::new(0x401f81ec as *mut u32, 1);
}

use daisy::*;
