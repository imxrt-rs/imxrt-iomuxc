//! FlexSPI pin implementation

use super::pads::{gpio_ad_b1::*, gpio_b0::*, gpio_b1::*, gpio_emc::*};

use crate::{flexio::Pin, Daisy};

//
// FlexIO 1
//
flexio!(module: 1, offset: 0, pad: GPIO_EMC_00, alt: 4, daisy: None);
flexio!(module: 1, offset: 1, pad: GPIO_EMC_01, alt: 4, daisy: None);
flexio!(module: 1, offset: 2, pad: GPIO_EMC_02, alt: 4, daisy: None);
flexio!(module: 1, offset: 3, pad: GPIO_EMC_03, alt: 4, daisy: None);
flexio!(module: 1, offset: 4, pad: GPIO_EMC_04, alt: 4, daisy: None);
flexio!(module: 1, offset: 5, pad: GPIO_EMC_05, alt: 4, daisy: None);
flexio!(module: 1, offset: 6, pad: GPIO_EMC_06, alt: 4, daisy: None);
flexio!(module: 1, offset: 7, pad: GPIO_EMC_07, alt: 4, daisy: None);
flexio!(module: 1, offset: 8, pad: GPIO_EMC_08, alt: 4, daisy: None);
flexio!(module: 1, offset: 9, pad: GPIO_EMC_09, alt: 4, daisy: None);
flexio!(module: 1, offset: 10, pad: GPIO_EMC_10, alt: 4, daisy: None);
flexio!(module: 1, offset: 11, pad: GPIO_EMC_11, alt: 4, daisy: None);
flexio!(module: 1, offset: 12, pad: GPIO_EMC_26, alt: 4, daisy: None);
flexio!(module: 1, offset: 13, pad: GPIO_EMC_27, alt: 4, daisy: None);
flexio!(module: 1, offset: 14, pad: GPIO_EMC_28, alt: 4, daisy: None);
flexio!(module: 1, offset: 15, pad: GPIO_EMC_29, alt: 4, daisy: None);

//
// FlexIO 2
//
flexio!(module: 2, offset: 0, pad: GPIO_B0_00, alt: 4, daisy: None);
flexio!(module: 2, offset: 1, pad: GPIO_B0_01, alt: 4, daisy: None);
flexio!(module: 2, offset: 2, pad: GPIO_B0_02, alt: 4, daisy: None);
flexio!(module: 2, offset: 3, pad: GPIO_B0_03, alt: 4, daisy: None);
flexio!(module: 2, offset: 4, pad: GPIO_B0_04, alt: 4, daisy: None);
flexio!(module: 2, offset: 5, pad: GPIO_B0_05, alt: 4, daisy: None);
flexio!(module: 2, offset: 6, pad: GPIO_B0_06, alt: 4, daisy: None);
flexio!(module: 2, offset: 7, pad: GPIO_B0_07, alt: 4, daisy: None);
flexio!(module: 2, offset: 8, pad: GPIO_B0_08, alt: 4, daisy: None);
flexio!(module: 2, offset: 9, pad: GPIO_B0_09, alt: 4, daisy: None);
flexio!(module: 2, offset: 10, pad: GPIO_B0_10, alt: 4, daisy: None);
flexio!(module: 2, offset: 11, pad: GPIO_B0_11, alt: 4, daisy: None);
flexio!(module: 2, offset: 12, pad: GPIO_B0_12, alt: 4, daisy: None);
flexio!(module: 2, offset: 13, pad: GPIO_B0_13, alt: 4, daisy: None);
flexio!(module: 2, offset: 14, pad: GPIO_B0_14, alt: 4, daisy: None);
flexio!(module: 2, offset: 15, pad: GPIO_B0_15, alt: 4, daisy: None);
flexio!(module: 2, offset: 16, pad: GPIO_B1_00, alt: 4, daisy: None);
flexio!(module: 2, offset: 17, pad: GPIO_B1_01, alt: 4, daisy: None);
flexio!(module: 2, offset: 18, pad: GPIO_B1_02, alt: 4, daisy: None);
flexio!(module: 2, offset: 19, pad: GPIO_B1_03, alt: 4, daisy: None);
flexio!(module: 2, offset: 20, pad: GPIO_B1_04, alt: 4, daisy: None);
flexio!(module: 2, offset: 21, pad: GPIO_B1_05, alt: 4, daisy: None);
flexio!(module: 2, offset: 22, pad: GPIO_B1_06, alt: 4, daisy: None);
flexio!(module: 2, offset: 23, pad: GPIO_B1_07, alt: 4, daisy: None);
flexio!(module: 2, offset: 24, pad: GPIO_B1_08, alt: 4, daisy: None);
flexio!(module: 2, offset: 25, pad: GPIO_B1_09, alt: 4, daisy: None);
flexio!(module: 2, offset: 26, pad: GPIO_B1_10, alt: 4, daisy: None);
flexio!(module: 2, offset: 27, pad: GPIO_B1_11, alt: 4, daisy: None);
flexio!(module: 2, offset: 28, pad: GPIO_B1_12, alt: 4, daisy: None);
flexio!(module: 2, offset: 29, pad: GPIO_B1_13, alt: 4, daisy: None);
flexio!(module: 2, offset: 30, pad: GPIO_B1_14, alt: 4, daisy: None);
flexio!(module: 2, offset: 31, pad: GPIO_B1_15, alt: 4, daisy: None);

//
// FlexIO 3
//
flexio!(module: 3, offset: 0, pad: GPIO_AD_B1_00, alt: 9, daisy: None);
flexio!(module: 3, offset: 1, pad: GPIO_AD_B1_01, alt: 9, daisy: None);
flexio!(module: 3, offset: 2, pad: GPIO_AD_B1_02, alt: 9, daisy: None);
flexio!(module: 3, offset: 3, pad: GPIO_AD_B1_03, alt: 9, daisy: None);
flexio!(module: 3, offset: 4, pad: GPIO_AD_B1_04, alt: 9, daisy: None);
flexio!(module: 3, offset: 5, pad: GPIO_AD_B1_05, alt: 9, daisy: None);
flexio!(module: 3, offset: 6, pad: GPIO_AD_B1_06, alt: 9, daisy: None);
flexio!(module: 3, offset: 7, pad: GPIO_AD_B1_07, alt: 9, daisy: None);
flexio!(module: 3, offset: 8, pad: GPIO_AD_B1_08, alt: 9, daisy: None);
flexio!(module: 3, offset: 9, pad: GPIO_AD_B1_09, alt: 9, daisy: None);
flexio!(module: 3, offset: 10, pad: GPIO_AD_B1_10, alt: 9, daisy: None);
flexio!(module: 3, offset: 11, pad: GPIO_AD_B1_11, alt: 9, daisy: None);
flexio!(module: 3, offset: 12, pad: GPIO_AD_B1_12, alt: 9, daisy: None);
flexio!(module: 3, offset: 13, pad: GPIO_AD_B1_13, alt: 9, daisy: None);
flexio!(module: 3, offset: 14, pad: GPIO_AD_B1_14, alt: 9, daisy: None);
flexio!(module: 3, offset: 15, pad: GPIO_AD_B1_15, alt: 9, daisy: None);
flexio!(module: 3, offset: 16, pad: GPIO_B1_00, alt: 9, daisy: None);
flexio!(module: 3, offset: 17, pad: GPIO_B1_01, alt: 9, daisy: None);
flexio!(module: 3, offset: 18, pad: GPIO_B1_02, alt: 9, daisy: None);
flexio!(module: 3, offset: 19, pad: GPIO_B1_03, alt: 9, daisy: None);
flexio!(module: 3, offset: 20, pad: GPIO_B1_04, alt: 9, daisy: None);
flexio!(module: 3, offset: 21, pad: GPIO_B1_05, alt: 9, daisy: None);
flexio!(module: 3, offset: 22, pad: GPIO_B1_06, alt: 9, daisy: None);
flexio!(module: 3, offset: 23, pad: GPIO_B1_07, alt: 9, daisy: None);
flexio!(module: 3, offset: 24, pad: GPIO_B1_08, alt: 9, daisy: None);
flexio!(module: 3, offset: 25, pad: GPIO_B1_09, alt: 9, daisy: None);
flexio!(module: 3, offset: 26, pad: GPIO_B1_10, alt: 9, daisy: None);
flexio!(module: 3, offset: 27, pad: GPIO_B1_11, alt: 9, daisy: None);
flexio!(module: 3, offset: 28, pad: GPIO_B1_12, alt: 9, daisy: None);
flexio!(module: 3, offset: 29, pad: GPIO_B1_13, alt: 9, daisy: None);
flexio!(module: 3, offset: 30, pad: GPIO_B1_14, alt: 9, daisy: None);
flexio!(module: 3, offset: 31, pad: GPIO_B1_15, alt: 9, daisy: None);

// FlexIO on this chip does not have any daisy values.
