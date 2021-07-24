//! UART pin implementations

use super::pads::{gpio::*, gpio_ad::*, gpio_sd::*};
use crate::{
    consts::*,
    lpuart::{Pin, RX, TX},
    Daisy,
};

//
// UART1
//
uart!(module: U1, alt: 0, pad: GPIO_09, direction: RX, daisy: Some(DAISY_LPUART1_RXD_GPIO_09));
uart!(module: U1, alt: 2, pad: GPIO_SD_11,   direction: RX, daisy: Some(DAISY_LPUART1_RXD_GPIO_SD_11));
uart!(module: U1, alt: 0, pad: GPIO_10, direction: TX, daisy: Some(DAISY_LPUART1_TXD_GPIO_10));
uart!(module: U1, alt: 2, pad: GPIO_SD_12,   direction: TX, daisy: Some(DAISY_LPUART1_TXD_GPIO_SD_12));

//
// UART2
//
uart!(module: U2, alt: 0, pad: GPIO_13, direction: RX, daisy: Some(DAISY_LPUART2_RXD_GPIO_13));
uart!(module: U2, alt: 2, pad: GPIO_SD_09,   direction: RX, daisy: Some(DAISY_LPUART2_RXD_GPIO_SD_09));
uart!(module: U2, alt: 0, pad: GPIO_AD_00,   direction: TX, daisy: Some(DAISY_LPUART2_TXD_GPIO_AD_00));
uart!(module: U2, alt: 2, pad: GPIO_SD_10,   direction: TX, daisy: Some(DAISY_LPUART2_TXD_GPIO_SD_10));

//
// UART3
//
uart!(module: U3, alt: 0, pad: GPIO_11, direction: RX, daisy: Some(DAISY_LPUART3_RXD_GPIO_11));
uart!(module: U3, alt: 1, pad: GPIO_AD_07,   direction: RX, daisy: Some(DAISY_LPUART3_RXD_GPIO_AD_07));
uart!(module: U3, alt: 3, pad: GPIO_07, direction: RX, daisy: Some(DAISY_LPUART3_RXD_GPIO_07));
uart!(module: U3, alt: 0, pad: GPIO_12, direction: TX, daisy: Some(DAISY_LPUART3_TXD_GPIO_12));
uart!(module: U3, alt: 1, pad: GPIO_AD_08,   direction: TX, daisy: Some(DAISY_LPUART3_TXD_GPIO_AD_08));
uart!(module: U3, alt: 3, pad: GPIO_08, direction: TX, daisy: Some(DAISY_LPUART3_TXD_GPIO_08));

//
// UART4
//
uart!(module: U4, alt: 0, pad: GPIO_AD_01,   direction: RX, daisy: Some(DAISY_LPUART4_RXD_GPIO_AD_01));
uart!(module: U4, alt: 3, pad: GPIO_05, direction: RX, daisy: Some(DAISY_LPUART4_RXD_GPIO_05));
uart!(module: U4, alt: 0, pad: GPIO_AD_02,   direction: TX, daisy: Some(DAISY_LPUART4_TXD_GPIO_AD_02));
uart!(module: U4, alt: 3, pad: GPIO_06, direction: TX, daisy: Some(DAISY_LPUART4_TXD_GPIO_06));

/// Auto-generated Daisy constants
mod daisy {
    use super::Daisy;

    pub const DAISY_LPUART1_RXD_GPIO_SD_11: Daisy = Daisy::new(0x401f81f0 as *mut u32, 0);
    pub const DAISY_LPUART1_RXD_GPIO_09: Daisy = Daisy::new(0x401f81f0 as *mut u32, 1);
    pub const DAISY_LPUART1_TXD_GPIO_SD_12: Daisy = Daisy::new(0x401f81f4 as *mut u32, 0);
    pub const DAISY_LPUART1_TXD_GPIO_10: Daisy = Daisy::new(0x401f81f4 as *mut u32, 1);
    pub const DAISY_LPUART2_RXD_GPIO_SD_09: Daisy = Daisy::new(0x401f81f8 as *mut u32, 0);
    pub const DAISY_LPUART2_RXD_GPIO_13: Daisy = Daisy::new(0x401f81f8 as *mut u32, 1);
    pub const DAISY_LPUART2_TXD_GPIO_AD_00: Daisy = Daisy::new(0x401f81fc as *mut u32, 0);
    pub const DAISY_LPUART2_TXD_GPIO_SD_10: Daisy = Daisy::new(0x401f81fc as *mut u32, 1);
    pub const DAISY_LPUART3_RXD_GPIO_AD_07: Daisy = Daisy::new(0x401f8200 as *mut u32, 0);
    pub const DAISY_LPUART3_RXD_GPIO_11: Daisy = Daisy::new(0x401f8200 as *mut u32, 1);
    pub const DAISY_LPUART3_RXD_GPIO_07: Daisy = Daisy::new(0x401f8200 as *mut u32, 2);
    pub const DAISY_LPUART3_TXD_GPIO_AD_08: Daisy = Daisy::new(0x401f8204 as *mut u32, 0);
    pub const DAISY_LPUART3_TXD_GPIO_12: Daisy = Daisy::new(0x401f8204 as *mut u32, 1);
    pub const DAISY_LPUART3_TXD_GPIO_08: Daisy = Daisy::new(0x401f8204 as *mut u32, 2);
    pub const DAISY_LPUART4_RXD_GPIO_AD_01: Daisy = Daisy::new(0x401f8208 as *mut u32, 0);
    pub const DAISY_LPUART4_RXD_GPIO_05: Daisy = Daisy::new(0x401f8208 as *mut u32, 1);
    pub const DAISY_LPUART4_TXD_GPIO_AD_02: Daisy = Daisy::new(0x401f820c as *mut u32, 0);
    pub const DAISY_LPUART4_TXD_GPIO_06: Daisy = Daisy::new(0x401f820c as *mut u32, 1);
}
use daisy::*;
