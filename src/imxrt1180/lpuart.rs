use super::pads::{
    gpio_ad::*, gpio_aon::*, gpio_b1::*, gpio_b2::*, gpio_emc_b1::*, gpio_emc_b2::*, gpio_sd_b2::*,
};
use crate::{
    consts::*,
    lpuart::{Pin, Rx, Tx},
    Daisy,
};

uart!(module: U1, alt: 0b0000, pad: GPIO_AON_08, direction: Tx, daisy: None);
uart!(module: U1, alt: 0b0000, pad: GPIO_AON_09, direction: Rx, daisy: None);
uart!(module: U2, alt: 0b0110, pad: GPIO_AON_01, direction: Rx, daisy: Some(DAISY_LPUART2_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_01));
uart!(module: U2, alt: 0b0010, pad: GPIO_AON_27, direction: Rx, daisy: Some(DAISY_LPUART2_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_27));
uart!(module: U2, alt: 0b0110, pad: GPIO_AON_00, direction: Tx, daisy: Some(DAISY_LPUART2_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_00));
uart!(module: U2, alt: 0b0010, pad: GPIO_AON_26, direction: Tx, daisy: Some(DAISY_LPUART2_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_26));
uart!(module: U3, alt: 0b0011, pad: GPIO_EMC_B1_02, direction: Rx, daisy: Some(DAISY_LPUART3_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_02));
uart!(module: U3, alt: 0b0110, pad: GPIO_AD_14, direction: Rx, daisy: Some(DAISY_LPUART3_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_14));
uart!(module: U3, alt: 0b0011, pad: GPIO_EMC_B1_03, direction: Tx, daisy: Some(DAISY_LPUART3_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_03));
uart!(module: U3, alt: 0b0110, pad: GPIO_AD_13, direction: Tx, daisy: Some(DAISY_LPUART3_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_13));
uart!(module: U4, alt: 0b0010, pad: GPIO_EMC_B1_12, direction: Tx, daisy: None);
uart!(module: U4, alt: 0b0010, pad: GPIO_EMC_B1_13, direction: Rx, daisy: None);
uart!(module: U5, alt: 0b0010, pad: GPIO_EMC_B1_15, direction: Rx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_15));
uart!(module: U5, alt: 0b0010, pad: GPIO_EMC_B1_36, direction: Rx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_36));
uart!(module: U5, alt: 0b0010, pad: GPIO_EMC_B2_18, direction: Rx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B2_18));
uart!(module: U5, alt: 0b0001, pad: GPIO_AD_27, direction: Rx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_27));
uart!(module: U5, alt: 0b0110, pad: GPIO_SD_B2_09, direction: Rx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_SD_B2_09));
uart!(module: U5, alt: 0b0010, pad: GPIO_EMC_B1_14, direction: Tx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_14));
uart!(module: U5, alt: 0b0010, pad: GPIO_EMC_B1_35, direction: Tx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_35));
uart!(module: U5, alt: 0b0010, pad: GPIO_EMC_B2_17, direction: Tx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B2_17));
uart!(module: U5, alt: 0b0001, pad: GPIO_AD_26, direction: Tx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_26));
uart!(module: U5, alt: 0b0110, pad: GPIO_SD_B2_08, direction: Tx, daisy: Some(DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_SD_B2_08));
uart!(module: U6, alt: 0b0010, pad: GPIO_EMC_B1_32, direction: Rx, daisy: Some(DAISY_LPUART6_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_32));
uart!(module: U6, alt: 0b0000, pad: GPIO_AD_25, direction: Rx, daisy: Some(DAISY_LPUART6_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_25));
uart!(module: U6, alt: 0b0100, pad: GPIO_B2_11, direction: Rx, daisy: Some(DAISY_LPUART6_IPP_IND_LPUART_RXD_SELECT_GPIO_B2_11));
uart!(module: U6, alt: 0b0010, pad: GPIO_EMC_B1_31, direction: Tx, daisy: Some(DAISY_LPUART6_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_31));
uart!(module: U6, alt: 0b0000, pad: GPIO_AD_24, direction: Tx, daisy: Some(DAISY_LPUART6_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_24));
uart!(module: U6, alt: 0b0100, pad: GPIO_B2_10, direction: Tx, daisy: Some(DAISY_LPUART6_IPP_IND_LPUART_TXD_SELECT_GPIO_B2_10));
uart!(module: U7, alt: 0b0010, pad: GPIO_AON_18, direction: Rx, daisy: Some(DAISY_LPUART7_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_18));
uart!(module: U7, alt: 0b0010, pad: GPIO_AON_23, direction: Rx, daisy: Some(DAISY_LPUART7_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_23));
uart!(module: U7, alt: 0b0010, pad: GPIO_AON_17, direction: Tx, daisy: Some(DAISY_LPUART7_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_17));
uart!(module: U7, alt: 0b0010, pad: GPIO_AON_22, direction: Tx, daisy: Some(DAISY_LPUART7_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_22));
uart!(module: U8, alt: 0b0100, pad: GPIO_AD_31, direction: Rx, daisy: Some(DAISY_LPUART8_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_31));
uart!(module: U8, alt: 0b1001, pad: GPIO_SD_B2_01, direction: Rx, daisy: Some(DAISY_LPUART8_IPP_IND_LPUART_RXD_SELECT_GPIO_SD_B2_01));
uart!(module: U8, alt: 0b0010, pad: GPIO_B2_13, direction: Rx, daisy: Some(DAISY_LPUART8_IPP_IND_LPUART_RXD_SELECT_GPIO_B2_13));
uart!(module: U8, alt: 0b0100, pad: GPIO_AD_30, direction: Tx, daisy: Some(DAISY_LPUART8_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_30));
uart!(module: U8, alt: 0b1001, pad: GPIO_SD_B2_00, direction: Tx, daisy: Some(DAISY_LPUART8_IPP_IND_LPUART_TXD_SELECT_GPIO_SD_B2_00));
uart!(module: U8, alt: 0b0010, pad: GPIO_B2_12, direction: Tx, daisy: Some(DAISY_LPUART8_IPP_IND_LPUART_TXD_SELECT_GPIO_B2_12));
uart!(module: U9, alt: 0b0010, pad: GPIO_EMC_B1_17, direction: Rx, daisy: Some(DAISY_LPUART9_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_17));
uart!(module: U9, alt: 0b0010, pad: GPIO_B1_04, direction: Rx, daisy: Some(DAISY_LPUART9_IPP_IND_LPUART_RXD_SELECT_GPIO_B1_04));
uart!(module: U9, alt: 0b0010, pad: GPIO_EMC_B1_16, direction: Tx, daisy: Some(DAISY_LPUART9_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_16));
uart!(module: U9, alt: 0b0010, pad: GPIO_B1_06, direction: Tx, daisy: Some(DAISY_LPUART9_IPP_IND_LPUART_TXD_SELECT_GPIO_B1_06));
uart!(module: U10, alt: 0b0001, pad: GPIO_AD_16, direction: Rx, daisy: Some(DAISY_LPUART10_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_16));
uart!(module: U10, alt: 0b1000, pad: GPIO_AD_33, direction: Rx, daisy: Some(DAISY_LPUART10_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_33));
uart!(module: U10, alt: 0b0001, pad: GPIO_AD_15, direction: Tx, daisy: Some(DAISY_LPUART10_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_15));
uart!(module: U10, alt: 0b1000, pad: GPIO_AD_32, direction: Tx, daisy: Some(DAISY_LPUART10_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_32));
uart!(module: U11, alt: 0b0010, pad: GPIO_EMC_B2_14, direction: Rx, daisy: Some(DAISY_LPUART11_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B2_14));
uart!(module: U11, alt: 0b1001, pad: GPIO_B1_03, direction: Rx, daisy: Some(DAISY_LPUART11_IPP_IND_LPUART_RXD_SELECT_GPIO_B1_03));
uart!(module: U11, alt: 0b1001, pad: GPIO_B2_07, direction: Rx, daisy: Some(DAISY_LPUART11_IPP_IND_LPUART_RXD_SELECT_GPIO_B2_07));
uart!(module: U11, alt: 0b0010, pad: GPIO_EMC_B2_13, direction: Tx, daisy: Some(DAISY_LPUART11_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B2_13));
uart!(module: U11, alt: 0b1001, pad: GPIO_B1_02, direction: Tx, daisy: Some(DAISY_LPUART11_IPP_IND_LPUART_TXD_SELECT_GPIO_B1_02));
uart!(module: U11, alt: 0b1001, pad: GPIO_B2_06, direction: Tx, daisy: Some(DAISY_LPUART11_IPP_IND_LPUART_TXD_SELECT_GPIO_B2_06));
uart!(module: U12, alt: 0b0010, pad: GPIO_AON_16, direction: Rx, daisy: Some(DAISY_LPUART12_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_16));
uart!(module: U12, alt: 0b1001, pad: GPIO_AON_20, direction: Rx, daisy: Some(DAISY_LPUART12_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_20));
uart!(module: U12, alt: 0b0010, pad: GPIO_AON_15, direction: Tx, daisy: Some(DAISY_LPUART12_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_15));
uart!(module: U12, alt: 0b1001, pad: GPIO_AON_19, direction: Tx, daisy: Some(DAISY_LPUART12_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_19));

mod daisy {
    use super::Daisy;

    pub const DAISY_LPUART2_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_01: Daisy =
        Daisy::new(0x443c0154 as *mut u32, 0);
    pub const DAISY_LPUART2_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_27: Daisy =
        Daisy::new(0x443c0154 as *mut u32, 1);
    pub const DAISY_LPUART2_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_00: Daisy =
        Daisy::new(0x443c0158 as *mut u32, 0);
    pub const DAISY_LPUART2_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_26: Daisy =
        Daisy::new(0x443c0158 as *mut u32, 1);
    pub const DAISY_LPUART3_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_02: Daisy =
        Daisy::new(0x42a10680 as *mut u32, 0);
    pub const DAISY_LPUART3_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_14: Daisy =
        Daisy::new(0x42a10680 as *mut u32, 1);
    pub const DAISY_LPUART3_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_03: Daisy =
        Daisy::new(0x42a10684 as *mut u32, 0);
    pub const DAISY_LPUART3_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_13: Daisy =
        Daisy::new(0x42a10684 as *mut u32, 1);
    pub const DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_15: Daisy =
        Daisy::new(0x42a1069c as *mut u32, 0);
    pub const DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_36: Daisy =
        Daisy::new(0x42a1069c as *mut u32, 1);
    pub const DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B2_18: Daisy =
        Daisy::new(0x42a1069c as *mut u32, 2);
    pub const DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_27: Daisy =
        Daisy::new(0x42a1069c as *mut u32, 3);
    pub const DAISY_LPUART5_IPP_IND_LPUART_RXD_SELECT_GPIO_SD_B2_09: Daisy =
        Daisy::new(0x42a1069c as *mut u32, 4);
    pub const DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_14: Daisy =
        Daisy::new(0x42a106a0 as *mut u32, 0);
    pub const DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_35: Daisy =
        Daisy::new(0x42a106a0 as *mut u32, 1);
    pub const DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B2_17: Daisy =
        Daisy::new(0x42a106a0 as *mut u32, 2);
    pub const DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_26: Daisy =
        Daisy::new(0x42a106a0 as *mut u32, 3);
    pub const DAISY_LPUART5_IPP_IND_LPUART_TXD_SELECT_GPIO_SD_B2_08: Daisy =
        Daisy::new(0x42a106a0 as *mut u32, 4);
    pub const DAISY_LPUART6_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_32: Daisy =
        Daisy::new(0x42a106b4 as *mut u32, 0);
    pub const DAISY_LPUART6_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_25: Daisy =
        Daisy::new(0x42a106b4 as *mut u32, 1);
    pub const DAISY_LPUART6_IPP_IND_LPUART_RXD_SELECT_GPIO_B2_11: Daisy =
        Daisy::new(0x42a106b4 as *mut u32, 2);
    pub const DAISY_LPUART6_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_31: Daisy =
        Daisy::new(0x42a106b8 as *mut u32, 0);
    pub const DAISY_LPUART6_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_24: Daisy =
        Daisy::new(0x42a106b8 as *mut u32, 1);
    pub const DAISY_LPUART6_IPP_IND_LPUART_TXD_SELECT_GPIO_B2_10: Daisy =
        Daisy::new(0x42a106b8 as *mut u32, 2);
    pub const DAISY_LPUART7_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_18: Daisy =
        Daisy::new(0x443c0160 as *mut u32, 0);
    pub const DAISY_LPUART7_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_23: Daisy =
        Daisy::new(0x443c0160 as *mut u32, 1);
    pub const DAISY_LPUART7_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_17: Daisy =
        Daisy::new(0x443c0164 as *mut u32, 0);
    pub const DAISY_LPUART7_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_22: Daisy =
        Daisy::new(0x443c0164 as *mut u32, 1);
    pub const DAISY_LPUART8_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_31: Daisy =
        Daisy::new(0x42a106c0 as *mut u32, 0);
    pub const DAISY_LPUART8_IPP_IND_LPUART_RXD_SELECT_GPIO_SD_B2_01: Daisy =
        Daisy::new(0x42a106c0 as *mut u32, 1);
    pub const DAISY_LPUART8_IPP_IND_LPUART_RXD_SELECT_GPIO_B2_13: Daisy =
        Daisy::new(0x42a106c0 as *mut u32, 2);
    pub const DAISY_LPUART8_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_30: Daisy =
        Daisy::new(0x42a106c4 as *mut u32, 0);
    pub const DAISY_LPUART8_IPP_IND_LPUART_TXD_SELECT_GPIO_SD_B2_00: Daisy =
        Daisy::new(0x42a106c4 as *mut u32, 1);
    pub const DAISY_LPUART8_IPP_IND_LPUART_TXD_SELECT_GPIO_B2_12: Daisy =
        Daisy::new(0x42a106c4 as *mut u32, 2);
    pub const DAISY_LPUART9_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B1_17: Daisy =
        Daisy::new(0x42a106c8 as *mut u32, 0);
    pub const DAISY_LPUART9_IPP_IND_LPUART_RXD_SELECT_GPIO_B1_04: Daisy =
        Daisy::new(0x42a106c8 as *mut u32, 1);
    pub const DAISY_LPUART9_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B1_16: Daisy =
        Daisy::new(0x42a106cc as *mut u32, 0);
    pub const DAISY_LPUART9_IPP_IND_LPUART_TXD_SELECT_GPIO_B1_06: Daisy =
        Daisy::new(0x42a106cc as *mut u32, 1);
    pub const DAISY_LPUART10_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_16: Daisy =
        Daisy::new(0x42a1066c as *mut u32, 0);
    pub const DAISY_LPUART10_IPP_IND_LPUART_RXD_SELECT_GPIO_AD_33: Daisy =
        Daisy::new(0x42a1066c as *mut u32, 1);
    pub const DAISY_LPUART10_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_15: Daisy =
        Daisy::new(0x42a10670 as *mut u32, 0);
    pub const DAISY_LPUART10_IPP_IND_LPUART_TXD_SELECT_GPIO_AD_32: Daisy =
        Daisy::new(0x42a10670 as *mut u32, 1);
    pub const DAISY_LPUART11_IPP_IND_LPUART_RXD_SELECT_GPIO_EMC_B2_14: Daisy =
        Daisy::new(0x42a10674 as *mut u32, 0);
    pub const DAISY_LPUART11_IPP_IND_LPUART_RXD_SELECT_GPIO_B1_03: Daisy =
        Daisy::new(0x42a10674 as *mut u32, 1);
    pub const DAISY_LPUART11_IPP_IND_LPUART_RXD_SELECT_GPIO_B2_07: Daisy =
        Daisy::new(0x42a10674 as *mut u32, 2);
    pub const DAISY_LPUART11_IPP_IND_LPUART_TXD_SELECT_GPIO_EMC_B2_13: Daisy =
        Daisy::new(0x42a10678 as *mut u32, 0);
    pub const DAISY_LPUART11_IPP_IND_LPUART_TXD_SELECT_GPIO_B1_02: Daisy =
        Daisy::new(0x42a10678 as *mut u32, 1);
    pub const DAISY_LPUART11_IPP_IND_LPUART_TXD_SELECT_GPIO_B2_06: Daisy =
        Daisy::new(0x42a10678 as *mut u32, 2);
    pub const DAISY_LPUART12_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_16: Daisy =
        Daisy::new(0x443c0148 as *mut u32, 0);
    pub const DAISY_LPUART12_IPP_IND_LPUART_RXD_SELECT_GPIO_AON_20: Daisy =
        Daisy::new(0x443c0148 as *mut u32, 1);
    pub const DAISY_LPUART12_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_15: Daisy =
        Daisy::new(0x443c014c as *mut u32, 0);
    pub const DAISY_LPUART12_IPP_IND_LPUART_TXD_SELECT_GPIO_AON_19: Daisy =
        Daisy::new(0x443c014c as *mut u32, 1);
}

use daisy::*;
