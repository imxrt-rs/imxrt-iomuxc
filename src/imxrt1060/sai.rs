//! SAI / I2S pin implementation

use super::{gpio_emc::*, gpio_sd_b1::*};
use crate::{consts::*, sai::*, Daisy};

//
// SAI1 and SAI2, as well as the signals, are shared with the
// 1050. See the common module.
//

//
// SAI3
//

sai!(module: U3, alt: 3, pad: GPIO_EMC_38,   signal: TxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXBCLK_GPIO_EMC_38));
sai!(module: U3, alt: 8, pad: GPIO_SD_B1_03, signal: TxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXBCLK_GPIO_SD_B1_03));

sai!(module: U3, alt: 3, pad: GPIO_EMC_39,   signal: TxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXSYNC_GPIO_EMC_39));
sai!(module: U3, alt: 8, pad: GPIO_SD_B1_02, signal: TxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_TXSYNC_GPIO_SD_B1_02));

sai!(module: U3, alt: 3, pad: GPIO_EMC_35,   signal: RxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXBCLK_GPIO_EMC_35));
sai!(module: U3, alt: 8, pad: GPIO_SD_B1_06, signal: RxBclk, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXBCLK_GPIO_SD_B1_06));

sai!(module: U3, alt: 3, pad: GPIO_EMC_34,   signal: RxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXSYNC_GPIO_EMC_34));
sai!(module: U3, alt: 8, pad: GPIO_SD_B1_05, signal: RxSync, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXSYNC_GPIO_SD_B1_05));

sai!(module: U3, alt: 3, pad: GPIO_EMC_37,   signal: Mclk, daisy: Some(DAISY_SAI3_IPG_CLK_SAI_MCLK_2_GPIO_EMC_37));
sai!(module: U3, alt: 8, pad: GPIO_SD_B1_04, signal: Mclk, daisy: Some(DAISY_SAI3_IPG_CLK_SAI_MCLK_2_GPIO_SD_B1_04));

sai!(module: U3, alt: 3, pad: GPIO_EMC_36,   signal: TxData, daisy: None);
sai!(module: U3, alt: 8, pad: GPIO_SD_B1_01, signal: TxData, daisy: None);

sai!(module: U3, alt: 3, pad: GPIO_EMC_33,   signal: RxData, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXDATA_0_GPIO_EMC_33));
sai!(module: U3, alt: 8, pad: GPIO_SD_B1_00, signal: RxData, daisy: Some(DAISY_SAI3_IPP_IND_SAI_RXDATA_0_GPIO_SD_B1_00));

mod daisy {
    use super::Daisy;

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
