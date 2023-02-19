//! SPI pin implementations

use super::pads::{gpio_ad_b0::*, gpio_ad_b1::*, gpio_emc::*, gpio_sd_b0::*, gpio_sd_b1::*};
use crate::{
    consts::*,
    lpspi::{Pcs0, Pin, Sck, Sdi, Sdo},
    Daisy,
};

//
// SPI1
//

// PCS0
spi!(module: U1, alt: 1, pad: GPIO_AD_B0_11, signal: Pcs0, daisy: Some(DAISY_LPSPI1_PCS0_GPIO_AD_B0_11));
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_03, signal: Pcs0, daisy: Some(DAISY_LPSPI1_PCS0_GPIO_SD_B0_03));

// SCK
spi!(module: U1, alt: 1, pad: GPIO_AD_B0_10, signal: Sck, daisy: Some(DAISY_LPSPI1_SCK_GPIO_AD_B0_10));
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_02, signal: Sck, daisy: Some(DAISY_LPSPI1_SCK_GPIO_SD_B0_02));

// SDI
spi!(module: U1, alt: 1, pad: GPIO_AD_B0_13, signal: Sdi, daisy: Some(DAISY_LPSPI1_SDI_GPIO_AD_B0_13));
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_05, signal: Sdi, daisy: Some(DAISY_LPSPI1_SDI_GPIO_SD_B0_05));

// SDO
spi!(module: U1, alt: 1, pad: GPIO_AD_B0_12, signal: Sdo, daisy: Some(DAISY_LPSPI1_SDO_GPIO_AD_B0_12));
spi!(module: U1, alt: 4, pad: GPIO_SD_B0_04, signal: Sdo, daisy: Some(DAISY_LPSPI1_SDO_GPIO_SD_B0_04));

//
// SPI2
//

// PCS0
spi!(module: U2, alt: 4, pad: GPIO_EMC_01, signal: Pcs0, daisy: Some(DAISY_LPSPI2_PCS0_GPIO_EMC_01));
spi!(module: U2, alt: 4, pad: GPIO_EMC_11, signal: Pcs0, daisy: Some(DAISY_LPSPI2_PCS0_GPIO_EMC_11));
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_06, signal: Pcs0, daisy: Some(DAISY_LPSPI2_PCS0_GPIO_SD_B1_06));

// SCK
spi!(module: U2, alt: 4, pad: GPIO_EMC_00, signal: Sck, daisy: Some(DAISY_LPSPI2_SCK_GPIO_EMC_00));
spi!(module: U2, alt: 4, pad: GPIO_EMC_10, signal: Sck, daisy: Some(DAISY_LPSPI2_SCK_GPIO_EMC_10));
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_07, signal: Sck, daisy: Some(DAISY_LPSPI2_SCK_GPIO_SD_B1_07));

// SDI
spi!(module: U2, alt: 4, pad: GPIO_EMC_03, signal: Sdi, daisy: Some(DAISY_LPSPI2_SDI_GPIO_EMC_03));
spi!(module: U2, alt: 4, pad: GPIO_EMC_13, signal: Sdi, daisy: Some(DAISY_LPSPI2_SDI_GPIO_EMC_13));
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_09, signal: Sdi, daisy: Some(DAISY_LPSPI2_SDI_GPIO_SD_B1_09));

// SD0
spi!(module: U2, alt: 4, pad: GPIO_EMC_02, signal: Sdo, daisy: Some(DAISY_LPSPI2_SDO_GPIO_EMC_02));
spi!(module: U2, alt: 4, pad: GPIO_EMC_12, signal: Sdo, daisy: Some(DAISY_LPSPI2_SDO_GPIO_EMC_12));
spi!(module: U2, alt: 4, pad: GPIO_SD_B1_08, signal: Sdo, daisy: Some(DAISY_LPSPI2_SDO_GPIO_SD_B1_08));

//
// SPI3
//

// PCS0
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_13, signal: Pcs0, daisy: None);

// SCK
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_12, signal: Sck, daisy: None);

// SDI
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_15, signal: Sdi, daisy: None);

// SDO
spi!(module: U3, alt: 2, pad: GPIO_AD_B1_14, signal: Sdo, daisy: None);

//
// SPI4
//

// PCS0
spi!(module: U4, alt: 2, pad: GPIO_AD_B1_03, signal: Pcs0, daisy: Some(DAISY_LPSPI4_PCS0_GPIO_AD_B1_03));
spi!(module: U4, alt: 4, pad: GPIO_EMC_33, signal: Pcs0, daisy: Some(DAISY_LPSPI4_PCS0_GPIO_EMC_33));

// SCK
spi!(module: U4, alt: 2, pad: GPIO_AD_B1_02, signal: Sck, daisy: Some(DAISY_LPSPI4_SCK_GPIO_AD_B1_02));
spi!(module: U4, alt: 4, pad: GPIO_EMC_32, signal: Sck, daisy: Some(DAISY_LPSPI4_SCK_GPIO_EMC_32));

// SDI
spi!(module: U4, alt: 2, pad: GPIO_AD_B1_05, signal: Sdi, daisy: Some(DAISY_LPSPI4_SDI_GPIO_AD_B1_05));
spi!(module: U4, alt: 4, pad: GPIO_EMC_35, signal: Sdi, daisy: Some(DAISY_LPSPI4_SDI_GPIO_EMC_35));

// SDO
spi!(module: U4, alt: 2, pad: GPIO_AD_B1_04, signal: Sdi, daisy: Some(DAISY_LPSPI4_SDO_GPIO_AD_B1_04));
spi!(module: U4, alt: 4, pad: GPIO_EMC_34, signal: Sdi, daisy: Some(DAISY_LPSPI4_SDO_GPIO_EMC_34));

mod daisy {
    use super::Daisy;

    pub const DAISY_LPSPI1_PCS0_GPIO_SD_B0_03: Daisy = Daisy::new(0x401f839c as *mut u32, 0);
    pub const DAISY_LPSPI1_PCS0_GPIO_AD_B0_11: Daisy = Daisy::new(0x401f839c as *mut u32, 1);
    pub const DAISY_LPSPI1_SCK_GPIO_SD_B0_02: Daisy = Daisy::new(0x401f83a0 as *mut u32, 0);
    pub const DAISY_LPSPI1_SCK_GPIO_AD_B0_10: Daisy = Daisy::new(0x401f83a0 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDI_GPIO_SD_B0_05: Daisy = Daisy::new(0x401f83a4 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDI_GPIO_AD_B0_13: Daisy = Daisy::new(0x401f83a4 as *mut u32, 1);
    pub const DAISY_LPSPI1_SDO_GPIO_SD_B0_04: Daisy = Daisy::new(0x401f83a8 as *mut u32, 0);
    pub const DAISY_LPSPI1_SDO_GPIO_AD_B0_12: Daisy = Daisy::new(0x401f83a8 as *mut u32, 1);
    pub const DAISY_LPSPI2_PCS0_GPIO_EMC_01: Daisy = Daisy::new(0x401f83ac as *mut u32, 0);
    pub const DAISY_LPSPI2_PCS0_GPIO_EMC_11: Daisy = Daisy::new(0x401f83ac as *mut u32, 1);
    pub const DAISY_LPSPI2_PCS0_GPIO_SD_B1_06: Daisy = Daisy::new(0x401f83ac as *mut u32, 2);
    pub const DAISY_LPSPI2_SCK_GPIO_EMC_00: Daisy = Daisy::new(0x401f83b0 as *mut u32, 0);
    pub const DAISY_LPSPI2_SCK_GPIO_EMC_10: Daisy = Daisy::new(0x401f83b0 as *mut u32, 1);
    pub const DAISY_LPSPI2_SCK_GPIO_SD_B1_07: Daisy = Daisy::new(0x401f83b0 as *mut u32, 2);
    pub const DAISY_LPSPI2_SDI_GPIO_EMC_03: Daisy = Daisy::new(0x401f83b4 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDI_GPIO_EMC_13: Daisy = Daisy::new(0x401f83b4 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDI_GPIO_SD_B1_09: Daisy = Daisy::new(0x401f83b4 as *mut u32, 2);
    pub const DAISY_LPSPI2_SDO_GPIO_EMC_02: Daisy = Daisy::new(0x401f83b8 as *mut u32, 0);
    pub const DAISY_LPSPI2_SDO_GPIO_EMC_12: Daisy = Daisy::new(0x401f83b8 as *mut u32, 1);
    pub const DAISY_LPSPI2_SDO_GPIO_SD_B1_08: Daisy = Daisy::new(0x401f83b8 as *mut u32, 2);
    pub const DAISY_LPSPI4_PCS0_GPIO_AD_B1_03: Daisy = Daisy::new(0x401f83bc as *mut u32, 0);
    pub const DAISY_LPSPI4_PCS0_GPIO_EMC_33: Daisy = Daisy::new(0x401f83bc as *mut u32, 1);
    pub const DAISY_LPSPI4_SCK_GPIO_AD_B1_02: Daisy = Daisy::new(0x401f83c0 as *mut u32, 0);
    pub const DAISY_LPSPI4_SCK_GPIO_EMC_32: Daisy = Daisy::new(0x401f83c0 as *mut u32, 1);
    pub const DAISY_LPSPI4_SDI_GPIO_AD_B1_05: Daisy = Daisy::new(0x401f83c4 as *mut u32, 0);
    pub const DAISY_LPSPI4_SDI_GPIO_EMC_35: Daisy = Daisy::new(0x401f83c4 as *mut u32, 1);
    pub const DAISY_LPSPI4_SDO_GPIO_AD_B1_04: Daisy = Daisy::new(0x401f83c8 as *mut u32, 0);
    pub const DAISY_LPSPI4_SDO_GPIO_EMC_34: Daisy = Daisy::new(0x401f83c8 as *mut u32, 1);
}
use daisy::*;
