//! Can pin implementation

use super::pads::{ad_b0::*, ad_b1::*, b0::*, b1::*, emc::*, sd_b1::*};

use crate::{
    can::{Pin, Rx, Tx},
    consts::*,
    Daisy,
};

//
// CAN1
//
can!(module: U1, alt: 2, pad: AD_B1_08, signal: Tx, daisy: None);
can!(module: U1, alt: 2, pad: B0_02, signal: Tx, daisy: None);
can!(module: U1, alt: 3, pad: EMC_17, signal: Tx, daisy: None);
can!(module: U1, alt: 4, pad: SD_B1_02, signal: Tx, daisy: None);
can!(module: U1, alt: 2, pad: AD_B1_09, signal: Rx, daisy: Some(DAISY_CAN1_RX_GPIO_AD_B1_09));
can!(module: U1, alt: 2, pad: B0_03, signal: Rx, daisy: Some(DAISY_CAN1_RX_GPIO_B0_03));
can!(module: U1, alt: 3, pad: EMC_18, signal: Rx, daisy: Some(DAISY_CAN1_RX_GPIO_EMC_18));
can!(module: U1, alt: 4, pad: SD_B1_03, signal: Rx, daisy: Some(DAISY_CAN1_RX_GPIO_SD_B1_03));

//
// CAN2
//
can!(module: U2, alt: 0, pad: AD_B0_02, signal: Tx, daisy: None);
can!(module: U2, alt: 3, pad: EMC_09, signal: Tx, daisy: None);
can!(module: U2, alt: 6, pad: B1_08, signal: Tx, daisy: None);
can!(module: U2, alt: 6, pad: AD_B0_14, signal: Tx, daisy: None);
can!(module: U2, alt: 0, pad: AD_B0_03, signal: Rx, daisy: Some(DAISY_CAN2_RX_GPIO_AD_B0_03));
can!(module: U2, alt: 3, pad: EMC_10, signal: Rx, daisy: Some(DAISY_CAN2_RX_GPIO_EMC_10));
can!(module: U2, alt: 6, pad: AD_B0_15, signal: Rx, daisy: Some(DAISY_CAN2_RX_GPIO_AD_B0_15));
can!(module: U2, alt: 6, pad: B1_09, signal: Rx, daisy: Some(DAISY_CAN2_RX_GPIO_B1_09));

/// Auto-generated DAISY values
mod daisy {
    #![allow(unused)]

    use super::Daisy;

    pub const DAISY_CAN1_RX_GPIO_SD_B1_03: Daisy = Daisy::new(0x401F_844C as *mut u32, 0);
    pub const DAISY_CAN1_RX_GPIO_EMC_18: Daisy = Daisy::new(0x401F_844C as *mut u32, 1);
    pub const DAISY_CAN1_RX_GPIO_AD_B1_09: Daisy = Daisy::new(0x401F_844C as *mut u32, 2);
    pub const DAISY_CAN1_RX_GPIO_B0_03: Daisy = Daisy::new(0x401F_844C as *mut u32, 3);

    pub const DAISY_CAN2_RX_GPIO_EMC_10: Daisy = Daisy::new(0x401F_8450 as *mut u32, 0);
    pub const DAISY_CAN2_RX_GPIO_AD_B0_03: Daisy = Daisy::new(0x401F_8450 as *mut u32, 1);
    pub const DAISY_CAN2_RX_GPIO_AD_B0_15: Daisy = Daisy::new(0x401F_8450 as *mut u32, 2);
    pub const DAISY_CAN2_RX_GPIO_B1_09: Daisy = Daisy::new(0x401F_8450 as *mut u32, 3);
}

use daisy::*;
