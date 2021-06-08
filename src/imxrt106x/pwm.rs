//! PWM implementation

use super::pads::{ad_b0::*, b0::*, b1::*, emc::*, sd_b0::*};
use crate::{
    consts::*,
    pwm::{Pin, A, B},
};

pwm!(module: U1, submodule: U0, alt: 1, pad: SD_B0_00, output: A);
pwm!(module: U1, submodule: U0, alt: 1, pad: SD_B0_01, output: B);
pwm!(module: U1, submodule: U3, alt: 1, pad: AD_B0_10, output: A);
pwm!(module: U1, submodule: U3, alt: 1, pad: AD_B0_11, output: B);
pwm!(module: U2, submodule: U2, alt: 2, pad: B0_10, output: A);
pwm!(module: U2, submodule: U2, alt: 2, pad: B0_11, output: B);
pwm!(module: U1, submodule: U3, alt: 6, pad: B1_01, output: B);
pwm!(module: U1, submodule: U3, alt: 6, pad: B1_00, output: A);
pwm!(module: U4, submodule: U2, alt: 1, pad: EMC_04, output: A);
pwm!(module: U4, submodule: U2, alt: 1, pad: EMC_05, output: B);
pwm!(module: U2, submodule: U0, alt: 1, pad: EMC_06, output: A);
pwm!(module: U2, submodule: U1, alt: 1, pad: EMC_08, output: A);
