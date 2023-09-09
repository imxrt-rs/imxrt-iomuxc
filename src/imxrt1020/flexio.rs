//! FlexIO pin configurations
//!
//! To conform with the imxrt-ral instance number,
//! we use "0" instead of "1."

use super::pads::{gpio_ad_b1::*, gpio_emc::*};
use crate::{flexio::Pin, Daisy};

//
// FLEXIO1
//
flexio!(module: 0, offset: 0, pad: GPIO_AD_B1_15, alt: 4, daisy: None);
flexio!(module: 0, offset: 1, pad: GPIO_AD_B1_14, alt: 4, daisy: None);
flexio!(module: 0, offset: 2, pad: GPIO_AD_B1_13, alt: 4, daisy: None);
flexio!(module: 0, offset: 3, pad: GPIO_AD_B1_12, alt: 4, daisy: None);
flexio!(module: 0, offset: 4, pad: GPIO_AD_B1_11, alt: 4, daisy: None);
flexio!(module: 0, offset: 5, pad: GPIO_AD_B1_10, alt: 4, daisy: None);
flexio!(module: 0, offset: 6, pad: GPIO_AD_B1_09, alt: 4, daisy: None);
flexio!(module: 0, offset: 7, pad: GPIO_AD_B1_08, alt: 4, daisy: None);
flexio!(module: 0, offset: 8, pad: GPIO_AD_B1_07, alt: 4, daisy: None);
flexio!(module: 0, offset: 9, pad: GPIO_AD_B1_06, alt: 4, daisy: None);
flexio!(module: 0, offset: 10, pad: GPIO_AD_B1_05, alt: 4, daisy: None);
flexio!(module: 0, offset: 11, pad: GPIO_AD_B1_04, alt: 4, daisy: None);
flexio!(module: 0, offset: 12, pad: GPIO_AD_B1_03, alt: 4, daisy: None);
flexio!(module: 0, offset: 13, pad: GPIO_AD_B1_02, alt: 4, daisy: None);
flexio!(module: 0, offset: 14, pad: GPIO_AD_B1_01, alt: 4, daisy: None);
flexio!(module: 0, offset: 15, pad: GPIO_AD_B1_00, alt: 4, daisy: None);
flexio!(module: 0, offset: 16, pad: GPIO_EMC_04, alt: 4, daisy: None);
flexio!(module: 0, offset: 17, pad: GPIO_EMC_05, alt: 4, daisy: None);
flexio!(module: 0, offset: 18, pad: GPIO_EMC_06, alt: 4, daisy: None);
flexio!(module: 0, offset: 19, pad: GPIO_EMC_07, alt: 4, daisy: None);
flexio!(module: 0, offset: 20, pad: GPIO_EMC_08, alt: 4, daisy: None);
flexio!(module: 0, offset: 21, pad: GPIO_EMC_09, alt: 4, daisy: None);
flexio!(module: 0, offset: 22, pad: GPIO_EMC_18, alt: 4, daisy: None);
flexio!(module: 0, offset: 23, pad: GPIO_EMC_19, alt: 4, daisy: None);
flexio!(module: 0, offset: 24, pad: GPIO_EMC_20, alt: 4, daisy: None);
flexio!(module: 0, offset: 25, pad: GPIO_EMC_21, alt: 4, daisy: None);
flexio!(module: 0, offset: 26, pad: GPIO_EMC_22, alt: 4, daisy: None);
flexio!(module: 0, offset: 27, pad: GPIO_EMC_23, alt: 4, daisy: None);
flexio!(module: 0, offset: 28, pad: GPIO_EMC_24, alt: 4, daisy: None);
flexio!(module: 0, offset: 29, pad: GPIO_EMC_25, alt: 4, daisy: None);
flexio!(module: 0, offset: 30, pad: GPIO_EMC_26, alt: 4, daisy: None);
flexio!(module: 0, offset: 31, pad: GPIO_EMC_27, alt: 4, daisy: None);

// FlexIO on this chip does not have any daisy values.
