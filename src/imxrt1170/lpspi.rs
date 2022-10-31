use super::pads::gpio_ad::*;
use crate::{
    consts::*,
    lpspi::{Pcs0, Pin, Sck, Sdi, Sdo},
    Daisy,
};

//
// SPI1
//

// PCS0
spi!(module: U1, alt: 0, pad: GPIO_AD_29, signal: Pcs0, daisy: DAISY_LPSPI1_LPSPI_PCS_0_SELECT_GPIO_AD_29);

// SCK
spi!(module: U1, alt: 0, pad: GPIO_AD_28, signal: Sck, daisy: DAISY_LPSPI1_LPSPI_SCK_SELECT_GPIO_AD_28);

// SDI
spi!(module: U1, alt: 0, pad: GPIO_AD_31, signal: Sdi, daisy: DAISY_LPSPI1_LPSPI_SDI_SELECT_GPIO_AD_31);

// SDO
spi!(module: U1, alt: 0, pad: GPIO_AD_30, signal: Sdo, daisy: DAISY_LPSPI1_LPSPI_SDO_SELECT_GPIO_AD_30);

mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_LPSPI1_LPSPI_PCS_0_SELECT_GPIO_EMC_B2_01: Daisy =
        Daisy::new(0x400e85cc as *mut u32, 0);
    pub const DAISY_LPSPI1_LPSPI_PCS_0_SELECT_GPIO_AD_29: Daisy =
        Daisy::new(0x400e85cc as *mut u32, 1);
    pub const DAISY_LPSPI1_LPSPI_SCK_SELECT_GPIO_EMC_B2_00: Daisy =
        Daisy::new(0x400e85d0 as *mut u32, 0);
    pub const DAISY_LPSPI1_LPSPI_SCK_SELECT_GPIO_AD_28: Daisy =
        Daisy::new(0x400e85d0 as *mut u32, 1);
    pub const DAISY_LPSPI1_LPSPI_SDI_SELECT_GPIO_EMC_B2_03: Daisy =
        Daisy::new(0x400e85d4 as *mut u32, 0);
    pub const DAISY_LPSPI1_LPSPI_SDI_SELECT_GPIO_AD_31: Daisy =
        Daisy::new(0x400e85d4 as *mut u32, 1);
    pub const DAISY_LPSPI1_LPSPI_SDO_SELECT_GPIO_EMC_B2_02: Daisy =
        Daisy::new(0x400e85d8 as *mut u32, 0);
    pub const DAISY_LPSPI1_LPSPI_SDO_SELECT_GPIO_AD_30: Daisy =
        Daisy::new(0x400e85d8 as *mut u32, 1);
    pub const DAISY_LPSPI2_LPSPI_PCS_0_SELECT_GPIO_AD_25: Daisy =
        Daisy::new(0x400e85dc as *mut u32, 0);
    pub const DAISY_LPSPI2_LPSPI_PCS_0_SELECT_GPIO_SD_B2_08: Daisy =
        Daisy::new(0x400e85dc as *mut u32, 1);
    pub const DAISY_LPSPI2_LPSPI_PCS_1_SELECT_GPIO_AD_21: Daisy =
        Daisy::new(0x400e85e0 as *mut u32, 0);
    pub const DAISY_LPSPI2_LPSPI_PCS_1_SELECT_GPIO_SD_B2_11: Daisy =
        Daisy::new(0x400e85e0 as *mut u32, 1);
    pub const DAISY_LPSPI2_LPSPI_SCK_SELECT_GPIO_AD_24: Daisy =
        Daisy::new(0x400e85e4 as *mut u32, 0);
    pub const DAISY_LPSPI2_LPSPI_SCK_SELECT_GPIO_SD_B2_07: Daisy =
        Daisy::new(0x400e85e4 as *mut u32, 1);
    pub const DAISY_LPSPI2_LPSPI_SDI_SELECT_GPIO_AD_27: Daisy =
        Daisy::new(0x400e85e8 as *mut u32, 0);
    pub const DAISY_LPSPI2_LPSPI_SDI_SELECT_GPIO_SD_B2_10: Daisy =
        Daisy::new(0x400e85e8 as *mut u32, 1);
    pub const DAISY_LPSPI2_LPSPI_SDO_SELECT_GPIO_AD_26: Daisy =
        Daisy::new(0x400e85ec as *mut u32, 0);
    pub const DAISY_LPSPI2_LPSPI_SDO_SELECT_GPIO_SD_B2_09: Daisy =
        Daisy::new(0x400e85ec as *mut u32, 1);
    pub const DAISY_LPSPI3_LPSPI_PCS_0_SELECT_GPIO_EMC_B2_05: Daisy =
        Daisy::new(0x400e85f0 as *mut u32, 0);
    pub const DAISY_LPSPI3_LPSPI_PCS_0_SELECT_GPIO_DISP_B1_07: Daisy =
        Daisy::new(0x400e85f0 as *mut u32, 1);
    pub const DAISY_LPSPI3_LPSPI_PCS_1_SELECT_GPIO_EMC_B2_08: Daisy =
        Daisy::new(0x400e85f4 as *mut u32, 0);
    pub const DAISY_LPSPI3_LPSPI_PCS_1_SELECT_GPIO_DISP_B1_08: Daisy =
        Daisy::new(0x400e85f4 as *mut u32, 1);
    pub const DAISY_LPSPI3_LPSPI_PCS_2_SELECT_GPIO_EMC_B2_09: Daisy =
        Daisy::new(0x400e85f8 as *mut u32, 0);
    pub const DAISY_LPSPI3_LPSPI_PCS_2_SELECT_GPIO_DISP_B1_09: Daisy =
        Daisy::new(0x400e85f8 as *mut u32, 1);
    pub const DAISY_LPSPI3_LPSPI_PCS_3_SELECT_GPIO_EMC_B2_10: Daisy =
        Daisy::new(0x400e85fc as *mut u32, 0);
    pub const DAISY_LPSPI3_LPSPI_PCS_3_SELECT_GPIO_DISP_B1_10: Daisy =
        Daisy::new(0x400e85fc as *mut u32, 1);
    pub const DAISY_LPSPI3_LPSPI_SCK_SELECT_GPIO_EMC_B2_04: Daisy =
        Daisy::new(0x400e8600 as *mut u32, 0);
    pub const DAISY_LPSPI3_LPSPI_SCK_SELECT_GPIO_DISP_B1_04: Daisy =
        Daisy::new(0x400e8600 as *mut u32, 1);
    pub const DAISY_LPSPI3_LPSPI_SDI_SELECT_GPIO_EMC_B2_07: Daisy =
        Daisy::new(0x400e8604 as *mut u32, 0);
    pub const DAISY_LPSPI3_LPSPI_SDI_SELECT_GPIO_DISP_B1_05: Daisy =
        Daisy::new(0x400e8604 as *mut u32, 1);
    pub const DAISY_LPSPI3_LPSPI_SDO_SELECT_GPIO_EMC_B2_06: Daisy =
        Daisy::new(0x400e8608 as *mut u32, 0);
    pub const DAISY_LPSPI3_LPSPI_SDO_SELECT_GPIO_DISP_B1_06: Daisy =
        Daisy::new(0x400e8608 as *mut u32, 1);
    pub const DAISY_LPSPI4_LPSPI_PCS_0_SELECT_GPIO_SD_B2_01: Daisy =
        Daisy::new(0x400e860c as *mut u32, 0);
    pub const DAISY_LPSPI4_LPSPI_PCS_0_SELECT_GPIO_DISP_B2_15: Daisy =
        Daisy::new(0x400e860c as *mut u32, 1);
    pub const DAISY_LPSPI4_LPSPI_SCK_SELECT_GPIO_SD_B2_00: Daisy =
        Daisy::new(0x400e8610 as *mut u32, 0);
    pub const DAISY_LPSPI4_LPSPI_SCK_SELECT_GPIO_DISP_B2_12: Daisy =
        Daisy::new(0x400e8610 as *mut u32, 1);
    pub const DAISY_LPSPI4_LPSPI_SDI_SELECT_GPIO_SD_B2_03: Daisy =
        Daisy::new(0x400e8614 as *mut u32, 0);
    pub const DAISY_LPSPI4_LPSPI_SDI_SELECT_GPIO_DISP_B2_13: Daisy =
        Daisy::new(0x400e8614 as *mut u32, 1);
    pub const DAISY_LPSPI4_LPSPI_SDO_SELECT_GPIO_SD_B2_02: Daisy =
        Daisy::new(0x400e8618 as *mut u32, 0);
    pub const DAISY_LPSPI4_LPSPI_SDO_SELECT_GPIO_DISP_B2_14: Daisy =
        Daisy::new(0x400e8618 as *mut u32, 1);
}

use daisy::*;
