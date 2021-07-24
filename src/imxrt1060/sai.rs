//! SAI / I2S pin implementation

use super::{gpio_ad_b0::*, gpio_ad_b1::*, gpio_b0::*, gpio_b1::*, gpio_emc::*, gpio_sd_b1::*};
use crate::{consts::*, sai::*, Daisy};

/// SAI1 multiplexed TX / RX pin
///
/// Implements both `RxDataSignal` and `TxDataSignal`.
pub enum TxData1RxData3 {}
/// SAI1 multiplexed TX / RX pin
///
/// Implements both `RxDataSignal` and `TxDataSignal`.
pub enum TxData2RxData2 {}
/// SAI1 multiplexed TX / RX pin
///
/// Implements both `RxDataSignal` and `TxDataSignal`.
pub enum TxData3RxData1 {}

impl Signal for TxData1RxData3 {}
impl TxDataSignal for TxData1RxData3 {
    type Index = U1;
}
impl RxDataSignal for TxData1RxData3 {
    type Index = U3;
}

impl Signal for TxData2RxData2 {}
impl TxDataSignal for TxData2RxData2 {
    type Index = U2;
}
impl RxDataSignal for TxData2RxData2 {
    type Index = U2;
}

impl Signal for TxData3RxData1 {}
impl TxDataSignal for TxData3RxData1 {
    type Index = U3;
}
impl RxDataSignal for TxData3RxData1 {
    type Index = U1;
}

impl private::Sealed for TxData1RxData3 {}
impl private::Sealed for TxData2RxData2 {}
impl private::Sealed for TxData3RxData1 {}

//
// SAI1
//

sai! { module: U1, alt: 3, pad: GPIO_SD_B1_08, signal: TxBclk, daisy: Some(DAISY_SAI1_TX_BCLK_GPIO_SD_B1_08) }
sai! { module: U1, alt: 3, pad: GPIO_B1_02,    signal: TxBclk, daisy: Some(DAISY_SAI1_TX_BCLK_GPIO_B1_02) }
sai! { module: U1, alt: 3, pad: GPIO_AD_B1_14, signal: TxBclk, daisy: Some(DAISY_SAI1_TX_BCLK_GPIO_AD_B1_14) }

sai! { module: U1, alt: 3, pad: GPIO_AD_B1_15, signal: TxSync, daisy: Some(DAISY_SAI1_TX_SYNC_GPIO_AD_B1_15) }
sai! { module: U1, alt: 3, pad: GPIO_B1_03,    signal: TxSync, daisy: Some(DAISY_SAI1_TX_SYNC_GPIO_B1_03) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_09, signal: TxSync, daisy: Some(DAISY_SAI1_TX_SYNC_GPIO_SD_B1_09) }

sai! { module: U1, alt: 3, pad: GPIO_B0_13,    signal: Mclk, daisy: Some(DAISY_SAI1_MCLK2_GPIO_B0_13) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_03, signal: Mclk, daisy: Some(DAISY_SAI1_MCLK2_GPIO_SD_B1_03) }
sai! { module: U1, alt: 3, pad: GPIO_AD_B1_09, signal: Mclk, daisy: Some(DAISY_SAI1_MCLK2_GPIO_AD_B1_09) }

sai! { module: U1, alt: 3, pad: GPIO_AD_B1_11, signal: RxBclk, daisy: Some(DAISY_SAI1_RX_BCLK_GPIO_AD_B1_11) }
sai! { module: U1, alt: 3, pad: GPIO_B0_15,    signal: RxBclk, daisy: Some(DAISY_SAI1_RX_BCLK_GPIO_B0_15) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_05, signal: RxBclk, daisy: Some(DAISY_SAI1_RX_BCLK_GPIO_SD_B1_05) }

sai! { module: U1, alt: 3, pad: GPIO_AD_B1_10, signal: RxSync, daisy: Some(DAISY_SAI1_RX_SYNC_GPIO_AD_B1_10) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_04, signal: RxSync, daisy: Some(DAISY_SAI1_RX_SYNC_GPIO_SD_B1_04) }
sai! { module: U1, alt: 3, pad: GPIO_B0_14,    signal: RxSync, daisy: Some(DAISY_SAI1_RX_SYNC_GPIO_B0_14) }

sai! { module: U1, alt: 3, pad: GPIO_AD_B1_13, signal: TxData, daisy: None }
sai! { module: U1, alt: 3, pad: GPIO_B1_01,    signal: TxData, daisy: None }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_07, signal: TxData, daisy: None }

sai! { module: U1, alt: 3, pad: GPIO_B1_00,    signal: RxData, daisy: Some(DAISY_SAI1_RX_DATA0_GPIO_B1_00) }
sai! { module: U1, alt: 3, pad: GPIO_AD_B1_12, signal: RxData, daisy: Some(DAISY_SAI1_RX_DATA0_GPIO_AD_B1_12) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_06, signal: RxData, daisy: Some(DAISY_SAI1_RX_DATA0_GPIO_SD_B1_06) }

sai! { module: U1, alt: 3, pad: GPIO_B0_12,    signal: TxData1RxData3, daisy: Some(DAISY_SAI1_RX_DATA3_GPIO_B0_12) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_02, signal: TxData1RxData3, daisy: Some(DAISY_SAI1_RX_DATA3_GPIO_SD_B1_02) }

sai! { module: U1, alt: 3, pad: GPIO_B0_11,    signal: TxData2RxData2, daisy: Some(DAISY_SAI1_RX_DATA2_GPIO_B0_11) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_01, signal: TxData2RxData2, daisy: Some(DAISY_SAI1_RX_DATA2_GPIO_SD_B1_01) }

sai! { module: U1, alt: 3, pad: GPIO_B0_10,    signal: TxData3RxData1, daisy: Some(DAISY_SAI1_RX_DATA1_GPIO_B0_10) }
sai! { module: U1, alt: 3, pad: GPIO_SD_B1_00, signal: TxData3RxData1, daisy: Some(DAISY_SAI1_RX_DATA1_GPIO_SD_B1_00) }

//
// SAI2
//

sai! { module: U2, alt: 3, pad: GPIO_AD_B0_05, signal: TxBclk, daisy: Some(DAISY_SAI2_TX_BCLK_GPIO_AD_B0_05) }
sai! { module: U2, alt: 2, pad: GPIO_EMC_06,   signal: TxBclk, daisy: Some(DAISY_SAI2_TX_BCLK_GPIO_EMC_06) }

sai! { module: U2, alt: 3, pad: GPIO_AD_B0_04, signal: TxSync, daisy: Some(DAISY_SAI2_TX_SYNC_GPIO_AD_B0_04) }
sai! { module: U2, alt: 2, pad: GPIO_EMC_05,   signal: TxSync, daisy: Some(DAISY_SAI2_TX_SYNC_GPIO_EMC_05) }

sai! { module: U2, alt: 2, pad: GPIO_EMC_10,   signal: RxBclk, daisy: Some(DAISY_SAI2_RX_BCLK_GPIO_EMC_10) }
sai! { module: U2, alt: 3, pad: GPIO_AD_B0_06, signal: RxBclk, daisy: Some(DAISY_SAI2_RX_BCLK_GPIO_AD_B0_06) }

sai! { module: U2, alt: 2, pad: GPIO_EMC_09,   signal: RxSync, daisy: Some(DAISY_SAI2_RX_SYNC_GPIO_EMC_09) }
sai! { module: U2, alt: 3, pad: GPIO_AD_B0_07, signal: RxSync, daisy: Some(DAISY_SAI2_RX_SYNC_GPIO_AD_B0_07) }

sai! { module: U2, alt: 2, pad: GPIO_EMC_07,   signal: Mclk, daisy: Some(DAISY_SAI2_MCLK2_GPIO_EMC_07) }
sai! { module: U2, alt: 3, pad: GPIO_AD_B0_10, signal: Mclk, daisy: Some(DAISY_SAI2_MCLK2_GPIO_AD_B0_10) }

sai! { module: U2, alt: 2, pad: GPIO_EMC_04,   signal: TxData, daisy: None }
sai! { module: U2, alt: 3, pad: GPIO_AD_B0_09, signal: TxData, daisy: None }

sai! { module: U2, alt: 3, pad: GPIO_AD_B0_08, signal: RxData, daisy: Some(DAISY_SAI2_RX_DATA0_GPIO_AD_B0_08) }
sai! { module: U2, alt: 2, pad: GPIO_EMC_08,   signal: RxData, daisy: Some(DAISY_SAI2_RX_DATA0_GPIO_EMC_08) }

//
// SAI3
//

sai! { module: U3, alt: 3, pad: GPIO_EMC_38,   signal: TxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXBCLK_GPIO_EMC_38) }
sai! { module: U3, alt: 8, pad: GPIO_SD_B1_03, signal: TxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXBCLK_GPIO_SD_B1_03) }

sai! { module: U3, alt: 3, pad: GPIO_EMC_39,   signal: TxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXSYNC_GPIO_EMC_39) }
sai! { module: U3, alt: 8, pad: GPIO_SD_B1_02, signal: TxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXSYNC_GPIO_SD_B1_02) }

sai! { module: U3, alt: 3, pad: GPIO_EMC_35,   signal: RxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXBCLK_GPIO_EMC_35) }
sai! { module: U3, alt: 8, pad: GPIO_SD_B1_06, signal: RxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXBCLK_GPIO_SD_B1_06) }

sai! { module: U3, alt: 3, pad: GPIO_EMC_34,   signal: RxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXSYNC_GPIO_EMC_34) }
sai! { module: U3, alt: 8, pad: GPIO_SD_B1_05, signal: RxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXSYNC_GPIO_SD_B1_05) }

sai! { module: U3, alt: 3, pad: GPIO_EMC_37,   signal: Mclk, daisy: Some(DAISY_SAI3_IPG_CLK_SAI_MCLK_2_GPIO_EMC_37) }
sai! { module: U3, alt: 8, pad: GPIO_SD_B1_04, signal: Mclk, daisy: Some(DAISY_SAI3_IPG_CLK_SAI_MCLK_2_GPIO_SD_B1_04) }

sai! { module: U3, alt: 3, pad: GPIO_EMC_36,   signal: TxData, daisy: None }
sai! { module: U3, alt: 8, pad: GPIO_SD_B1_01, signal: TxData, daisy: None }

sai! { module: U3, alt: 3, pad: GPIO_EMC_33,   signal: RxData, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXDATA_0_GPIO_EMC_33) }
sai! { module: U3, alt: 8, pad: GPIO_SD_B1_00, signal: RxData, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXDATA_0_GPIO_SD_B1_00) }

mod daisy {
    use super::Daisy;

    pub const DAISY_SAI1_MCLK2_GPIO_SD_B1_03: Daisy = Daisy::new(0x401f858c as *mut u32, 0);
    pub const DAISY_SAI1_MCLK2_GPIO_AD_B1_09: Daisy = Daisy::new(0x401f858c as *mut u32, 1);
    pub const DAISY_SAI1_MCLK2_GPIO_B0_13: Daisy = Daisy::new(0x401f858c as *mut u32, 2);
    pub const DAISY_SAI1_RX_BCLK_GPIO_SD_B1_05: Daisy = Daisy::new(0x401f8590 as *mut u32, 0);
    pub const DAISY_SAI1_RX_BCLK_GPIO_AD_B1_11: Daisy = Daisy::new(0x401f8590 as *mut u32, 1);
    pub const DAISY_SAI1_RX_BCLK_GPIO_B0_15: Daisy = Daisy::new(0x401f8590 as *mut u32, 2);
    pub const DAISY_SAI1_RX_DATA0_GPIO_SD_B1_06: Daisy = Daisy::new(0x401f8594 as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA0_GPIO_AD_B1_12: Daisy = Daisy::new(0x401f8594 as *mut u32, 1);
    pub const DAISY_SAI1_RX_DATA0_GPIO_B1_00: Daisy = Daisy::new(0x401f8594 as *mut u32, 2);
    pub const DAISY_SAI1_RX_DATA1_GPIO_SD_B1_00: Daisy = Daisy::new(0x401f8598 as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA1_GPIO_B0_10: Daisy = Daisy::new(0x401f8598 as *mut u32, 1);
    pub const DAISY_SAI1_RX_DATA2_GPIO_SD_B1_01: Daisy = Daisy::new(0x401f859c as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA2_GPIO_B0_11: Daisy = Daisy::new(0x401f859c as *mut u32, 1);
    pub const DAISY_SAI1_RX_DATA3_GPIO_SD_B1_02: Daisy = Daisy::new(0x401f85a0 as *mut u32, 0);
    pub const DAISY_SAI1_RX_DATA3_GPIO_B0_12: Daisy = Daisy::new(0x401f85a0 as *mut u32, 1);
    pub const DAISY_SAI1_RX_SYNC_GPIO_SD_B1_04: Daisy = Daisy::new(0x401f85a4 as *mut u32, 0);
    pub const DAISY_SAI1_RX_SYNC_GPIO_AD_B1_10: Daisy = Daisy::new(0x401f85a4 as *mut u32, 1);
    pub const DAISY_SAI1_RX_SYNC_GPIO_B0_14: Daisy = Daisy::new(0x401f85a4 as *mut u32, 2);
    pub const DAISY_SAI1_TX_BCLK_GPIO_SD_B1_08: Daisy = Daisy::new(0x401f85a8 as *mut u32, 0);
    pub const DAISY_SAI1_TX_BCLK_GPIO_AD_B1_14: Daisy = Daisy::new(0x401f85a8 as *mut u32, 1);
    pub const DAISY_SAI1_TX_BCLK_GPIO_B1_02: Daisy = Daisy::new(0x401f85a8 as *mut u32, 2);
    pub const DAISY_SAI1_TX_SYNC_GPIO_SD_B1_09: Daisy = Daisy::new(0x401f85ac as *mut u32, 0);
    pub const DAISY_SAI1_TX_SYNC_GPIO_AD_B1_15: Daisy = Daisy::new(0x401f85ac as *mut u32, 1);
    pub const DAISY_SAI1_TX_SYNC_GPIO_B1_03: Daisy = Daisy::new(0x401f85ac as *mut u32, 2);
    pub const DAISY_SAI2_MCLK2_GPIO_EMC_07: Daisy = Daisy::new(0x401f85b0 as *mut u32, 0);
    pub const DAISY_SAI2_MCLK2_GPIO_AD_B0_10: Daisy = Daisy::new(0x401f85b0 as *mut u32, 1);
    pub const DAISY_SAI2_RX_BCLK_GPIO_EMC_10: Daisy = Daisy::new(0x401f85b4 as *mut u32, 0);
    pub const DAISY_SAI2_RX_BCLK_GPIO_AD_B0_06: Daisy = Daisy::new(0x401f85b4 as *mut u32, 1);
    pub const DAISY_SAI2_RX_DATA0_GPIO_EMC_08: Daisy = Daisy::new(0x401f85b8 as *mut u32, 0);
    pub const DAISY_SAI2_RX_DATA0_GPIO_AD_B0_08: Daisy = Daisy::new(0x401f85b8 as *mut u32, 1);
    pub const DAISY_SAI2_RX_SYNC_GPIO_EMC_09: Daisy = Daisy::new(0x401f85bc as *mut u32, 0);
    pub const DAISY_SAI2_RX_SYNC_GPIO_AD_B0_07: Daisy = Daisy::new(0x401f85bc as *mut u32, 1);
    pub const DAISY_SAI2_TX_BCLK_GPIO_EMC_06: Daisy = Daisy::new(0x401f85c0 as *mut u32, 0);
    pub const DAISY_SAI2_TX_BCLK_GPIO_AD_B0_05: Daisy = Daisy::new(0x401f85c0 as *mut u32, 1);
    pub const DAISY_SAI2_TX_SYNC_GPIO_EMC_05: Daisy = Daisy::new(0x401f85c4 as *mut u32, 0);
    pub const DAISY_SAI2_TX_SYNC_GPIO_AD_B0_04: Daisy = Daisy::new(0x401f85c4 as *mut u32, 1);
    pub const DAISY_SAI3_IPG_CLK_SAI_MCLK_2_GPIO_EMC_37: Daisy =
        Daisy::new(0x401f8770 as *mut u32, 0);
    pub const DAISY_SAI3_IPG_CLK_SAI_MCLK_2_GPIO_SD_B1_04: Daisy =
        Daisy::new(0x401f8770 as *mut u32, 1);
    pub const DAISY_SAI3_IPP_IND_SAI_RXBCLK_GPIO_EMC_35: Daisy =
        Daisy::new(0x401f8774 as *mut u32, 0);
    pub const DAISY_SAI3_IPP_IND_SAI_RXBCLK_GPIO_SD_B1_06: Daisy =
        Daisy::new(0x401f8774 as *mut u32, 1);
    pub const DAISY_SAI3_IPP_IND_SAI_RXDATA_0_GPIO_EMC_33: Daisy =
        Daisy::new(0x401f8778 as *mut u32, 0);
    pub const DAISY_SAI3_IPP_IND_SAI_RXDATA_0_GPIO_SD_B1_00: Daisy =
        Daisy::new(0x401f8778 as *mut u32, 1);
    pub const DAISY_SAI3_IPP_IND_SAI_RXSYNC_GPIO_EMC_34: Daisy =
        Daisy::new(0x401f877c as *mut u32, 0);
    pub const DAISY_SAI3_IPP_IND_SAI_RXSYNC_GPIO_SD_B1_05: Daisy =
        Daisy::new(0x401f877c as *mut u32, 1);
    pub const DAISY_SAI3_IPP_IND_SAI_TXBCLK_GPIO_EMC_38: Daisy =
        Daisy::new(0x401f8780 as *mut u32, 0);
    pub const DAISY_SAI3_IPP_IND_SAI_TXBCLK_GPIO_SD_B1_03: Daisy =
        Daisy::new(0x401f8780 as *mut u32, 1);
    pub const DAISY_SAI3_IPP_IND_SAI_TXSYNC_GPIO_EMC_39: Daisy =
        Daisy::new(0x401f8784 as *mut u32, 0);
    pub const DAISY_SAI3_IPP_IND_SAI_TXSYNC_GPIO_SD_B1_02: Daisy =
        Daisy::new(0x401f8784 as *mut u32, 1);
}

use daisy::*;
