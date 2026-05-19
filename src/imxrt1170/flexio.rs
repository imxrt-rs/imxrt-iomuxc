//! FlexIO pin implementation

use super::pads::{gpio_ad::*, gpio_emc_b1::*};

use crate::{flexio::Pin, Daisy};

//
// FlexIO 1
//
flexio!(module: 1, offset:  0, pad: GPIO_EMC_B1_00, alt: 8, daisy: None);
flexio!(module: 1, offset:  1, pad: GPIO_EMC_B1_01, alt: 8, daisy: None);
flexio!(module: 1, offset:  2, pad: GPIO_EMC_B1_02, alt: 8, daisy: None);
flexio!(module: 1, offset:  3, pad: GPIO_EMC_B1_03, alt: 8, daisy: None);
flexio!(module: 1, offset:  4, pad: GPIO_EMC_B1_04, alt: 8, daisy: None);
flexio!(module: 1, offset:  5, pad: GPIO_EMC_B1_05, alt: 8, daisy: None);
flexio!(module: 1, offset:  6, pad: GPIO_EMC_B1_06, alt: 8, daisy: None);
flexio!(module: 1, offset:  7, pad: GPIO_EMC_B1_07, alt: 8, daisy: None);
flexio!(module: 1, offset:  8, pad: GPIO_EMC_B1_08, alt: 8, daisy: None);
flexio!(module: 1, offset:  9, pad: GPIO_EMC_B1_09, alt: 8, daisy: None);
flexio!(module: 1, offset: 10, pad: GPIO_EMC_B1_10, alt: 8, daisy: None);
flexio!(module: 1, offset: 11, pad: GPIO_EMC_B1_11, alt: 8, daisy: None);
flexio!(module: 1, offset: 12, pad: GPIO_EMC_B1_12, alt: 8, daisy: None);
flexio!(module: 1, offset: 13, pad: GPIO_EMC_B1_13, alt: 8, daisy: None);
flexio!(module: 1, offset: 14, pad: GPIO_EMC_B1_14, alt: 8, daisy: None);
flexio!(module: 1, offset: 15, pad: GPIO_EMC_B1_15, alt: 8, daisy: None);
flexio!(module: 1, offset: 16, pad: GPIO_EMC_B1_16, alt: 8, daisy: None);
flexio!(module: 1, offset: 17, pad: GPIO_EMC_B1_17, alt: 8, daisy: None);
flexio!(module: 1, offset: 18, pad: GPIO_EMC_B1_18, alt: 8, daisy: None);
flexio!(module: 1, offset: 19, pad: GPIO_EMC_B1_19, alt: 8, daisy: None);
flexio!(module: 1, offset: 20, pad: GPIO_EMC_B1_20, alt: 8, daisy: None);
flexio!(module: 1, offset: 21, pad: GPIO_EMC_B1_21, alt: 8, daisy: None);
flexio!(module: 1, offset: 22, pad: GPIO_EMC_B1_22, alt: 8, daisy: None);
flexio!(module: 1, offset: 23, pad: GPIO_EMC_B1_23, alt: 8, daisy: None);
flexio!(module: 1, offset: 24, pad: GPIO_EMC_B1_24, alt: 8, daisy: None);
flexio!(module: 1, offset: 25, pad: GPIO_EMC_B1_25, alt: 8, daisy: None);
flexio!(module: 1, offset: 26, pad: GPIO_EMC_B1_26, alt: 8, daisy: None);
flexio!(module: 1, offset: 27, pad: GPIO_EMC_B1_27, alt: 8, daisy: None);
flexio!(module: 1, offset: 28, pad: GPIO_EMC_B1_28, alt: 8, daisy: None);
flexio!(module: 1, offset: 29, pad: GPIO_EMC_B1_29, alt: 8, daisy: None);
flexio!(module: 1, offset: 30, pad: GPIO_EMC_B1_30, alt: 8, daisy: None);
flexio!(module: 1, offset: 31, pad: GPIO_EMC_B1_31, alt: 8, daisy: None);

//
// FlexIO 2
//
flexio!(module: 2, offset:  0, pad: GPIO_AD_00, alt: 8, daisy: None);
flexio!(module: 2, offset:  1, pad: GPIO_AD_01, alt: 8, daisy: None);
flexio!(module: 2, offset:  2, pad: GPIO_AD_02, alt: 8, daisy: None);
flexio!(module: 2, offset:  3, pad: GPIO_AD_03, alt: 8, daisy: None);
flexio!(module: 2, offset:  4, pad: GPIO_AD_04, alt: 8, daisy: None);
flexio!(module: 2, offset:  5, pad: GPIO_AD_05, alt: 8, daisy: None);
flexio!(module: 2, offset:  6, pad: GPIO_AD_06, alt: 8, daisy: None);
flexio!(module: 2, offset:  7, pad: GPIO_AD_07, alt: 8, daisy: None);
flexio!(module: 2, offset:  8, pad: GPIO_AD_08, alt: 8, daisy: None);
flexio!(module: 2, offset:  9, pad: GPIO_AD_09, alt: 8, daisy: None);
flexio!(module: 2, offset: 10, pad: GPIO_AD_10, alt: 8, daisy: None);
flexio!(module: 2, offset: 11, pad: GPIO_AD_11, alt: 8, daisy: None);
flexio!(module: 2, offset: 12, pad: GPIO_AD_12, alt: 8, daisy: None);
flexio!(module: 2, offset: 13, pad: GPIO_AD_13, alt: 8, daisy: None);
flexio!(module: 2, offset: 14, pad: GPIO_AD_14, alt: 8, daisy: None);
flexio!(module: 2, offset: 15, pad: GPIO_AD_15, alt: 8, daisy: None);
flexio!(module: 2, offset: 16, pad: GPIO_AD_16, alt: 8, daisy: None);
flexio!(module: 2, offset: 17, pad: GPIO_AD_17, alt: 8, daisy: None);
flexio!(module: 2, offset: 18, pad: GPIO_AD_18, alt: 8, daisy: None);
flexio!(module: 2, offset: 19, pad: GPIO_AD_19, alt: 8, daisy: None);
flexio!(module: 2, offset: 20, pad: GPIO_AD_20, alt: 8, daisy: None);
flexio!(module: 2, offset: 21, pad: GPIO_AD_21, alt: 8, daisy: None);
flexio!(module: 2, offset: 22, pad: GPIO_AD_22, alt: 8, daisy: None);
flexio!(module: 2, offset: 23, pad: GPIO_AD_23, alt: 8, daisy: None);
flexio!(module: 2, offset: 24, pad: GPIO_AD_24, alt: 8, daisy: None);
flexio!(module: 2, offset: 25, pad: GPIO_AD_25, alt: 8, daisy: None);
flexio!(module: 2, offset: 26, pad: GPIO_AD_26, alt: 8, daisy: None);
flexio!(module: 2, offset: 27, pad: GPIO_AD_27, alt: 8, daisy: None);
flexio!(module: 2, offset: 28, pad: GPIO_AD_28, alt: 8, daisy: None);
flexio!(module: 2, offset: 29, pad: GPIO_AD_29, alt: 8, daisy: None);
flexio!(module: 2, offset: 30, pad: GPIO_AD_30, alt: 8, daisy: None);
flexio!(module: 2, offset: 31, pad: GPIO_AD_31, alt: 8, daisy: None);
