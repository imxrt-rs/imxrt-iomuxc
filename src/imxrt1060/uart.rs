//! UART pin implementations

use super::pads::{ad_b0::*, ad_b1::*, b1::*, emc::*};
use crate::{
    consts::*,
    uart::{Pin, RX, TX},
    Daisy,
};

//
// UART1
//

uart!(module: U1, alt: 2, pad: AD_B0_13, direction: RX, daisy: None);

uart!(module: U1, alt: 2, pad: AD_B0_12, direction: TX, daisy: None);

//
// UART2
//

uart!(module: U2, alt: 2, pad: AD_B1_03, direction: RX, daisy: Some(DAISY_LPUART2_RX_AD_B1_03));

uart!(module: U2, alt: 2, pad: AD_B1_02, direction: TX, daisy: Some(DAISY_LPUART2_TX_AD_B1_02));

//
// UART3
//

uart!(module: U3, alt: 2, pad: AD_B1_07, direction: RX, daisy: Some(DAISY_LPUART3_RX_AD_B1_07));

uart!(module: U3, alt: 2, pad: AD_B1_06, direction: TX, daisy: Some(DAISY_LPUART3_TX_AD_B1_06));

//
// UART4
//

uart!(module: U4, alt: 2, pad: B1_01, direction: RX, daisy: Some(DAISY_LPUART4_RX_B1_01));

uart!(module: U4, alt: 2, pad: B1_00, direction: TX, daisy: Some(DAISY_LPUART4_TX_B1_00));

//
// UART5
//

// TODO

//
// UART6
//

uart!(module: U6, alt: 2, pad: AD_B0_03, direction: RX, daisy: Some(DAISY_LPUART6_RX_AD_B0_03));

uart!(module: U6, alt: 2, pad: AD_B0_02, direction: TX, daisy: Some(DAISY_LPUART6_TX_AD_B0_02));

//
// UART7
//

uart!(module: U7, alt: 2, pad: EMC_32, direction: RX, daisy: Some(DAISY_LPUART7_RX_EMC_32));

uart!(module: U7, alt: 2, pad: EMC_31, direction: TX, daisy: Some(DAISY_LPUART7_TX_EMC_31));

//
// UART8
//

uart!(module: U8, alt: 2, pad: AD_B1_11, direction: RX, daisy: Some(DAISY_LPUART8_RX_AD_B1_11));

uart!(module: U8, alt: 2, pad: AD_B1_10, direction: TX, daisy: Some(DAISY_LPUART8_TX_AD_B1_10));

/// Auto-generated Daisy constants
mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_LPUART2_RX_SD_B1_10: Daisy = Daisy::new(0x401f852c as *mut u32, 0);
    pub const DAISY_LPUART2_RX_AD_B1_03: Daisy = Daisy::new(0x401f852c as *mut u32, 1);
    pub const DAISY_LPUART2_TX_SD_B1_11: Daisy = Daisy::new(0x401f8530 as *mut u32, 0);
    pub const DAISY_LPUART2_TX_AD_B1_02: Daisy = Daisy::new(0x401f8530 as *mut u32, 1);
    pub const DAISY_LPUART3_CTS_B_EMC_15: Daisy = Daisy::new(0x401f8534 as *mut u32, 0);
    pub const DAISY_LPUART3_CTS_B_AD_B1_04: Daisy = Daisy::new(0x401f8534 as *mut u32, 1);
    pub const DAISY_LPUART3_RX_AD_B1_07: Daisy = Daisy::new(0x401f8538 as *mut u32, 0);
    pub const DAISY_LPUART3_RX_EMC_14: Daisy = Daisy::new(0x401f8538 as *mut u32, 1);
    pub const DAISY_LPUART3_RX_B0_09: Daisy = Daisy::new(0x401f8538 as *mut u32, 2);
    pub const DAISY_LPUART3_TX_AD_B1_06: Daisy = Daisy::new(0x401f853c as *mut u32, 0);
    pub const DAISY_LPUART3_TX_EMC_13: Daisy = Daisy::new(0x401f853c as *mut u32, 1);
    pub const DAISY_LPUART3_TX_B0_08: Daisy = Daisy::new(0x401f853c as *mut u32, 2);
    pub const DAISY_LPUART4_RX_SD_B1_01: Daisy = Daisy::new(0x401f8540 as *mut u32, 0);
    pub const DAISY_LPUART4_RX_EMC_20: Daisy = Daisy::new(0x401f8540 as *mut u32, 1);
    pub const DAISY_LPUART4_RX_B1_01: Daisy = Daisy::new(0x401f8540 as *mut u32, 2);
    pub const DAISY_LPUART4_TX_SD_B1_00: Daisy = Daisy::new(0x401f8544 as *mut u32, 0);
    pub const DAISY_LPUART4_TX_EMC_19: Daisy = Daisy::new(0x401f8544 as *mut u32, 1);
    pub const DAISY_LPUART4_TX_B1_00: Daisy = Daisy::new(0x401f8544 as *mut u32, 2);
    pub const DAISY_LPUART5_RX_EMC_24: Daisy = Daisy::new(0x401f8548 as *mut u32, 0);
    pub const DAISY_LPUART5_RX_B1_13: Daisy = Daisy::new(0x401f8548 as *mut u32, 1);
    pub const DAISY_LPUART5_TX_EMC_23: Daisy = Daisy::new(0x401f854c as *mut u32, 0);
    pub const DAISY_LPUART5_TX_B1_12: Daisy = Daisy::new(0x401f854c as *mut u32, 1);
    pub const DAISY_LPUART6_RX_EMC_26: Daisy = Daisy::new(0x401f8550 as *mut u32, 0);
    pub const DAISY_LPUART6_RX_AD_B0_03: Daisy = Daisy::new(0x401f8550 as *mut u32, 1);
    pub const DAISY_LPUART6_TX_EMC_25: Daisy = Daisy::new(0x401f8554 as *mut u32, 0);
    pub const DAISY_LPUART6_TX_AD_B0_02: Daisy = Daisy::new(0x401f8554 as *mut u32, 1);
    pub const DAISY_LPUART7_RX_SD_B1_09: Daisy = Daisy::new(0x401f8558 as *mut u32, 0);
    pub const DAISY_LPUART7_RX_EMC_32: Daisy = Daisy::new(0x401f8558 as *mut u32, 1);
    pub const DAISY_LPUART7_TX_SD_B1_08: Daisy = Daisy::new(0x401f855c as *mut u32, 0);
    pub const DAISY_LPUART7_TX_EMC_31: Daisy = Daisy::new(0x401f855c as *mut u32, 1);
    pub const DAISY_LPUART8_RX_SD_B0_05: Daisy = Daisy::new(0x401f8560 as *mut u32, 0);
    pub const DAISY_LPUART8_RX_AD_B1_11: Daisy = Daisy::new(0x401f8560 as *mut u32, 1);
    pub const DAISY_LPUART8_RX_EMC_39: Daisy = Daisy::new(0x401f8560 as *mut u32, 2);
    pub const DAISY_LPUART8_TX_SD_B0_04: Daisy = Daisy::new(0x401f8564 as *mut u32, 0);
    pub const DAISY_LPUART8_TX_AD_B1_10: Daisy = Daisy::new(0x401f8564 as *mut u32, 1);
    pub const DAISY_LPUART8_TX_EMC_38: Daisy = Daisy::new(0x401f8564 as *mut u32, 2);
}

use daisy::*;
