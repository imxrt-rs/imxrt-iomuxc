//! SEMC pin implementation

use super::pads::{
    gpio_emc::*,
    gpio_b0::*,
    gpio_sd_b1::*,
};
use crate::{
    consts::*,
    semc::*,
};

semc!(module: U1, alt: 0, pad: GPIO_EMC_00, signal: Data0);
semc!(module: U1, alt: 0, pad: GPIO_EMC_01, signal: Data1);
semc!(module: U1, alt: 0, pad: GPIO_EMC_02, signal: Data2);
semc!(module: U1, alt: 0, pad: GPIO_EMC_03, signal: Data3);
semc!(module: U1, alt: 0, pad: GPIO_EMC_04, signal: Data4);
semc!(module: U1, alt: 0, pad: GPIO_EMC_05, signal: Data5);
semc!(module: U1, alt: 0, pad: GPIO_EMC_06, signal: Data6);
semc!(module: U1, alt: 0, pad: GPIO_EMC_07, signal: Data7);
semc!(module: U1, alt: 0, pad: GPIO_EMC_30, signal: Data8);
semc!(module: U1, alt: 0, pad: GPIO_EMC_31, signal: Data9);
semc!(module: U1, alt: 0, pad: GPIO_EMC_32, signal: Data10);
semc!(module: U1, alt: 0, pad: GPIO_EMC_33, signal: Data11);
semc!(module: U1, alt: 0, pad: GPIO_EMC_34, signal: Data12);
semc!(module: U1, alt: 0, pad: GPIO_EMC_35, signal: Data13);
semc!(module: U1, alt: 0, pad: GPIO_EMC_36, signal: Data14);
semc!(module: U1, alt: 0, pad: GPIO_EMC_37, signal: Data15);
semc!(module: U1, alt: 0, pad: GPIO_EMC_09, signal: Addr0);
semc!(module: U1, alt: 0, pad: GPIO_EMC_10, signal: Addr1);
semc!(module: U1, alt: 0, pad: GPIO_EMC_11, signal: Addr2);
semc!(module: U1, alt: 0, pad: GPIO_EMC_12, signal: Addr3);
semc!(module: U1, alt: 0, pad: GPIO_EMC_13, signal: Addr4);
semc!(module: U1, alt: 0, pad: GPIO_EMC_14, signal: Addr5);
semc!(module: U1, alt: 0, pad: GPIO_EMC_15, signal: Addr6);
semc!(module: U1, alt: 0, pad: GPIO_EMC_16, signal: Addr7);
semc!(module: U1, alt: 0, pad: GPIO_EMC_17, signal: Addr8);
semc!(module: U1, alt: 0, pad: GPIO_EMC_18, signal: Addr9);
semc!(module: U1, alt: 0, pad: GPIO_EMC_23, signal: Addr10);
semc!(module: U1, alt: 0, pad: GPIO_EMC_19, signal: Addr11);
semc!(module: U1, alt: 0, pad: GPIO_EMC_20, signal: Addr12);
semc!(module: U1, alt: 0, pad: GPIO_EMC_08, signal: Dm0);
semc!(module: U1, alt: 0, pad: GPIO_EMC_38, signal: Dm1);
semc!(module: U1, alt: 0, pad: GPIO_EMC_21, signal: Ba0);
semc!(module: U1, alt: 0, pad: GPIO_EMC_22, signal: Ba1);
semc!(module: U1, alt: 0, pad: GPIO_EMC_24, signal: Cas);
semc!(module: U1, alt: 0, pad: GPIO_EMC_25, signal: Ras);
semc!(module: U1, alt: 0, pad: GPIO_EMC_26, signal: Clk);
semc!(module: U1, alt: 0, pad: GPIO_EMC_27, signal: Cke);
semc!(module: U1, alt: 0, pad: GPIO_EMC_28, signal: We);
semc!(module: U1, alt: 0, pad: GPIO_EMC_29, signal: Cs0);
semc!(module: U1, alt: 0, pad: GPIO_EMC_39, signal: Dqs);
semc!(module: U1, alt: 0, pad: GPIO_EMC_40, signal: Rdy);
semc!(module: U1, alt: 0, pad: GPIO_EMC_41, signal: Csx0);
semc!(module: U1, alt: 0, pad: GPIO_SD_B1_07, signal: Csx1);
semc!(module: U1, alt: 6, pad: GPIO_B0_00, signal: Csx1);
semc!(module: U1, alt: 6, pad: GPIO_SD_B1_08, signal: Csx2);
semc!(module: U1, alt: 6, pad: GPIO_B0_01, signal: Csx2);
semc!(module: U1, alt: 6, pad: GPIO_B0_02, signal: Csx3);

