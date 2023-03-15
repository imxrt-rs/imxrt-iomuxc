//! PWM implementation.
//!
//! To conform with the imxrt-ral instance number,
//! we use "0" instead of "1."

use super::pads::{gpio::*, gpio_ad::*, gpio_sd::*};
use crate::{
    consts::*,
    flexpwm::{Pin, A, B},
    Daisy,
};

pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_SD_02, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA0_GPIO_SD_02));
pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_02, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA0_GPIO_02));
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_SD_04, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA1_GPIO_SD_04));
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_04, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA1_GPIO_04));
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_AD_04, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA2_GPIO_AD_04));
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_06, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA2_GPIO_06));
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_AD_06, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA3_GPIO_AD_06));
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_08, output: A, daisy: Some(DAISY_FLEXPWM0_PWMA3_GPIO_08));

pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_SD_01, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB0_GPIO_SD_01));
pwm!(module: U0, submodule: U0, alt: 2, pad: GPIO_01, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB0_GPIO_01));
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_SD_03, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB1_GPIO_SD_03));
pwm!(module: U0, submodule: U1, alt: 2, pad: GPIO_03, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB1_GPIO_03));
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_AD_03, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB2_GPIO_AD_03));
pwm!(module: U0, submodule: U2, alt: 2, pad: GPIO_05, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB2_GPIO_05));
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_AD_05, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB3_GPIO_AD_05));
pwm!(module: U0, submodule: U3, alt: 2, pad: GPIO_07, output: B, daisy: Some(DAISY_FLEXPWM0_PWMB3_GPIO_07));

mod daisy {
    use super::Daisy;

    pub const DAISY_FLEXPWM0_PWMA0_GPIO_SD_02: Daisy = Daisy::new(0x401f8174 as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMA0_GPIO_02: Daisy = Daisy::new(0x401f8174 as *mut u32, 1);
    pub const DAISY_FLEXPWM0_PWMA1_GPIO_SD_04: Daisy = Daisy::new(0x401f8178 as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMA1_GPIO_04: Daisy = Daisy::new(0x401f8178 as *mut u32, 1);
    pub const DAISY_FLEXPWM0_PWMA2_GPIO_AD_04: Daisy = Daisy::new(0x401f817c as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMA2_GPIO_06: Daisy = Daisy::new(0x401f817c as *mut u32, 1);
    pub const DAISY_FLEXPWM0_PWMA3_GPIO_AD_06: Daisy = Daisy::new(0x401f8180 as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMA3_GPIO_08: Daisy = Daisy::new(0x401f8180 as *mut u32, 1);
    pub const DAISY_FLEXPWM0_PWMB0_GPIO_SD_01: Daisy = Daisy::new(0x401f8184 as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMB0_GPIO_01: Daisy = Daisy::new(0x401f8184 as *mut u32, 1);
    pub const DAISY_FLEXPWM0_PWMB1_GPIO_SD_03: Daisy = Daisy::new(0x401f8188 as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMB1_GPIO_03: Daisy = Daisy::new(0x401f8188 as *mut u32, 1);
    pub const DAISY_FLEXPWM0_PWMB2_GPIO_AD_03: Daisy = Daisy::new(0x401f818c as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMB2_GPIO_05: Daisy = Daisy::new(0x401f818c as *mut u32, 1);
    pub const DAISY_FLEXPWM0_PWMB3_GPIO_AD_05: Daisy = Daisy::new(0x401f8190 as *mut u32, 0);
    pub const DAISY_FLEXPWM0_PWMB3_GPIO_07: Daisy = Daisy::new(0x401f8190 as *mut u32, 1);
}

use daisy::*;
