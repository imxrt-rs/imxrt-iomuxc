//! PWM implementation.
//!
//! To conform with the imxrt-ral instance number,
//! we use "0" instead of "1."

use super::pads::{gpio::*, gpio_ad::*, gpio_sd::*};
use crate::{
    consts::*,
    flexpwm::{Pin, A, B},
};

pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_SD_02, output: A);
pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_02, output: A);
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_SD_04, output: A);
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_04, output: A);
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_AD_04, output: A);
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_06, output: A);
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_AD_06, output: A);
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_08, output: A);

pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_SD_01, output: B);
pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_01, output: B);
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_SD_03, output: B);
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_03, output: B);
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_AD_03, output: B);
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_05, output: B);
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_AD_05, output: B);
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_07, output: B);
