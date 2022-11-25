//! PWM implementation.

use super::pads::gpio_ad::*;
use crate::{
    consts::*,
    flexpwm::{Pin, A, B},
};

pwm!(module: U1, submodule: U2, alt: 4, pad: GPIO_AD_04, output: A);
pwm!(module: U1, submodule: U2, alt: 4, pad: GPIO_AD_05, output: B);
pwm!(module: U2, submodule: U2, alt: 4, pad: GPIO_AD_28, output: A);
pwm!(module: U2, submodule: U2, alt: 4, pad: GPIO_AD_29, output: B);
