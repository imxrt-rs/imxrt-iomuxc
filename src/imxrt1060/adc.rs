//! ADC pin implementations
//!
//! Implementation derived from Table 66-2: ADC External Signals
//! from the iMXRT1060 Reference Manual, Rev 2. There is a similar
//! information available in Table 10-1: Muxing Options, in the IOMUXC
//! section of the reference manual.

use super::{gpio_ad_b0::*, gpio_ad_b1::*};
use crate::adc::{Adc1, Adc2, Pin};

//
// Adc1
//
adc!(module: Adc1, pad: GPIO_AD_B1_11, input: 0);
adc!(module: Adc1, pad: GPIO_AD_B0_12, input: 1);
adc!(module: Adc1, pad: GPIO_AD_B0_13, input: 2);
adc!(module: Adc1, pad: GPIO_AD_B0_14, input: 3);
adc!(module: Adc1, pad: GPIO_AD_B0_15, input: 4);
adc!(module: Adc1, pad: GPIO_AD_B1_00, input: 5);
adc!(module: Adc1, pad: GPIO_AD_B1_01, input: 6);
adc!(module: Adc1, pad: GPIO_AD_B1_02, input: 7);
adc!(module: Adc1, pad: GPIO_AD_B1_03, input: 8);
adc!(module: Adc1, pad: GPIO_AD_B1_04, input: 9);
adc!(module: Adc1, pad: GPIO_AD_B1_05, input: 10);
adc!(module: Adc1, pad: GPIO_AD_B1_06, input: 11);
adc!(module: Adc1, pad: GPIO_AD_B1_07, input: 12);
adc!(module: Adc1, pad: GPIO_AD_B1_08, input: 13);
adc!(module: Adc1, pad: GPIO_AD_B1_09, input: 14);
adc!(module: Adc1, pad: GPIO_AD_B1_10, input: 15);

//
// Adc2
//
adc!(module: Adc2, pad: GPIO_AD_B1_11, input: 0);
adc!(module: Adc2, pad: GPIO_AD_B1_12, input: 1);
adc!(module: Adc2, pad: GPIO_AD_B1_13, input: 2);
adc!(module: Adc2, pad: GPIO_AD_B1_14, input: 3);
adc!(module: Adc2, pad: GPIO_AD_B1_15, input: 4);
adc!(module: Adc2, pad: GPIO_AD_B1_00, input: 5);
adc!(module: Adc2, pad: GPIO_AD_B1_01, input: 6);
adc!(module: Adc2, pad: GPIO_AD_B1_02, input: 7);
adc!(module: Adc2, pad: GPIO_AD_B1_03, input: 8);
adc!(module: Adc2, pad: GPIO_AD_B1_04, input: 9);
adc!(module: Adc2, pad: GPIO_AD_B1_05, input: 10);
adc!(module: Adc2, pad: GPIO_AD_B1_06, input: 11);
adc!(module: Adc2, pad: GPIO_AD_B1_07, input: 12);
adc!(module: Adc2, pad: GPIO_AD_B1_08, input: 13);
adc!(module: Adc2, pad: GPIO_AD_B1_09, input: 14);
adc!(module: Adc2, pad: GPIO_AD_B1_10, input: 15);
