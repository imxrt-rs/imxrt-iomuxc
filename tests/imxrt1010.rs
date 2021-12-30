//! Smoke tests for the 1010 pads.

#![cfg(feature = "imxrt1010")]

#[macro_use]
mod macros;

use imxrt_iomuxc::imxrt1010 as pads;

group!(gpio_ad, 15,
         [GPIO_AD_14, GPIO_AD_00],
    mux: [0x401F_8010, 0x401F_8048],
    pad: [0x401F_80C0, 0x401F_80F8]);
group!(gpio_sd, 15,
         [GPIO_SD_14, GPIO_SD_00],
    mux: [0x401F_804C, 0x401F_8084],
    pad: [0x401F_80FC, 0x401F_8134]);
group!(gpio, 14,
         [GPIO_13, GPIO_00],
    mux: [0x401F_8088, 0x401F_80BC],
    pad: [0x401F_8138, 0x401F_816C]);
