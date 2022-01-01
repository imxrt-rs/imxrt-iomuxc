//! SPI pin implementation

use super::pads::{
    gpio_ad_b0::*, gpio_ad_b1::*, gpio_b0::*, gpio_emc::*, gpio_sd_b0::*, gpio_sd_b1::*,
    gpio_b1::*,
};
use crate::{
    consts::*,
    lpspi::{Pcs0, Pin, Sck, Sdi, Sdo},
    Daisy,
};

//
// SPI1
//
spi!(module: U1, alt: 3, pad: GPIO_EMC_30,   signal: Pcs0, daisy: DAISY_LPSPI1_PCS0_GPIO_EMC_30);
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_01, signal: Pcs0, daisy: DAISY_LPSPI1_PCS0_GPIO_SD_B0_01);
spi!(module: U1, alt: 3, pad: GPIO_EMC_27,   signal: Sck,  daisy: DAISY_LPSPI1_SCK_GPIO_EMC_27);
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_00, signal: Sck,  daisy: DAISY_LPSPI1_SCK_GPIO_SD_B0_00);
spi!(module: U1, alt: 3, pad: GPIO_EMC_29,   signal: Sdi,  daisy: DAISY_LPSPI1_SDI_GPIO_EMC_29);
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_03, signal: Sdi,  daisy: DAISY_LPSPI1_SDI_GPIO_SD_B0_03);
spi!(module: U1, alt: 3, pad: GPIO_EMC_28,   signal: Sdo,  daisy: DAISY_LPSPI1_SDO_GPIO_EMC_28);
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_02, signal: Sdo,  daisy: DAISY_LPSPI1_SDO_GPIO_SD_B0_02);

//
// SPI2
//
spi!(module: U2, alt: 2, pad: GPIO_EMC_00,   signal: Sck,  daisy: DAISY_LPSPI2_SCK_GPIO_EMC_00);
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_07, signal: Sck,  daisy: DAISY_LPSPI2_SCK_GPIO_SD_B1_07);
spi!(module: U2, alt: 2, pad: GPIO_EMC_02,   signal: Sdo,  daisy: DAISY_LPSPI2_SDO_GPIO_EMC_02);
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_08, signal: Sdo,  daisy: DAISY_LPSPI2_SDO_GPIO_SD_B1_08);
spi!(module: U2, alt: 2, pad: GPIO_EMC_03,   signal: Sdi,  daisy: DAISY_LPSPI2_SDI_GPIO_EMC_03);
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_09, signal: Sdi,  daisy: DAISY_LPSPI2_SDI_GPIO_SD_B1_09);
spi!(module: U2, alt: 2, pad: GPIO_EMC_01,   signal: Pcs0, daisy: DAISY_LPSPI2_PCS0_GPIO_EMC_01);
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_06, signal: Pcs0, daisy: DAISY_LPSPI2_PCS0_GPIO_SD_B1_06);

//
// SPI3
//
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_15, signal: Sck,  daisy: DAISY_LPSPI3_SCK_GPIO_AD_B1_15);
spi!(module: U3, alt: 7, pad: GPIO_AD_B0_00, signal: Sck,  daisy: DAISY_LPSPI3_SCK_GPIO_AD_B0_00);
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_14, signal: Sdo,  daisy: DAISY_LPSPI3_SDO_GPIO_AD_B1_14);
spi!(module: U3, alt: 7, pad: GPIO_AD_B0_01, signal: Sdo,  daisy: DAISY_LPSPI3_SDO_GPIO_AD_B0_01);
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_13, signal: Sdi,  daisy: DAISY_LPSPI3_SDI_GPIO_AD_B1_13);
spi!(module: U3, alt: 7, pad: GPIO_AD_B0_02, signal: Sdi,  daisy: DAISY_LPSPI3_SDI_GPIO_AD_B0_02);
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_12, signal: Pcs0, daisy: DAISY_LPSPI3_PCS0_GPIO_AD_B1_12);
spi!(module: U3, alt: 7, pad: GPIO_AD_B0_03, signal: Pcs0, daisy: DAISY_LPSPI3_PCS0_GPIO_AD_B0_03);

//
// SPI4
//
spi!(module: U4, alt: 1, pad: GPIO_B1_07, signal: Sck,  daisy: DAISY_LPSPI4_SCK_GPIO_B1_07);
spi!(module: U4, alt: 3, pad: GPIO_B0_03, signal: Sck,  daisy: DAISY_LPSPI4_SCK_GPIO_B0_03);
spi!(module: U4, alt: 1, pad: GPIO_B1_06, signal: Sdo,  daisy: DAISY_LPSPI4_SDO_GPIO_B1_06);
spi!(module: U4, alt: 3, pad: GPIO_B0_02, signal: Sdo,  daisy: DAISY_LPSPI4_SDO_GPIO_B0_02);
spi!(module: U4, alt: 1, pad: GPIO_B1_05, signal: Sdi,  daisy: DAISY_LPSPI4_SDI_GPIO_B1_05);
spi!(module: U4, alt: 3, pad: GPIO_B0_01, signal: Sdi,  daisy: DAISY_LPSPI4_SDI_GPIO_B0_01);
spi!(module: U4, alt: 1, pad: GPIO_B1_04, signal: Pcs0, daisy: DAISY_LPSPI4_PCS0_GPIO_B1_04);
spi!(module: U4, alt: 3, pad: GPIO_B0_00, signal: Pcs0, daisy: DAISY_LPSPI4_PCS0_GPIO_B0_00);

/// Auto-generated DAISY values
mod daisy {
    #![allow(unused)]

    use super::Daisy;

    pub const DAISY_LPSPI1_PCS0_GPIO_SD_B0_01: Daisy = Daisy::new(0x401f84ec as *mut u32, 0);
    pub const DAISY_LPSPI1_PCS0_GPIO_EMC_30: Daisy = Daisy::new(0x401f84ec as *mut u32, 1);
    pub const DAISY_LPSPI1_SCK_GPIO_EMC_27: Daisy = Daisy::new(0x401f84f0 as *mut u32, 0);
    pub const DAISY_LPSPI1_SCK_GPIO_SD_B0_00: Daisy = Daisy::new(0x401f84f0 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDI_GPIO_EMC_29: Daisy = Daisy::new(0x401f84f4 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDI_GPIO_SD_B0_03: Daisy = Daisy::new(0x401f84f4 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDO_GPIO_EMC_28: Daisy = Daisy::new(0x401f84f8 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDO_GPIO_SD_B0_02: Daisy = Daisy::new(0x401f84f8 as *mut u32, 1);
    pub const DAISY_LPSPI2_PCS0_GPIO_SD_B1_06: Daisy = Daisy::new(0x401f84fc as *mut u32, 0);
    pub const DAISY_LPSPI2_PCS0_GPIO_EMC_01: Daisy = Daisy::new(0x401f84fc as *mut u32, 1);
    pub const DAISY_LPSPI2_SCK_GPIO_SD_B1_07: Daisy = Daisy::new(0x401f8500 as *mut u32, 0);
    pub const DAISY_LPSPI2_SCK_GPIO_EMC_00: Daisy = Daisy::new(0x401f8500 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDI_GPIO_SD_B1_09: Daisy = Daisy::new(0x401f8504 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDI_GPIO_EMC_03: Daisy = Daisy::new(0x401f8504 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDO_GPIO_SD_B1_08: Daisy = Daisy::new(0x401f8508 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDO_GPIO_EMC_02: Daisy = Daisy::new(0x401f8508 as *mut u32, 1);
    pub const DAISY_LPSPI3_PCS0_GPIO_AD_B0_03: Daisy = Daisy::new(0x401f850c as *mut u32, 0);
    pub const DAISY_LPSPI3_PCS0_GPIO_AD_B1_12: Daisy = Daisy::new(0x401f850c as *mut u32, 1);
    pub const DAISY_LPSPI3_SCK_GPIO_AD_B0_00: Daisy = Daisy::new(0x401f8510 as *mut u32, 0);
    pub const DAISY_LPSPI3_SCK_GPIO_AD_B1_15: Daisy = Daisy::new(0x401f8510 as *mut u32, 1);
    pub const DAISY_LPSPI3_SDI_GPIO_AD_B0_02: Daisy = Daisy::new(0x401f8514 as *mut u32, 0);
    pub const DAISY_LPSPI3_SDI_GPIO_AD_B1_13: Daisy = Daisy::new(0x401f8514 as *mut u32, 1);
    pub const DAISY_LPSPI3_SDO_GPIO_AD_B0_01: Daisy = Daisy::new(0x401f8518 as *mut u32, 0);
    pub const DAISY_LPSPI3_SDO_GPIO_AD_B1_14: Daisy = Daisy::new(0x401f8518 as *mut u32, 1);
    pub const DAISY_LPSPI4_PCS0_GPIO_B0_00: Daisy = Daisy::new(0x401f851c as *mut u32, 0);
    pub const DAISY_LPSPI4_PCS0_GPIO_B1_04: Daisy = Daisy::new(0x401f851c as *mut u32, 1);
    pub const DAISY_LPSPI4_SCK_GPIO_B0_03: Daisy = Daisy::new(0x401f8520 as *mut u32, 0);
    pub const DAISY_LPSPI4_SCK_GPIO_B1_07: Daisy = Daisy::new(0x401f8520 as *mut u32, 1);
    pub const DAISY_LPSPI4_SDI_GPIO_B0_01: Daisy = Daisy::new(0x401f8524 as *mut u32, 0);
    pub const DAISY_LPSPI4_SDI_GPIO_B1_05: Daisy = Daisy::new(0x401f8524 as *mut u32, 1);
    pub const DAISY_LPSPI4_SDO_GPIO_B0_02: Daisy = Daisy::new(0x401f8528 as *mut u32, 0);
    pub const DAISY_LPSPI4_SDO_GPIO_B1_06: Daisy = Daisy::new(0x401f8528 as *mut u32, 1);
}

use daisy::*;
