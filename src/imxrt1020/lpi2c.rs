//! I2C pin implementations

use super::pads::{gpio_ad_b0::*, gpio_ad_b1::*, gpio_emc::*, gpio_sd_b0::*, gpio_sd_b1::*};
use crate::{
    consts::*,
    lpi2c::{Pin, Scl, Sda},
    Daisy,
};

//
// I2C1
//

// SCL
i2c!(module: U1, alt: 0, pad: GPIO_AD_B1_14,  signal: Scl, daisy: DAISY_LPI2C1_SCL_GPIO_AD_B1_14);
i2c!(module: U1, alt: 6, pad: GPIO_EMC_02,    signal: Scl, daisy: DAISY_LPI2C1_SCL_GPIO_EMC_02);

// SDA
i2c!(module: U1, alt: 0, pad: GPIO_AD_B1_15,  signal: Sda, daisy: DAISY_LPI2C1_SDA_GPIO_AD_B1_15);
i2c!(module: U1, alt: 6, pad: GPIO_EMC_03,    signal: Sda, daisy: DAISY_LPI2C1_SDA_GPIO_EMC_03);

//
// I2C2
//

// SCL
i2c!(module: U2, alt: 0, pad: GPIO_AD_B1_08,  signal: Scl, daisy: DAISY_LPI2C2_SCL_GPIO_AD_B1_08);
i2c!(module: U2, alt: 2, pad: GPIO_EMC_19,    signal: Scl, daisy: DAISY_LPI2C2_SCL_GPIO_EMC_19);

// SDA
i2c!(module: U2, alt: 0, pad: GPIO_AD_B1_09,  signal: Sda, daisy: DAISY_LPI2C2_SDA_GPIO_AD_B1_09);
i2c!(module: U2, alt: 2, pad: GPIO_EMC_18,    signal: Sda, daisy: DAISY_LPI2C2_SDA_GPIO_EMC_18);

//
// I2C3
//

// SCL
i2c!(module: U3, alt: 1, pad: GPIO_AD_B0_08,  signal: Scl, daisy: DAISY_LPI2C3_SCL_GPIO_AD_B0_08);
i2c!(module: U3, alt: 4, pad: GPIO_SD_B0_00,  signal: Scl, daisy: DAISY_LPI2C3_SCL_GPIO_SD_B0_00);

// SDA
i2c!(module: U3, alt: 1, pad: GPIO_AD_B0_09,  signal: Sda, daisy: DAISY_LPI2C3_SDA_GPIO_AD_B0_09);
i2c!(module: U3, alt: 4, pad: GPIO_SD_B0_01,  signal: Sda, daisy: DAISY_LPI2C3_SDA_GPIO_SD_B0_01);

//
// I2C4
//

// SCL
i2c!(module: U4, alt: 2, pad: GPIO_EMC_11,    signal: Scl, daisy: DAISY_LPI2C4_SCL_GPIO_EMC_11);
i2c!(module: U4, alt: 3, pad: GPIO_SD_B1_02,  signal: Scl, daisy: DAISY_LPI2C4_SCL_GPIO_SD_B1_02);

// SDA
i2c!(module: U4, alt: 2, pad: GPIO_EMC_10,    signal: Sda, daisy: DAISY_LPI2C4_SDA_GPIO_EMC_10);
i2c!(module: U4, alt: 3, pad: GPIO_SD_B1_03,  signal: Sda, daisy: DAISY_LPI2C4_SDA_GPIO_SD_B1_03);

mod daisy {
    use super::Daisy;

    pub const DAISY_LPI2C1_SCL_GPIO_EMC_02: Daisy = Daisy::new(0x401f837c as *mut u32, 0);
    pub const DAISY_LPI2C1_SCL_GPIO_AD_B1_14: Daisy = Daisy::new(0x401f837c as *mut u32, 1);
    pub const DAISY_LPI2C1_SDA_GPIO_EMC_03: Daisy = Daisy::new(0x401f8380 as *mut u32, 0);
    pub const DAISY_LPI2C1_SDA_GPIO_AD_B1_15: Daisy = Daisy::new(0x401f8380 as *mut u32, 1);
    pub const DAISY_LPI2C2_SCL_GPIO_AD_B1_08: Daisy = Daisy::new(0x401f8384 as *mut u32, 0);
    pub const DAISY_LPI2C2_SCL_GPIO_EMC_19: Daisy = Daisy::new(0x401f8384 as *mut u32, 1);
    pub const DAISY_LPI2C2_SDA_GPIO_AD_B1_09: Daisy = Daisy::new(0x401f8388 as *mut u32, 0);
    pub const DAISY_LPI2C2_SDA_GPIO_EMC_18: Daisy = Daisy::new(0x401f8388 as *mut u32, 1);
    pub const DAISY_LPI2C3_SCL_GPIO_SD_B0_00: Daisy = Daisy::new(0x401f838c as *mut u32, 0);
    pub const DAISY_LPI2C3_SCL_GPIO_AD_B0_08: Daisy = Daisy::new(0x401f838c as *mut u32, 1);
    pub const DAISY_LPI2C3_SDA_GPIO_SD_B0_01: Daisy = Daisy::new(0x401f8390 as *mut u32, 0);
    pub const DAISY_LPI2C3_SDA_GPIO_AD_B0_09: Daisy = Daisy::new(0x401f8390 as *mut u32, 1);
    pub const DAISY_LPI2C4_SCL_GPIO_EMC_11: Daisy = Daisy::new(0x401f8394 as *mut u32, 0);
    pub const DAISY_LPI2C4_SCL_GPIO_SD_B1_02: Daisy = Daisy::new(0x401f8394 as *mut u32, 1);
    pub const DAISY_LPI2C4_SDA_GPIO_EMC_10: Daisy = Daisy::new(0x401f8398 as *mut u32, 0);
    pub const DAISY_LPI2C4_SDA_GPIO_SD_B1_03: Daisy = Daisy::new(0x401f8398 as *mut u32, 1);
}
use daisy::*;
