//! I2C pin implementations

use super::pads::gpio_lpsr::*;
use crate::{
    consts::*,
    lpi2c::{Pin, Scl, Sda},
    Daisy,
};

//
// I2C5
//
i2c!(module: U5, alt: 0, pad: GPIO_LPSR_05, signal: Scl, daisy: Some(DAISY_LPI2C5_IPP_IND_LPI2C_SCL_SELECT_GPIO_LPSR_05));
i2c!(module: U5, alt: 0, pad: GPIO_LPSR_04, signal: Sda, daisy: Some(DAISY_LPI2C5_IPP_IND_LPI2C_SDA_SELECT_GPIO_LPSR_04));

mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_LPI2C1_LPI2C_SCL_SELECT_GPIO_AD_08: Daisy =
        Daisy::new(0x400e85ac as *mut u32, 0);
    pub const DAISY_LPI2C1_LPI2C_SCL_SELECT_GPIO_AD_32: Daisy =
        Daisy::new(0x400e85ac as *mut u32, 1);
    pub const DAISY_LPI2C1_LPI2C_SDA_SELECT_GPIO_AD_09: Daisy =
        Daisy::new(0x400e85b0 as *mut u32, 0);
    pub const DAISY_LPI2C1_LPI2C_SDA_SELECT_GPIO_AD_33: Daisy =
        Daisy::new(0x400e85b0 as *mut u32, 1);
    pub const DAISY_LPI2C2_LPI2C_SCL_SELECT_GPIO_EMC_B2_00: Daisy =
        Daisy::new(0x400e85b4 as *mut u32, 0);
    pub const DAISY_LPI2C2_LPI2C_SCL_SELECT_GPIO_AD_18: Daisy =
        Daisy::new(0x400e85b4 as *mut u32, 1);
    pub const DAISY_LPI2C2_LPI2C_SDA_SELECT_GPIO_EMC_B2_01: Daisy =
        Daisy::new(0x400e85b8 as *mut u32, 0);
    pub const DAISY_LPI2C2_LPI2C_SDA_SELECT_GPIO_AD_19: Daisy =
        Daisy::new(0x400e85b8 as *mut u32, 1);
    pub const DAISY_LPI2C3_LPI2C_SCL_SELECT_GPIO_DISP_B1_02: Daisy =
        Daisy::new(0x400e85bc as *mut u32, 0);
    pub const DAISY_LPI2C3_LPI2C_SCL_SELECT_GPIO_DISP_B2_10: Daisy =
        Daisy::new(0x400e85bc as *mut u32, 1);
    pub const DAISY_LPI2C3_LPI2C_SDA_SELECT_GPIO_DISP_B1_03: Daisy =
        Daisy::new(0x400e85c0 as *mut u32, 0);
    pub const DAISY_LPI2C3_LPI2C_SDA_SELECT_GPIO_DISP_B2_11: Daisy =
        Daisy::new(0x400e85c0 as *mut u32, 1);
    pub const DAISY_LPI2C4_LPI2C_SCL_SELECT_GPIO_AD_24: Daisy =
        Daisy::new(0x400e85c4 as *mut u32, 0);
    pub const DAISY_LPI2C4_LPI2C_SCL_SELECT_GPIO_DISP_B2_12: Daisy =
        Daisy::new(0x400e85c4 as *mut u32, 1);
    pub const DAISY_LPI2C4_LPI2C_SDA_SELECT_GPIO_AD_25: Daisy =
        Daisy::new(0x400e85c8 as *mut u32, 0);
    pub const DAISY_LPI2C4_LPI2C_SDA_SELECT_GPIO_DISP_B2_13: Daisy =
        Daisy::new(0x400e85c8 as *mut u32, 1);
    pub const DAISY_LPI2C5_IPP_IND_LPI2C_SCL_SELECT_GPIO_LPSR_05: Daisy =
        Daisy::new(0x40c08084 as *mut u32, 0);
    pub const DAISY_LPI2C5_IPP_IND_LPI2C_SCL_SELECT_GPIO_LPSR_09: Daisy =
        Daisy::new(0x40c08084 as *mut u32, 1);
    pub const DAISY_LPI2C5_IPP_IND_LPI2C_SDA_SELECT_GPIO_LPSR_04: Daisy =
        Daisy::new(0x40c08088 as *mut u32, 0);
    pub const DAISY_LPI2C5_IPP_IND_LPI2C_SDA_SELECT_GPIO_LPSR_08: Daisy =
        Daisy::new(0x40c08088 as *mut u32, 1);
    pub const DAISY_LPI2C6_IPP_IND_LPI2C_SCL_SELECT_GPIO_LPSR_07: Daisy =
        Daisy::new(0x40c0808c as *mut u32, 0);
    pub const DAISY_LPI2C6_IPP_IND_LPI2C_SCL_SELECT_GPIO_LPSR_11: Daisy =
        Daisy::new(0x40c0808c as *mut u32, 1);
    pub const DAISY_LPI2C6_IPP_IND_LPI2C_SDA_SELECT_GPIO_LPSR_06: Daisy =
        Daisy::new(0x40c08090 as *mut u32, 0);
    pub const DAISY_LPI2C6_IPP_IND_LPI2C_SDA_SELECT_GPIO_LPSR_10: Daisy =
        Daisy::new(0x40c08090 as *mut u32, 1);
}

use daisy::*;
