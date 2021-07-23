//! I2C pin implementations

use super::pads::{ad_b0::*, ad_b1::*, sd_b0::*};
use crate::{
    consts::*,
    lpi2c::{Pin, Scl, Sda},
    Daisy,
};

//
// I2C1
//
i2c!(module: U1, alt: 3, pad: AD_B1_00, signal: Scl, daisy: DAISY_LPI2C1_SCL_AD_B1_00);
i2c!(module: U1, alt: 3, pad: AD_B1_01, signal: Sda, daisy: DAISY_LPI2C1_SDA_AD_B1_01);

//
// I2C2
//

// TODO

//
// I2C3
//
i2c!(module: U3, alt: 1, pad: AD_B1_07, signal: Scl, daisy: DAISY_LPI2C3_SCL_AD_B1_07);
i2c!(module: U3, alt: 1, pad: AD_B1_06, signal: Sda, daisy: DAISY_LPI2C3_SDA_AD_B1_06);
i2c!(module: U3, alt: 2, pad: SD_B0_00, signal: Scl, daisy: DAISY_LPI2C3_SCL_SD_B0_00);
i2c!(module: U3, alt: 2, pad: SD_B0_01, signal: Sda, daisy: DAISY_LPI2C3_SDA_SD_B0_01);

//
// I2C4
//
i2c!(module: U4, alt: 0, pad: AD_B0_12, signal: Scl, daisy: DAISY_LPI2C4_SCL_AD_B0_12);
i2c!(module: U4, alt: 0, pad: AD_B0_13, signal: Sda, daisy: DAISY_LPI2C4_SDA_AD_B0_13);

/// Auto-generated Daisy constants
mod daisy {
    #![allow(unused)]

    use super::Daisy;

    pub const DAISY_LPI2C1_SCL_SD_B1_04: Daisy = Daisy::new(0x401f84cc as *mut u32, 0);
    pub const DAISY_LPI2C1_SCL_AD_B1_00: Daisy = Daisy::new(0x401f84cc as *mut u32, 1);
    pub const DAISY_LPI2C1_SDA_SD_B1_05: Daisy = Daisy::new(0x401f84d0 as *mut u32, 0);
    pub const DAISY_LPI2C1_SDA_AD_B1_01: Daisy = Daisy::new(0x401f84d0 as *mut u32, 1);
    pub const DAISY_LPI2C2_SCL_SD_B1_11: Daisy = Daisy::new(0x401f84d4 as *mut u32, 0);
    pub const DAISY_LPI2C2_SCL_B0_04: Daisy = Daisy::new(0x401f84d4 as *mut u32, 1);
    pub const DAISY_LPI2C2_SDA_SD_B1_10: Daisy = Daisy::new(0x401f84d8 as *mut u32, 0);
    pub const DAISY_LPI2C2_SDA_B0_05: Daisy = Daisy::new(0x401f84d8 as *mut u32, 1);
    pub const DAISY_LPI2C3_SCL_EMC_22: Daisy = Daisy::new(0x401f84dc as *mut u32, 0);
    pub const DAISY_LPI2C3_SCL_SD_B0_00: Daisy = Daisy::new(0x401f84dc as *mut u32, 1);
    pub const DAISY_LPI2C3_SCL_AD_B1_07: Daisy = Daisy::new(0x401f84dc as *mut u32, 2);
    pub const DAISY_LPI2C3_SDA_EMC_21: Daisy = Daisy::new(0x401f84e0 as *mut u32, 0);
    pub const DAISY_LPI2C3_SDA_SD_B0_01: Daisy = Daisy::new(0x401f84e0 as *mut u32, 1);
    pub const DAISY_LPI2C3_SDA_AD_B1_06: Daisy = Daisy::new(0x401f84e0 as *mut u32, 2);
    pub const DAISY_LPI2C4_SCL_EMC_12: Daisy = Daisy::new(0x401f84e4 as *mut u32, 0);
    pub const DAISY_LPI2C4_SCL_AD_B0_12: Daisy = Daisy::new(0x401f84e4 as *mut u32, 1);
    pub const DAISY_LPI2C4_SDA_EMC_11: Daisy = Daisy::new(0x401f84e8 as *mut u32, 0);
    pub const DAISY_LPI2C4_SDA_AD_B0_13: Daisy = Daisy::new(0x401f84e8 as *mut u32, 1);
}

use daisy::*;
