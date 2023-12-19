//! SAI / I2S pin implementation

use super::{gpio_ad_b0::*, gpio_ad_b1::*, gpio_emc::*, gpio_sd_b0::*, gpio_sd_b1::*};
use crate::{consts::*, sai::*, Daisy};

/// SAI1 multiplexed TX_DATA1 / RX_DATA3 pin
///
/// Implements both `RxDataSignal` and `TxDataSignal`.
pub enum TxData1RxData3 {}

impl Signal for TxData1RxData3 {}
impl TxDataSignal for TxData1RxData3 {
    type Index = U1;
}
impl RxDataSignal for TxData1RxData3 {
    type Index = U3;
}
impl private::Sealed for TxData1RxData3 {}

/// SAI1 multiplexed TX_DATA2 / RX_DATA2 pin
///
/// Implements both `RxDataSignal` and `TxDataSignal`.
pub enum TxData2RxData2 {}

impl Signal for TxData2RxData2 {}
impl TxDataSignal for TxData2RxData2 {
    type Index = U2;
}
impl RxDataSignal for TxData2RxData2 {
    type Index = U2;
}
impl private::Sealed for TxData2RxData2 {}

/// SAI1 multiplexed TX_DATA3 / RX_DATA1 pin
///
/// Implements both `RxDataSignal` and `TxDataSignal`.
pub enum TxData3RxData1 {}

impl Signal for TxData3RxData1 {}
impl TxDataSignal for TxData3RxData1 {
    type Index = U3;
}
impl RxDataSignal for TxData3RxData1 {
    type Index = U1;
}
impl private::Sealed for TxData3RxData1 {}

//
// SAI1
//

// MCLK
sai!(module: U1, alt: 2, pad: GPIO_SD_B0_00, signal: Mclk, daisy: Some(DAISY_SAI1_MCLK_GPIO_SD_B0_00));
sai!(module: U1, alt: 3, pad: GPIO_EMC_20, signal: Mclk, daisy: Some(DAISY_SAI1_MCLK_GPIO_EMC_20));
sai!(module: U1, alt: 3, pad: GPIO_AD_B0_03, signal: Mclk, daisy: Some(DAISY_SAI1_MCLK_GPIO_AD_B0_03));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_00, signal: Mclk, daisy: Some(DAISY_SAI1_MCLK_GPIO_AD_B1_00));

// RX_BCLK
sai!(module: U1, alt: 3, pad: GPIO_EMC_14, signal: RxBclk, daisy: Some(DAISY_SAI1_RX_BCLK_GPIO_EMC_14));
sai!(module: U1, alt: 3, pad: GPIO_EMC_19, signal: RxBclk, daisy: Some(DAISY_SAI1_RX_BCLK_GPIO_EMC_19));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_06, signal: RxBclk, daisy: Some(DAISY_SAI1_RX_BCLK_GPIO_AD_B1_06));

// RX_DATA0
sai!(module: U1, alt: 3, pad: GPIO_EMC_13, signal: RxData, daisy: Some(DAISY_SAI1_RX_DATA0_GPIO_EMC_13));
sai!(module: U1, alt: 3, pad: GPIO_EMC_21, signal: RxData, daisy: Some(DAISY_SAI1_RX_DATA0_GPIO_EMC_21));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_05, signal: RxData, daisy: Some(DAISY_SAI1_RX_DATA0_GPIO_AD_B1_05));

// RX_SYNC
sai!(module: U1, alt: 3, pad: GPIO_EMC_15, signal: RxSync, daisy: Some(DAISY_SAI1_RX_SYNC_GPIO_EMC_15));
sai!(module: U1, alt: 3, pad: GPIO_EMC_18, signal: RxSync, daisy: Some(DAISY_SAI1_RX_SYNC_GPIO_EMC_18));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_04, signal: RxSync, daisy: Some(DAISY_SAI1_RX_SYNC_GPIO_AD_B1_04));

// TX_BCLK
sai!(module: U1, alt: 3, pad: GPIO_EMC_11, signal: TxBclk, daisy: Some(DAISY_SAI1_TX_BCLK_GPIO_EMC_11));
sai!(module: U1, alt: 3, pad: GPIO_EMC_26, signal: TxBclk, daisy: Some(DAISY_SAI1_TX_BCLK_GPIO_EMC_26));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_01, signal: TxBclk, daisy: Some(DAISY_SAI1_TX_BCLK_GPIO_AD_B1_01));

// TX_DATA0
sai!(module: U1, alt: 3, pad: GPIO_EMC_12, signal: TxData, daisy: None);
sai!(module: U1, alt: 3, pad: GPIO_EMC_25, signal: TxData, daisy: None);
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_03, signal: TxData, daisy: None);

// TX_DATA1 and RX_DATA3
sai!(module: U1, alt: 3, pad: GPIO_EMC_24, signal: TxData1RxData3, daisy: Some(DAISY_SAI1_RX_DATA3_GPIO_EMC_24));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_07, signal: TxData1RxData3, daisy: Some(DAISY_SAI1_RX_DATA3_GPIO_AD_B1_07));

// TX_DATA2 and RX_DATA2
sai!(module: U1, alt: 3, pad: GPIO_EMC_23, signal: TxData2RxData2, daisy: Some(DAISY_SAI1_RX_DATA2_GPIO_EMC_23));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_08, signal: TxData2RxData2, daisy: Some(DAISY_SAI1_RX_DATA2_GPIO_AD_B1_08));

// TX_DATA3 and RX_DATA1
sai!(module: U1, alt: 3, pad: GPIO_EMC_22, signal: TxData3RxData1, daisy: Some(DAISY_SAI1_RX_DATA1_GPIO_EMC_22));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_09, signal: TxData3RxData1, daisy: Some(DAISY_SAI1_RX_DATA1_GPIO_AD_B1_09));

// TX_SYNC
sai!(module: U1, alt: 3, pad: GPIO_EMC_10, signal: TxSync, daisy: Some(DAISY_SAI1_TX_SYNC_GPIO_EMC_10));
sai!(module: U1, alt: 3, pad: GPIO_EMC_27, signal: TxSync, daisy: Some(DAISY_SAI1_TX_SYNC_GPIO_EMC_27));
sai!(module: U1, alt: 3, pad: GPIO_AD_B1_02, signal: TxSync, daisy: Some(DAISY_SAI1_TX_SYNC_GPIO_AD_B1_02));

//
// SAI2
//

// MCLK
sai!(module: U2, alt: 3, pad: GPIO_EMC_16, signal: Mclk, daisy: Some(DAISY_SAI2_MCLK_GPIO_EMC_16));
sai!(module: U2, alt: 3, pad: GPIO_SD_B0_00, signal: Mclk, daisy: Some(DAISY_SAI2_MCLK_GPIO_SD_B0_00));

// RX_BCLK
sai!(module: U2, alt: 3, pad: GPIO_EMC_09, signal: RxBclk, daisy: Some(DAISY_SAI2_RX_BCLK_GPIO_EMC_09));
sai!(module: U2, alt: 3, pad: GPIO_SD_B0_02, signal: RxBclk, daisy: Some(DAISY_SAI2_RX_BCLK_GPIO_SD_B0_02));

// RX_DATA
sai!(module: U2, alt: 3, pad: GPIO_EMC_08, signal: RxData, daisy: Some(DAISY_SAI2_RX_DATA0_GPIO_EMC_08));
sai!(module: U2, alt: 3, pad: GPIO_SD_B0_03, signal: RxData, daisy: Some(DAISY_SAI2_RX_DATA0_GPIO_SD_B0_03));

// RX_SYNC
sai!(module: U2, alt: 3, pad: GPIO_EMC_07, signal: RxSync, daisy: Some(DAISY_SAI2_RX_SYNC_GPIO_EMC_07));
sai!(module: U2, alt: 3, pad: GPIO_SD_B0_01, signal: RxSync, daisy: Some(DAISY_SAI2_RX_SYNC_GPIO_SD_B0_01));

// TX_BCLK
sai!(module: U2, alt: 3, pad: GPIO_EMC_04, signal: TxBclk, daisy: Some(DAISY_SAI2_TX_BCLK_GPIO_EMC_04));
sai!(module: U2, alt: 3, pad: GPIO_SD_B0_05, signal: TxBclk, daisy: Some(DAISY_SAI2_TX_BCLK_GPIO_SD_B0_05));

// TX_DATA
sai!(module: U2, alt: 3, pad: GPIO_EMC_06, signal: TxData, daisy: None);
sai!(module: U2, alt: 3, pad: GPIO_SD_B0_04, signal: TxData, daisy: None);

// TX_SYNC
sai!(module: U2, alt: 3, pad: GPIO_EMC_05, signal: TxSync, daisy: Some(DAISY_SAI2_TX_SYNC_GPIO_EMC_05));
sai!(module: U2, alt: 3, pad: GPIO_SD_B0_06, signal: TxSync, daisy: Some(DAISY_SAI2_TX_SYNC_GPIO_SD_B0_06));

//
// SAI3
//

// MCLK
sai!(module: U3, alt: 3, pad: GPIO_EMC_17, signal: Mclk, daisy: Some(DAISY_SAI3_MCLK_GPIO_EMC_17));
sai!(module: U3, alt: 3, pad: GPIO_EMC_28, signal: Mclk, daisy: Some(DAISY_SAI3_MCLK_GPIO_EMC_28));
sai!(module: U3, alt: 3, pad: GPIO_SD_B1_05, signal: Mclk, daisy: Some(DAISY_SAI3_MCLK_GPIO_SD_B1_05));

// RX_BCLK
sai!(module: U3, alt: 3, pad: GPIO_EMC_29, signal: RxBclk, daisy: Some(DAISY_SAI3_RX_BCLK_GPIO_EMC_29));
sai!(module: U3, alt: 3, pad: GPIO_SD_B1_09, signal: RxBclk, daisy: Some(DAISY_SAI3_RX_BCLK_GPIO_SD_B1_09));

// RX_DATA
sai!(module: U3, alt: 3, pad: GPIO_EMC_31, signal: RxData, daisy: Some(DAISY_SAI3_RX_DATA0_GPIO_EMC_31));
sai!(module: U3, alt: 3, pad: GPIO_SD_B1_11, signal: RxData, daisy: Some(DAISY_SAI3_RX_DATA0_GPIO_SD_B1_11));

// RX_SYNC
sai!(module: U3, alt: 3, pad: GPIO_EMC_30, signal: RxSync, daisy: Some(DAISY_SAI3_RX_SYNC_GPIO_EMC_30));
sai!(module: U3, alt: 3, pad: GPIO_SD_B1_10, signal: RxSync, daisy: Some(DAISY_SAI3_RX_SYNC_GPIO_SD_B1_10));

// TX_BCLK
sai!(module: U3, alt: 3, pad: GPIO_EMC_33, signal: TxBclk, daisy: Some(DAISY_SAI3_TX_BCLK_GPIO_EMC_33));
sai!(module: U3, alt: 3, pad: GPIO_SD_B1_06, signal: TxBclk, daisy: Some(DAISY_SAI3_TX_BCLK_GPIO_SD_B1_06));

// TX_DATA
sai!(module: U3, alt: 3, pad: GPIO_EMC_32, signal: TxData, daisy: None);
sai!(module: U3, alt: 3, pad: GPIO_SD_B1_08, signal: TxData, daisy: None);

// TX_SYNC
sai!(module: U3, alt: 3, pad: GPIO_EMC_34, signal: TxSync, daisy: Some(DAISY_SAI3_TX_SYNC_GPIO_EMC_34));
sai!(module: U3, alt: 3, pad: GPIO_SD_B1_07, signal: TxSync, daisy: Some(DAISY_SAI3_TX_SYNC_GPIO_SD_B1_07));

mod daisy {
    use super::Daisy;

    pub const DAISY_SAI1_MCLK_GPIO_SD_B0_00: Daisy = Daisy::new(0x401f8430 as *mut u32, 0);
    pub const DAISY_SAI1_MCLK_GPIO_AD_B0_03: Daisy = Daisy::new(0x401f8430 as *mut u32, 1);
    pub const DAISY_SAI1_MCLK_GPIO_AD_B1_00: Daisy = Daisy::new(0x401f8430 as *mut u32, 2);
    pub const DAISY_SAI1_MCLK_GPIO_EMC_20: Daisy = Daisy::new(0x401f8430 as *mut u32, 3);
    pub const DAISY_SAI1_RX_BCLK_GPIO_AD_B1_06: Daisy = Daisy::new(0x401f8434 as *mut u32, 0);
    pub const DAISY_SAI1_RX_BCLK_GPIO_EMC_14: Daisy = Daisy::new(0x401f8434 as *mut u32, 1);
    pub const DAISY_SAI1_RX_BCLK_GPIO_EMC_19: Daisy = Daisy::new(0x401f8434 as *mut u32, 2);
    pub const DAISY_SAI1_RX_DATA0_GPIO_EMC_13: Daisy = Daisy::new(0x401f8438 as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA0_GPIO_AD_B1_05: Daisy = Daisy::new(0x401f8438 as *mut u32, 1);
    pub const DAISY_SAI1_RX_DATA0_GPIO_EMC_21: Daisy = Daisy::new(0x401f8438 as *mut u32, 2);
    pub const DAISY_SAI1_RX_DATA1_GPIO_AD_B1_09: Daisy = Daisy::new(0x401f843c as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA1_GPIO_EMC_22: Daisy = Daisy::new(0x401f843c as *mut u32, 1);
    pub const DAISY_SAI1_RX_DATA2_GPIO_AD_B1_08: Daisy = Daisy::new(0x401f8440 as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA2_GPIO_EMC_23: Daisy = Daisy::new(0x401f8440 as *mut u32, 1);
    pub const DAISY_SAI1_RX_DATA3_GPIO_AD_B1_07: Daisy = Daisy::new(0x401f8444 as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA3_GPIO_EMC_24: Daisy = Daisy::new(0x401f8444 as *mut u32, 1);
    pub const DAISY_SAI1_RX_SYNC_GPIO_AD_B1_04: Daisy = Daisy::new(0x401f8448 as *mut u32, 0);
    pub const DAISY_SAI1_RX_SYNC_GPIO_EMC_15: Daisy = Daisy::new(0x401f8448 as *mut u32, 1);
    pub const DAISY_SAI1_RX_SYNC_GPIO_EMC_18: Daisy = Daisy::new(0x401f8448 as *mut u32, 2);
    pub const DAISY_SAI1_TX_BCLK_GPIO_EMC_11: Daisy = Daisy::new(0x401f844c as *mut u32, 0);
    pub const DAISY_SAI1_TX_BCLK_GPIO_AD_B1_01: Daisy = Daisy::new(0x401f844c as *mut u32, 1);
    pub const DAISY_SAI1_TX_BCLK_GPIO_EMC_26: Daisy = Daisy::new(0x401f844c as *mut u32, 2);
    pub const DAISY_SAI1_TX_SYNC_GPIO_EMC_10: Daisy = Daisy::new(0x401f8450 as *mut u32, 0);
    pub const DAISY_SAI1_TX_SYNC_GPIO_AD_B1_02: Daisy = Daisy::new(0x401f8450 as *mut u32, 1);
    pub const DAISY_SAI1_TX_SYNC_GPIO_EMC_27: Daisy = Daisy::new(0x401f8450 as *mut u32, 2);
    pub const DAISY_SAI2_MCLK_GPIO_SD_B0_00: Daisy = Daisy::new(0x401f8454 as *mut u32, 0);
    pub const DAISY_SAI2_MCLK_GPIO_EMC_16: Daisy = Daisy::new(0x401f8454 as *mut u32, 1);
    pub const DAISY_SAI2_RX_BCLK_GPIO_SD_B0_02: Daisy = Daisy::new(0x401f8458 as *mut u32, 0);
    pub const DAISY_SAI2_RX_BCLK_GPIO_EMC_09: Daisy = Daisy::new(0x401f8458 as *mut u32, 1);
    pub const DAISY_SAI2_RX_DATA0_GPIO_SD_B0_03: Daisy = Daisy::new(0x401f845c as *mut u32, 0);
    pub const DAISY_SAI2_RX_DATA0_GPIO_EMC_08: Daisy = Daisy::new(0x401f845c as *mut u32, 1);
    pub const DAISY_SAI2_RX_SYNC_GPIO_SD_B0_01: Daisy = Daisy::new(0x401f8460 as *mut u32, 0);
    pub const DAISY_SAI2_RX_SYNC_GPIO_EMC_07: Daisy = Daisy::new(0x401f8460 as *mut u32, 1);
    pub const DAISY_SAI2_TX_BCLK_GPIO_SD_B0_05: Daisy = Daisy::new(0x401f8464 as *mut u32, 0);
    pub const DAISY_SAI2_TX_BCLK_GPIO_EMC_04: Daisy = Daisy::new(0x401f8464 as *mut u32, 1);
    pub const DAISY_SAI2_TX_SYNC_GPIO_SD_B0_06: Daisy = Daisy::new(0x401f8468 as *mut u32, 0);
    pub const DAISY_SAI2_TX_SYNC_GPIO_EMC_05: Daisy = Daisy::new(0x401f8468 as *mut u32, 1);
    pub const DAISY_SAI3_MCLK_GPIO_SD_B1_05: Daisy = Daisy::new(0x401f846c as *mut u32, 0);
    pub const DAISY_SAI3_MCLK_GPIO_EMC_17: Daisy = Daisy::new(0x401f846c as *mut u32, 1);
    pub const DAISY_SAI3_MCLK_GPIO_EMC_28: Daisy = Daisy::new(0x401f846c as *mut u32, 2);
    pub const DAISY_SAI3_RX_BCLK_GPIO_SD_B1_09: Daisy = Daisy::new(0x401f8470 as *mut u32, 0);
    pub const DAISY_SAI3_RX_BCLK_GPIO_EMC_29: Daisy = Daisy::new(0x401f8470 as *mut u32, 1);
    pub const DAISY_SAI3_RX_DATA0_GPIO_SD_B1_11: Daisy = Daisy::new(0x401f8474 as *mut u32, 0);
    pub const DAISY_SAI3_RX_DATA0_GPIO_EMC_31: Daisy = Daisy::new(0x401f8474 as *mut u32, 1);
    pub const DAISY_SAI3_RX_SYNC_GPIO_SD_B1_10: Daisy = Daisy::new(0x401f8478 as *mut u32, 0);
    pub const DAISY_SAI3_RX_SYNC_GPIO_EMC_30: Daisy = Daisy::new(0x401f8478 as *mut u32, 1);
    pub const DAISY_SAI3_TX_BCLK_GPIO_SD_B1_06: Daisy = Daisy::new(0x401f847c as *mut u32, 0);
    pub const DAISY_SAI3_TX_BCLK_GPIO_EMC_33: Daisy = Daisy::new(0x401f847c as *mut u32, 1);
    pub const DAISY_SAI3_TX_SYNC_GPIO_SD_B1_07: Daisy = Daisy::new(0x401f8480 as *mut u32, 0);
    pub const DAISY_SAI3_TX_SYNC_GPIO_EMC_34: Daisy = Daisy::new(0x401f8480 as *mut u32, 1);
}
use daisy::*;
