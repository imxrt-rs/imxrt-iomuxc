//! UART pin implementations

use super::pads::{gpio_ad_b0::*, gpio_ad_b1::*, gpio_emc::*, gpio_sd_b0::*, gpio_sd_b1::*};
use crate::{
    consts::*,
    lpuart::{Pin, Rx, Tx},
    Daisy,
};

//
// UART1
//
uart!(module: U1, alt: 2, pad: GPIO_AD_B0_07, direction: Rx, daisy: None);
uart!(module: U1, alt: 2, pad: GPIO_AD_B0_06, direction: Tx, daisy: None);

//
// UART2
//
uart!(module: U2, alt: 2, pad: GPIO_AD_B1_09, direction: Rx, daisy: Some(DAISY_LPUART2_RX_GPIO_AD_B1_09));
uart!(module: U2, alt: 2, pad: GPIO_EMC_23, direction: Rx, daisy: Some(DAISY_LPUART2_RX_GPIO_EMC_23));
uart!(module: U2, alt: 2, pad: GPIO_AD_B1_08, direction: Tx, daisy: Some(DAISY_LPUART2_TX_GPIO_AD_B1_08));
uart!(module: U2, alt: 2, pad: GPIO_EMC_22, direction: Tx, daisy: Some(DAISY_LPUART2_TX_GPIO_EMC_22));

//
// UART3
//
uart!(module: U3, alt: 2, pad: GPIO_AD_B0_15, direction: Rx, daisy: Some(DAISY_LPUART3_RX_GPIO_AD_B0_15));
uart!(module: U3, alt: 2, pad: GPIO_EMC_07, direction: Rx, daisy: Some(DAISY_LPUART3_RX_GPIO_EMC_07));
uart!(module: U3, alt: 2, pad: GPIO_AD_B0_14, direction: Tx, daisy: Some(DAISY_LPUART3_TX_GPIO_AD_B0_14));
uart!(module: U3, alt: 2, pad: GPIO_EMC_06, direction: Rx, daisy: Some(DAISY_LPUART3_TX_GPIO_EMC_06));

//
// UART4
//
uart!(module: U4, alt: 2, pad: GPIO_AD_B1_11, direction: Rx, daisy: Some(DAISY_LPUART4_RX_GPIO_AD_B1_11));
uart!(module: U4, alt: 2, pad: GPIO_EMC_03, direction: Rx, daisy: Some(DAISY_LPUART4_RX_GPIO_EMC_03));
uart!(module: U4, alt: 2, pad: GPIO_EMC_33, direction: Rx, daisy: Some(DAISY_LPUART4_RX_GPIO_EMC_33));
uart!(module: U4, alt: 2, pad: GPIO_AD_B1_10, direction: Tx, daisy: Some(DAISY_LPUART4_TX_GPIO_AD_B1_10));
uart!(module: U4, alt: 2, pad: GPIO_EMC_02, direction: Tx, daisy: Some(DAISY_LPUART4_TX_GPIO_EMC_02));
uart!(module: U4, alt: 2, pad: GPIO_EMC_32, direction: Tx, daisy: Some(DAISY_LPUART4_TX_GPIO_EMC_32));

//
// UART5
//
uart!(module: U5, alt: 2, pad: GPIO_AD_B0_11, direction: Rx, daisy: Some(DAISY_LPUART5_RX_GPIO_AD_B0_11));
uart!(module: U5, alt: 2, pad: GPIO_EMC_39, direction: Rx, daisy: Some(DAISY_LPUART5_RX_GPIO_EMC_39));
uart!(module: U5, alt: 2, pad: GPIO_AD_B0_10, direction: Tx, daisy: Some(DAISY_LPUART5_TX_GPIO_AD_B0_10));
uart!(module: U5, alt: 2, pad: GPIO_EMC_38, direction: Tx, daisy: Some(DAISY_LPUART5_TX_GPIO_EMC_38));

//
// UART6
//
uart!(module: U6, alt: 2, pad: GPIO_EMC_13, direction: Rx, daisy: Some(DAISY_LPUART6_RX_GPIO_EMC_13));
uart!(module: U6, alt: 2, pad: GPIO_SD_B1_01, direction: Rx, daisy: Some(DAISY_LPUART6_RX_GPIO_SD_B1_01));
uart!(module: U6, alt: 2, pad: GPIO_EMC_12, direction: Tx, daisy: Some(DAISY_LPUART6_TX_GPIO_EMC_12));
uart!(module: U6, alt: 2, pad: GPIO_SD_B1_00, direction: Tx, daisy: Some(DAISY_LPUART6_TX_GPIO_SD_B1_00));

//
// UART7
//
uart!(module: U7, alt: 2, pad: GPIO_EMC_35, direction: Rx, daisy: Some(DAISY_LPUART7_RX_GPIO_EMC_35));
uart!(module: U7, alt: 2, pad: GPIO_SD_B0_05, direction: Rx, daisy: Some(DAISY_LPUART7_RX_GPIO_SD_B0_05));
uart!(module: U7, alt: 2, pad: GPIO_EMC_34, direction: Tx, daisy: Some(DAISY_LPUART7_TX_GPIO_EMC_34));
uart!(module: U7, alt: 2, pad: GPIO_SD_B0_04, direction: Tx, daisy: Some(DAISY_LPUART7_TX_GPIO_SD_B0_04));

//
// UART8
//
uart!(module: U8, alt: 2, pad: GPIO_EMC_27, direction: Rx, daisy: Some(DAISY_LPUART8_RX_GPIO_EMC_27));
uart!(module: U8, alt: 2, pad: GPIO_SD_B1_03, direction: Rx, daisy: Some(DAISY_LPUART8_RX_GPIO_SD_B1_03));
uart!(module: U8, alt: 2, pad: GPIO_EMC_26, direction: Tx, daisy: Some(DAISY_LPUART8_TX_GPIO_EMC_26));
uart!(module: U8, alt: 2, pad: GPIO_SD_B1_02, direction: Tx, daisy: Some(DAISY_LPUART8_TX_GPIO_SD_B1_02));

/// Auto-generated Daisy constants
mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_LPUART2_CTS_B_GPIO_AD_B1_06: Daisy = Daisy::new(0x401f83cc as *mut u32, 0);
    pub const DAISY_LPUART2_CTS_B_GPIO_EMC_20: Daisy = Daisy::new(0x401f83cc as *mut u32, 1);
    pub const DAISY_LPUART2_RX_GPIO_AD_B1_09: Daisy = Daisy::new(0x401f83d0 as *mut u32, 0);
    pub const DAISY_LPUART2_RX_GPIO_EMC_23: Daisy = Daisy::new(0x401f83d0 as *mut u32, 1);
    pub const DAISY_LPUART2_TX_GPIO_AD_B1_08: Daisy = Daisy::new(0x401f83d4 as *mut u32, 0);
    pub const DAISY_LPUART2_TX_GPIO_EMC_22: Daisy = Daisy::new(0x401f83d4 as *mut u32, 1);
    pub const DAISY_LPUART3_RX_GPIO_EMC_07: Daisy = Daisy::new(0x401f83d8 as *mut u32, 0);
    pub const DAISY_LPUART3_RX_GPIO_AD_B0_15: Daisy = Daisy::new(0x401f83d8 as *mut u32, 1);
    pub const DAISY_LPUART3_TX_GPIO_EMC_06: Daisy = Daisy::new(0x401f83dc as *mut u32, 0);
    pub const DAISY_LPUART3_TX_GPIO_AD_B0_14: Daisy = Daisy::new(0x401f83dc as *mut u32, 1);
    pub const DAISY_LPUART4_CTS_B_GPIO_EMC_00: Daisy = Daisy::new(0x401f83e0 as *mut u32, 0);
    pub const DAISY_LPUART4_CTS_B_GPIO_EMC_30: Daisy = Daisy::new(0x401f83e0 as *mut u32, 1);
    pub const DAISY_LPUART4_RX_GPIO_EMC_03: Daisy = Daisy::new(0x401f83e4 as *mut u32, 0);
    pub const DAISY_LPUART4_RX_GPIO_AD_B1_11: Daisy = Daisy::new(0x401f83e4 as *mut u32, 1);
    pub const DAISY_LPUART4_RX_GPIO_EMC_33: Daisy = Daisy::new(0x401f83e4 as *mut u32, 2);
    pub const DAISY_LPUART4_TX_GPIO_EMC_02: Daisy = Daisy::new(0x401f83e8 as *mut u32, 0);
    pub const DAISY_LPUART4_TX_GPIO_AD_B1_10: Daisy = Daisy::new(0x401f83e8 as *mut u32, 1);
    pub const DAISY_LPUART4_TX_GPIO_EMC_32: Daisy = Daisy::new(0x401f83e8 as *mut u32, 2);
    pub const DAISY_LPUART5_RX_GPIO_AD_B0_11: Daisy = Daisy::new(0x401f83ec as *mut u32, 0);
    pub const DAISY_LPUART5_RX_GPIO_EMC_39: Daisy = Daisy::new(0x401f83ec as *mut u32, 1);
    pub const DAISY_LPUART5_TX_GPIO_AD_B0_10: Daisy = Daisy::new(0x401f83f0 as *mut u32, 0);
    pub const DAISY_LPUART5_TX_GPIO_EMC_38: Daisy = Daisy::new(0x401f83f0 as *mut u32, 1);
    pub const DAISY_LPUART6_RX_GPIO_EMC_13: Daisy = Daisy::new(0x401f83f4 as *mut u32, 0);
    pub const DAISY_LPUART6_RX_GPIO_SD_B1_01: Daisy = Daisy::new(0x401f83f4 as *mut u32, 1);
    pub const DAISY_LPUART6_TX_GPIO_EMC_12: Daisy = Daisy::new(0x401f83f8 as *mut u32, 0);
    pub const DAISY_LPUART6_TX_GPIO_SD_B1_00: Daisy = Daisy::new(0x401f83f8 as *mut u32, 1);
    pub const DAISY_LPUART7_RX_GPIO_SD_B0_05: Daisy = Daisy::new(0x401f83fc as *mut u32, 0);
    pub const DAISY_LPUART7_RX_GPIO_EMC_35: Daisy = Daisy::new(0x401f83fc as *mut u32, 1);
    pub const DAISY_LPUART7_TX_GPIO_SD_B0_04: Daisy = Daisy::new(0x401f8400 as *mut u32, 0);
    pub const DAISY_LPUART7_TX_GPIO_EMC_34: Daisy = Daisy::new(0x401f8400 as *mut u32, 1);
    pub const DAISY_LPUART8_RX_GPIO_SD_B1_03: Daisy = Daisy::new(0x401f8404 as *mut u32, 0);
    pub const DAISY_LPUART8_RX_GPIO_EMC_27: Daisy = Daisy::new(0x401f8404 as *mut u32, 1);
    pub const DAISY_LPUART8_TX_GPIO_SD_B1_02: Daisy = Daisy::new(0x401f8408 as *mut u32, 0);
    pub const DAISY_LPUART8_TX_GPIO_EMC_26: Daisy = Daisy::new(0x401f8408 as *mut u32, 1);
}
use daisy::*;
