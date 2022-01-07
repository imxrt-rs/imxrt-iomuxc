//! uSDHC pin implementations

use super::pads::gpio_sd_b0::*;
use crate::{
    consts::*,
    usdhc::{Clk, Cmd, Data1, Data2, Data3, Data4},
    Daisy,
};

//
// uSDHC1
//
usdhc!(module: U1, alt: 0, pad: GPIO_SD_B0_00, Signal: Cmd, daisy: DAISY_USDHC_CMD_GPIO_SD_B0_00);
usdhc!(module: U1, alt: 0, pad: GPIO_SD_B0_01, Signal: Clk, daisy: DAISY_USDHC_CLK_GPIO_SD_B0_01);
usdhc!(module: U1, alt: 0, pad: GPIO_SD_B0_02, Signal: Data0, daisy: DAISY_USDHC_DATA_0_GPIO_SD_B0_02);
usdhc!(module: U1, alt: 0, pad: GPIO_SD_B0_03, Signal: Data1, daisy: DAISY_USDHC_DATA_1_GPIO_SD_B0_03);
usdhc!(module: U1, alt: 0, pad: GPIO_SD_B0_04, Signal: Data2, daisy: DAISY_USDHC_DATA_2_GPIO_SD_B0_04);
usdhc!(module: U1, alt: 0, pad: GPIO_SD_B0_05, Signal: Data3, daisy: DAISY_USDHC_DATA_3_GPIO_SD_B0_05);

mod daisy {
    #![allow(unused)]
    use super::Daisy;

    /// TODO:
    pub const DAISY_USDHC_CMD_GPIO_SD_B0_00: Daisy = Daisy::new(0x401f81bc as *mut u32, 0);
    pub const DAISY_USDHC_CLK_GPIO_SD_B0_01: Daisy = Daisy::new(0x401f81c0 as *mut u32, 0);
    pub const DAISY_USDHC_DATA_0_GPIO_SD_B0_02: Daisy = Daisy::new(0x401f81c4 as *mut u32, 0);
    pub const DAISY_USDHC_DATA_1_GPIO_SD_B0_03: Daisy = Daisy::new(0x401f81c8 as *mut u32, 0);
    pub const DAISY_USDHC_DATA_2_GPIO_SD_B0_04: Daisy = Daisy::new(0x401f81cc as *mut u32, 0);
    pub const DAISY_USDHC_DATA_3_GPIO_SD_B0_05: Daisy = Daisy::new(0x401f81d0 as *mut u32, 0);
}

use daisy::*;
