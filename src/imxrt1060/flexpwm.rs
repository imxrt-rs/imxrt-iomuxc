//! PWM implementation

use super::pads::{
    gpio_ad_b0::*, gpio_ad_b1::*, gpio_b0::*, gpio_b1::*, gpio_emc::*, gpio_sd_b0::*, gpio_sd_b1::*,
};
use crate::{
    consts::*,
    flexpwm::{Pin, A, B},
    Daisy,
};

pwm!(module: U1, submodule: U0, alt: 1, pad: GPIO_EMC_23, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA0_GPIO_EMC_23));
pwm!(module: U1, submodule: U0, alt: 1, pad: GPIO_SD_B0_00, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA0_GPIO_SD_B0_00));
pwm!(module: U1, submodule: U0, alt: 1, pad: GPIO_SD_B0_01, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB0_GPIO_SD_B0_01));
pwm!(module: U1, submodule: U0, alt: 1, pad: GPIO_EMC_24, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB0_GPIO_EMC_24));
pwm!(module: U1, submodule: U1, alt: 1, pad: GPIO_EMC_25, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA1_GPIO_EMC_25));
pwm!(module: U1, submodule: U1, alt: 1, pad: GPIO_SD_B0_02, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA1_GPIO_SD_B0_02));
pwm!(module: U1, submodule: U1, alt: 1, pad: GPIO_EMC_26, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB1_GPIO_EMC_26));
pwm!(module: U1, submodule: U1, alt: 1, pad: GPIO_SD_B0_03, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB1_GPIO_SD_B0_03));
pwm!(module: U1, submodule: U2, alt: 1, pad: GPIO_SD_B0_04, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA2_GPIO_SD_B0_04));
pwm!(module: U1, submodule: U2, alt: 1, pad: GPIO_EMC_27, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA2_GPIO_EMC_27));
pwm!(module: U1, submodule: U2, alt: 1, pad: GPIO_SD_B0_05, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB2_GPIO_SD_B0_05));
pwm!(module: U1, submodule: U2, alt: 1, pad: GPIO_EMC_28, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB2_GPIO_EMC_28));
pwm!(module: U1, submodule: U3, alt: 1, pad: GPIO_EMC_38, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA3_GPIO_EMC_38));
pwm!(module: U1, submodule: U3, alt: 2, pad: GPIO_SD_B1_00, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA3_GPIO_SD_B1_00));
pwm!(module: U1, submodule: U3, alt: 1, pad: GPIO_AD_B0_10, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA3_GPIO_AD_B0_10));
pwm!(module: U1, submodule: U3, alt: 4, pad: GPIO_EMC_12, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA3_GPIO_EMC_12));
pwm!(module: U1, submodule: U3, alt: 6, pad: GPIO_B1_00, output: A, daisy: Some(DAISY_FLEXPWM1_PWMA3_GPIO_B1_00));
pwm!(module: U1, submodule: U3, alt: 1, pad: GPIO_EMC_39, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB3_GPIO_EMC_39));
pwm!(module: U1, submodule: U3, alt: 6, pad: GPIO_B1_01, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB3_GPIO_B1_01));
pwm!(module: U1, submodule: U3, alt: 1, pad: GPIO_AD_B0_11, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB3_GPIO_AD_B0_11));
pwm!(module: U1, submodule: U3, alt: 2, pad: GPIO_SD_B1_01, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB3_GPIO_SD_B1_01));
pwm!(module: U1, submodule: U3, alt: 4, pad: GPIO_EMC_13, output: B, daisy: Some(DAISY_FLEXPWM1_PWMB3_GPIO_EMC_13));
pwm!(module: U2, submodule: U0, alt: 2, pad: GPIO_B0_06, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA0_GPIO_B0_06));
pwm!(module: U2, submodule: U0, alt: 1, pad: GPIO_EMC_06, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA0_GPIO_EMC_06));
pwm!(module: U2, submodule: U0, alt: 2, pad: GPIO_B0_07, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB0_GPIO_B0_07));
pwm!(module: U2, submodule: U0, alt: 1, pad: GPIO_EMC_07, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB0_GPIO_EMC_07));
pwm!(module: U2, submodule: U1, alt: 1, pad: GPIO_EMC_08, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA1_GPIO_EMC_08));
pwm!(module: U2, submodule: U1, alt: 2, pad: GPIO_B0_08, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA1_GPIO_B0_08));
pwm!(module: U2, submodule: U1, alt: 1, pad: GPIO_EMC_09, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB1_GPIO_EMC_09));
pwm!(module: U2, submodule: U1, alt: 2, pad: GPIO_B0_09, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB1_GPIO_B0_09));
pwm!(module: U2, submodule: U2, alt: 1, pad: GPIO_EMC_10, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA2_GPIO_EMC_10));
pwm!(module: U2, submodule: U2, alt: 2, pad: GPIO_B0_10, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA2_GPIO_B0_10));
pwm!(module: U2, submodule: U2, alt: 1, pad: GPIO_EMC_11, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB2_GPIO_EMC_11));
pwm!(module: U2, submodule: U2, alt: 2, pad: GPIO_B0_11, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB2_GPIO_B0_11));
pwm!(module: U2, submodule: U3, alt: 1, pad: GPIO_AD_B0_09, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA3_GPIO_AD_B0_09));
pwm!(module: U2, submodule: U3, alt: 2, pad: GPIO_SD_B1_02, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA3_GPIO_SD_B1_02));
pwm!(module: U2, submodule: U3, alt: 1, pad: GPIO_EMC_19, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA3_GPIO_EMC_19));
pwm!(module: U2, submodule: U3, alt: 6, pad: GPIO_B1_02, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA3_GPIO_B1_02));
pwm!(module: U2, submodule: U3, alt: 0, pad: GPIO_AD_B0_00, output: A, daisy: Some(DAISY_FLEXPWM2_PWMA3_GPIO_AD_B0_00));
pwm!(module: U2, submodule: U3, alt: 0, pad: GPIO_AD_B0_01, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB3_GPIO_AD_B0_01));
pwm!(module: U2, submodule: U3, alt: 2, pad: GPIO_SD_B1_03, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB3_GPIO_SD_B1_03));
pwm!(module: U2, submodule: U3, alt: 6, pad: GPIO_B1_03, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB3_GPIO_B1_03));
pwm!(module: U2, submodule: U3, alt: 1, pad: GPIO_EMC_20, output: B, daisy: Some(DAISY_FLEXPWM2_PWMB3_GPIO_EMC_20));
pwm!(module: U3, submodule: U0, alt: 1, pad: GPIO_EMC_29, output: A, daisy: None);
pwm!(module: U3, submodule: U0, alt: 1, pad: GPIO_EMC_30, output: B, daisy: None);
pwm!(module: U3, submodule: U1, alt: 1, pad: GPIO_EMC_31, output: A, daisy: None);
pwm!(module: U3, submodule: U1, alt: 1, pad: GPIO_EMC_32, output: B, daisy: None);
pwm!(module: U3, submodule: U2, alt: 1, pad: GPIO_EMC_33, output: A, daisy: None);
pwm!(module: U3, submodule: U2, alt: 1, pad: GPIO_EMC_34, output: B, daisy: None);
pwm!(module: U3, submodule: U3, alt: 1, pad: GPIO_EMC_21, output: A, daisy: None);
pwm!(module: U3, submodule: U3, alt: 1, pad: GPIO_EMC_22, output: B, daisy: None);
pwm!(module: U4, submodule: U0, alt: 1, pad: GPIO_AD_B1_08, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA0_GPIO_AD_B1_08));
pwm!(module: U4, submodule: U0, alt: 1, pad: GPIO_EMC_00, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA0_GPIO_EMC_00));
pwm!(module: U4, submodule: U0, alt: 1, pad: GPIO_EMC_01, output: B, daisy: None);
pwm!(module: U4, submodule: U1, alt: 1, pad: GPIO_AD_B1_09, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA1_GPIO_AD_B1_09));
pwm!(module: U4, submodule: U1, alt: 1, pad: GPIO_EMC_02, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA1_GPIO_EMC_02));
pwm!(module: U4, submodule: U1, alt: 1, pad: GPIO_EMC_03, output: B, daisy: None);
pwm!(module: U4, submodule: U2, alt: 1, pad: GPIO_B1_14, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA2_GPIO_B1_14));
pwm!(module: U4, submodule: U2, alt: 1, pad: GPIO_EMC_04, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA2_GPIO_EMC_04));
pwm!(module: U4, submodule: U2, alt: 1, pad: GPIO_EMC_05, output: B, daisy: None);
pwm!(module: U4, submodule: U3, alt: 1, pad: GPIO_B1_15, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA3_GPIO_B1_15));
pwm!(module: U4, submodule: U3, alt: 1, pad: GPIO_EMC_17, output: A, daisy: Some(DAISY_FLEXPWM4_PWMA3_GPIO_EMC_17));
pwm!(module: U4, submodule: U3, alt: 1, pad: GPIO_EMC_18, output: B, daisy: None);

mod daisy {
    #![allow(unused)]
    use super::Daisy;

    pub const DAISY_FLEXPWM1_PWMA3_GPIO_SD_B1_00: Daisy = Daisy::new(0x401f8454 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMA3_GPIO_EMC_12: Daisy = Daisy::new(0x401f8454 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMA3_GPIO_EMC_38: Daisy = Daisy::new(0x401f8454 as *mut u32, 2);
    pub const DAISY_FLEXPWM1_PWMA3_GPIO_AD_B0_10: Daisy = Daisy::new(0x401f8454 as *mut u32, 3);
    pub const DAISY_FLEXPWM1_PWMA3_GPIO_B1_00: Daisy = Daisy::new(0x401f8454 as *mut u32, 4);
    pub const DAISY_FLEXPWM1_PWMA0_GPIO_EMC_23: Daisy = Daisy::new(0x401f8458 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMA0_GPIO_SD_B0_00: Daisy = Daisy::new(0x401f8458 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMA1_GPIO_EMC_25: Daisy = Daisy::new(0x401f845c as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMA1_GPIO_SD_B0_02: Daisy = Daisy::new(0x401f845c as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMA2_GPIO_EMC_27: Daisy = Daisy::new(0x401f8460 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMA2_GPIO_SD_B0_04: Daisy = Daisy::new(0x401f8460 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMB3_GPIO_SD_B1_01: Daisy = Daisy::new(0x401f8464 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMB3_GPIO_EMC_13: Daisy = Daisy::new(0x401f8464 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMB3_GPIO_EMC_39: Daisy = Daisy::new(0x401f8464 as *mut u32, 2);
    pub const DAISY_FLEXPWM1_PWMB3_GPIO_AD_B0_11: Daisy = Daisy::new(0x401f8464 as *mut u32, 3);
    pub const DAISY_FLEXPWM1_PWMB3_GPIO_B1_01: Daisy = Daisy::new(0x401f8464 as *mut u32, 4);
    pub const DAISY_FLEXPWM1_PWMB0_GPIO_EMC_24: Daisy = Daisy::new(0x401f8468 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMB0_GPIO_SD_B0_01: Daisy = Daisy::new(0x401f8468 as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMB1_GPIO_EMC_26: Daisy = Daisy::new(0x401f846c as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMB1_GPIO_SD_B0_03: Daisy = Daisy::new(0x401f846c as *mut u32, 1);
    pub const DAISY_FLEXPWM1_PWMB2_GPIO_EMC_28: Daisy = Daisy::new(0x401f8470 as *mut u32, 0);
    pub const DAISY_FLEXPWM1_PWMB2_GPIO_SD_B0_05: Daisy = Daisy::new(0x401f8470 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMA3_GPIO_SD_B1_02: Daisy = Daisy::new(0x401f8474 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMA3_GPIO_EMC_19: Daisy = Daisy::new(0x401f8474 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMA3_GPIO_AD_B0_00: Daisy = Daisy::new(0x401f8474 as *mut u32, 2);
    pub const DAISY_FLEXPWM2_PWMA3_GPIO_AD_B0_09: Daisy = Daisy::new(0x401f8474 as *mut u32, 3);
    pub const DAISY_FLEXPWM2_PWMA3_GPIO_B1_02: Daisy = Daisy::new(0x401f8474 as *mut u32, 4);
    pub const DAISY_FLEXPWM2_PWMA0_GPIO_EMC_06: Daisy = Daisy::new(0x401f8478 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMA0_GPIO_B0_06: Daisy = Daisy::new(0x401f8478 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMA1_GPIO_EMC_08: Daisy = Daisy::new(0x401f847c as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMA1_GPIO_B0_08: Daisy = Daisy::new(0x401f847c as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMA2_GPIO_EMC_10: Daisy = Daisy::new(0x401f8480 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMA2_GPIO_B0_10: Daisy = Daisy::new(0x401f8480 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMB3_GPIO_SD_B1_03: Daisy = Daisy::new(0x401f8484 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMB3_GPIO_EMC_20: Daisy = Daisy::new(0x401f8484 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMB3_GPIO_AD_B0_01: Daisy = Daisy::new(0x401f8484 as *mut u32, 2);
    pub const DAISY_FLEXPWM2_PWMB3_GPIO_B1_03: Daisy = Daisy::new(0x401f8484 as *mut u32, 3);
    pub const DAISY_FLEXPWM2_PWMB0_GPIO_EMC_07: Daisy = Daisy::new(0x401f8488 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMB0_GPIO_B0_07: Daisy = Daisy::new(0x401f8488 as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMB1_GPIO_EMC_09: Daisy = Daisy::new(0x401f848c as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMB1_GPIO_B0_09: Daisy = Daisy::new(0x401f848c as *mut u32, 1);
    pub const DAISY_FLEXPWM2_PWMB2_GPIO_EMC_11: Daisy = Daisy::new(0x401f8490 as *mut u32, 0);
    pub const DAISY_FLEXPWM2_PWMB2_GPIO_B0_11: Daisy = Daisy::new(0x401f8490 as *mut u32, 1);
    pub const DAISY_FLEXPWM4_PWMA0_GPIO_EMC_00: Daisy = Daisy::new(0x401f8494 as *mut u32, 0);
    pub const DAISY_FLEXPWM4_PWMA0_GPIO_AD_B1_08: Daisy = Daisy::new(0x401f8494 as *mut u32, 1);
    pub const DAISY_FLEXPWM4_PWMA1_GPIO_EMC_02: Daisy = Daisy::new(0x401f8498 as *mut u32, 0);
    pub const DAISY_FLEXPWM4_PWMA1_GPIO_AD_B1_09: Daisy = Daisy::new(0x401f8498 as *mut u32, 1);
    pub const DAISY_FLEXPWM4_PWMA2_GPIO_EMC_04: Daisy = Daisy::new(0x401f849c as *mut u32, 0);
    pub const DAISY_FLEXPWM4_PWMA2_GPIO_B1_14: Daisy = Daisy::new(0x401f849c as *mut u32, 1);
    pub const DAISY_FLEXPWM4_PWMA3_GPIO_EMC_17: Daisy = Daisy::new(0x401f84a0 as *mut u32, 0);
    pub const DAISY_FLEXPWM4_PWMA3_GPIO_B1_15: Daisy = Daisy::new(0x401f84a0 as *mut u32, 1);
}

#[allow(unused)]
use daisy::*;
