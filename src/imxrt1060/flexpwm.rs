//! PWM implementation

use super::pads::{gpio_ad_b0::*, gpio_b0::*, gpio_b1::*, gpio_emc::*, gpio_sd_b0::*};
use crate::{
    consts::*,
    flexpwm::{Pin, A, B},
};

pwm!(module: U1, submodule: U0, alt: 1, pad: GPIO_SD_B0_00, output: A);
pwm!(module: U1, submodule: U0, alt: 1, pad: GPIO_SD_B0_01, output: B);
pwm!(module: U1, submodule: U3, alt: 1, pad: GPIO_AD_B0_10, output: A);
pwm!(module: U1, submodule: U3, alt: 1, pad: GPIO_AD_B0_11, output: B);
pwm!(module: U2, submodule: U2, alt: 2, pad: GPIO_B0_10, output: A);
pwm!(module: U2, submodule: U2, alt: 2, pad: GPIO_B0_11, output: B);
pwm!(module: U1, submodule: U3, alt: 6, pad: GPIO_B1_01, output: B);
pwm!(module: U1, submodule: U3, alt: 6, pad: GPIO_B1_00, output: A);
pwm!(module: U4, submodule: U2, alt: 1, pad: GPIO_EMC_04, output: A);
pwm!(module: U4, submodule: U2, alt: 1, pad: GPIO_EMC_05, output: B);
pwm!(module: U2, submodule: U0, alt: 1, pad: GPIO_EMC_06, output: A);
pwm!(module: U2, submodule: U1, alt: 1, pad: GPIO_EMC_08, output: A);
