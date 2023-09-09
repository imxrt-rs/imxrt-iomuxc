//! FlexIO pin configurations
//!
//! To conform with the imxrt-ral instance number,
//! we use "0" instead of "1."

use super::pads::{gpio::*, gpio_ad::*, gpio_sd::*};
use crate::{flexio::Pin, Daisy};

//
// FLEXIO1
//
flexio!(module: 0, offset: 0, pad: GPIO_08, alt: 4, daisy: None);
flexio!(module: 0, offset: 1, pad: GPIO_09, alt: 4, daisy: None);
flexio!(module: 0, offset: 2, pad: GPIO_10, alt: 4, daisy: None);
flexio!(module: 0, offset: 3, pad: GPIO_11, alt: 4, daisy: None);
flexio!(module: 0, offset: 4, pad: GPIO_12, alt: 4, daisy: None);
flexio!(module: 0, offset: 5, pad: GPIO_13, alt: 4, daisy: None);
flexio!(module: 0, offset: 6, pad: GPIO_SD_00, alt: 4, daisy: None);
flexio!(module: 0, offset: 7, pad: GPIO_SD_01, alt: 4, daisy: None);
flexio!(module: 0, offset: 8, pad: GPIO_SD_02, alt: 4, daisy: None);
flexio!(module: 0, offset: 9, pad: GPIO_SD_03, alt: 4, daisy: None);
flexio!(module: 0, offset: 10, pad: GPIO_SD_04, alt: 4, daisy: None);
flexio!(module: 0, offset: 11, pad: GPIO_SD_05, alt: 4, daisy: None);
flexio!(module: 0, offset: 12, pad: GPIO_SD_06, alt: 4, daisy: None);
flexio!(module: 0, offset: 13, pad: GPIO_SD_07, alt: 4, daisy: None);
flexio!(module: 0, offset: 14, pad: GPIO_SD_08, alt: 4, daisy: None);
flexio!(module: 0, offset: 15, pad: GPIO_SD_09, alt: 4, daisy: None);
flexio!(module: 0, offset: 16, pad: GPIO_SD_10, alt: 4, daisy: None);
flexio!(module: 0, offset: 17, pad: GPIO_SD_11, alt: 4, daisy: None);
flexio!(module: 0, offset: 18, pad: GPIO_SD_12, alt: 4, daisy: None);
flexio!(module: 0, offset: 19, pad: GPIO_SD_13, alt: 4, daisy: None);
flexio!(module: 0, offset: 20, pad: GPIO_AD_00, alt: 4, daisy: None);
flexio!(module: 0, offset: 21, pad: GPIO_AD_09, alt: 4, daisy: None);
flexio!(module: 0, offset: 22, pad: GPIO_AD_10, alt: 4, daisy: None);
flexio!(module: 0, offset: 23, pad: GPIO_AD_11, alt: 4, daisy: None);
flexio!(module: 0, offset: 24, pad: GPIO_AD_12, alt: 4, daisy: None);
flexio!(module: 0, offset: 25, pad: GPIO_AD_13, alt: 4, daisy: None);
flexio!(module: 0, offset: 26, pad: GPIO_AD_14, alt: 4, daisy: None);

// FlexIO on this chip does not have any daisy values.
