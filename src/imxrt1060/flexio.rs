//! FlexIO pin implementation
//!
//! Most implementations are exposed through the 1050/1060 common module.
//! The 1060 has a third peripheral, which we support here.

use super::pads::{gpio_ad_b1::*, gpio_b1::*};

use crate::{flexio::Pin, Daisy};

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
