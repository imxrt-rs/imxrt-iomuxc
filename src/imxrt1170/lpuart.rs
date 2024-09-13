use super::pads::gpio_ad::*;
use super::pads::gpio_disp_b1::*;
use crate::{
    consts::*,
    lpuart::{Pin, Rx, Tx},
    Daisy,
};

//
// UART1
//
uart!(module: U1, alt: 0, pad: GPIO_AD_24, direction: Tx, daisy: Some(DAISY_LPUART1_LPUART_TXD_SELECT_GPIO_AD_24));
uart!(module: U1, alt: 0, pad: GPIO_AD_25, direction: Rx, daisy: Some(DAISY_LPUART1_LPUART_RXD_SELECT_GPIO_AD_25));
uart!(module: U1, alt: 9, pad: GPIO_DISP_B1_02, direction: Tx, daisy: Some(DAISY_LPUART1_LPUART_TXD_SELECT_GPIO_DISP_B1_02));
uart!(module: U1, alt: 9, pad: GPIO_DISP_B1_03, direction: Rx, daisy: Some(DAISY_LPUART1_LPUART_RXD_SELECT_GPIO_DISP_B1_03));

mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_LPUART1_LPUART_RXD_SELECT_GPIO_AD_25: Daisy =
        Daisy::new(0x400e861c as *mut u32, 0);
    pub const DAISY_LPUART1_LPUART_RXD_SELECT_GPIO_DISP_B1_03: Daisy =
        Daisy::new(0x400e861c as *mut u32, 1);
    pub const DAISY_LPUART1_LPUART_RXD_SELECT_GPIO_DISP_B2_09: Daisy =
        Daisy::new(0x400e861c as *mut u32, 2);
    pub const DAISY_LPUART1_LPUART_TXD_SELECT_GPIO_AD_24: Daisy =
        Daisy::new(0x400e8620 as *mut u32, 0);
    pub const DAISY_LPUART1_LPUART_TXD_SELECT_GPIO_DISP_B1_02: Daisy =
        Daisy::new(0x400e8620 as *mut u32, 1);
    pub const DAISY_LPUART1_LPUART_TXD_SELECT_GPIO_DISP_B2_08: Daisy =
        Daisy::new(0x400e8620 as *mut u32, 2);
    pub const DAISY_LPUART10_LPUART_RXD_SELECT_GPIO_AD_16: Daisy =
        Daisy::new(0x400e8624 as *mut u32, 0);
    pub const DAISY_LPUART10_LPUART_RXD_SELECT_GPIO_AD_33: Daisy =
        Daisy::new(0x400e8624 as *mut u32, 1);
    pub const DAISY_LPUART10_LPUART_TXD_SELECT_GPIO_AD_15: Daisy =
        Daisy::new(0x400e8628 as *mut u32, 0);
    pub const DAISY_LPUART10_LPUART_TXD_SELECT_GPIO_AD_32: Daisy =
        Daisy::new(0x400e8628 as *mut u32, 1);
    pub const DAISY_LPUART7_LPUART_RXD_SELECT_GPIO_AD_01: Daisy =
        Daisy::new(0x400e862c as *mut u32, 0);
    pub const DAISY_LPUART7_LPUART_RXD_SELECT_GPIO_DISP_B2_07: Daisy =
        Daisy::new(0x400e862c as *mut u32, 1);
    pub const DAISY_LPUART7_LPUART_TXD_SELECT_GPIO_AD_00: Daisy =
        Daisy::new(0x400e8630 as *mut u32, 0);
    pub const DAISY_LPUART7_LPUART_TXD_SELECT_GPIO_DISP_B2_06: Daisy =
        Daisy::new(0x400e8630 as *mut u32, 1);
    pub const DAISY_LPUART8_LPUART_RXD_SELECT_GPIO_AD_03: Daisy =
        Daisy::new(0x400e8634 as *mut u32, 0);
    pub const DAISY_LPUART8_LPUART_RXD_SELECT_GPIO_DISP_B2_09: Daisy =
        Daisy::new(0x400e8634 as *mut u32, 1);
    pub const DAISY_LPUART8_LPUART_TXD_SELECT_GPIO_AD_02: Daisy =
        Daisy::new(0x400e8638 as *mut u32, 0);
    pub const DAISY_LPUART8_LPUART_TXD_SELECT_GPIO_DISP_B2_08: Daisy =
        Daisy::new(0x400e8638 as *mut u32, 1);
}

use daisy::*;
