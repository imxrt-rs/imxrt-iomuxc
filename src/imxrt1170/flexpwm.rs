//! PWM implementation.

use super::pads::gpio_ad::*;
use crate::{
    consts::*,
    flexpwm::{Pin, A, B},
    Daisy,
};

pwm!(module: U1, submodule: U2, alt: 4, pad: GPIO_AD_04, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA2_GPIO_AD_04));
pwm!(module: U1, submodule: U2, alt: 4, pad: GPIO_AD_05, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB2_GPIO_AD_05));
pwm!(module: U2, submodule: U2, alt: 4, pad: GPIO_AD_28, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA2_GPIO_AD_28));
pwm!(module: U2, submodule: U2, alt: 4, pad: GPIO_AD_29, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB2_GPIO_AD_29));

mod daisy {
    #![allow(unused)]

    use super::Daisy;
    pub const DAISY_FLEXPWM1_PWMA0_GPIO_EMC_B1_23: Daisy = Daisy::new(0x400e8500 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMA0_GPIO_AD_00: Daisy = Daisy::new(0x400e8500 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMA1_GPIO_EMC_B1_25: Daisy = Daisy::new(0x400e8504 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMA1_GPIO_AD_02: Daisy = Daisy::new(0x400e8504 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMA2_GPIO_EMC_B1_27: Daisy = Daisy::new(0x400e8508 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMA2_GPIO_AD_04: Daisy = Daisy::new(0x400e8508 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMB0_GPIO_EMC_B1_24: Daisy = Daisy::new(0x400e850c as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMB0_GPIO_AD_01: Daisy = Daisy::new(0x400e850c as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMB1_GPIO_EMC_B1_26: Daisy = Daisy::new(0x400e8510 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMB1_GPIO_AD_03: Daisy = Daisy::new(0x400e8510 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMB2_GPIO_EMC_B1_28: Daisy = Daisy::new(0x400e8514 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMB2_GPIO_AD_05: Daisy = Daisy::new(0x400e8514 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMA0_GPIO_EMC_B1_06: Daisy = Daisy::new(0x400e8518 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMA0_GPIO_AD_24: Daisy = Daisy::new(0x400e8518 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMA1_GPIO_EMC_B1_08: Daisy = Daisy::new(0x400e851c as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMA1_GPIO_AD_26: Daisy = Daisy::new(0x400e851c as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMA2_GPIO_EMC_B1_10: Daisy = Daisy::new(0x400e8520 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMA2_GPIO_AD_28: Daisy = Daisy::new(0x400e8520 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMB0_GPIO_EMC_B1_07: Daisy = Daisy::new(0x400e8524 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMB0_GPIO_AD_25: Daisy = Daisy::new(0x400e8524 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMB1_GPIO_EMC_B1_09: Daisy = Daisy::new(0x400e8528 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMB1_GPIO_AD_27: Daisy = Daisy::new(0x400e8528 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMB2_GPIO_EMC_B1_11: Daisy = Daisy::new(0x400e852c as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMB2_GPIO_AD_29: Daisy = Daisy::new(0x400e852c as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMA0_GPIO_EMC_B1_29: Daisy = Daisy::new(0x400e8530 as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMA0_GPIO_EMC_B2_00_: Daisy = Daisy::new(0x400e8530 as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMA1_GPIO_EMC_B1_31: Daisy = Daisy::new(0x400e8534 as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMA1_GPIO_EMC_B2_02_: Daisy = Daisy::new(0x400e8534 as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMA2_GPIO_EMC_B1_33: Daisy = Daisy::new(0x400e8538 as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMA2_GPIO_EMC_B2_04_: Daisy = Daisy::new(0x400e8538 as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMA3_GPIO_EMC_B1_21: Daisy = Daisy::new(0x400e853c as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMA3_GPIO_EMC_B2_06_: Daisy = Daisy::new(0x400e853c as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMB0_GPIO_EMC_B1_30: Daisy = Daisy::new(0x400e8540 as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMB0_GPIO_EMC_B2_01_: Daisy = Daisy::new(0x400e8540 as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMB1_GPIO_EMC_B1_32: Daisy = Daisy::new(0x400e8544 as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMB1_GPIO_EMC_B2_03_: Daisy = Daisy::new(0x400e8544 as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMB2_GPIO_EMC_B1_34: Daisy = Daisy::new(0x400e8548 as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMB2_GPIO_EMC_B2_05_: Daisy = Daisy::new(0x400e8548 as *mut u32, 1);
    pub const DAISY_FLEXPWM3_PWMB3_GPIO_EMC_B1_22: Daisy = Daisy::new(0x400e854c as *mut u32, 0);
    pub const DAISY_FLEXPWM3_PWMB3_GPIO_EMC_B2_07_: Daisy = Daisy::new(0x400e854c as *mut u32, 1);
}

#[allow(unused)]
use daisy::*;
