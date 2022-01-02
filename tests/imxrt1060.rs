//! Smoke tests for 1060 pads.

#![cfg(feature = "imxrt1060")]

#[macro_use]
mod macros;

use imxrt_iomuxc::imxrt1060 as pads;

group!(gpio_emc, 42,
         [GPIO_EMC_00, GPIO_EMC_41],
    mux: [0x401F_8014, 0x401F_80B8],
    pad: [0x401F_8204, 0x401F_82A8]);
group!(gpio_ad_b0, 16,
         [GPIO_AD_B0_00, GPIO_AD_B0_15],
    mux: [0x401F_80BC, 0x401F_80F8],
    pad: [0x401F_82AC, 0x401F_82E8]);
group!(gpio_ad_b1, 16,
         [GPIO_AD_B1_00, GPIO_AD_B1_15],
    mux: [0x401F_80FC, 0x401F_8138],
    pad: [0x401F_82EC, 0x401F_8328]);
group!(gpio_b0, 16,
         [GPIO_B0_00, GPIO_B0_15],
    mux: [0x401F_813C, 0x401F_8178],
    pad: [0x401F_832C, 0x401F_8368]);
group!(gpio_b1, 16,
         [GPIO_B1_00, GPIO_B1_15],
    mux: [0x401F_817C, 0x401F_81B8],
    pad: [0x401F_836C, 0x401F_83A8]);
group!(gpio_sd_b0, 06,
         [GPIO_SD_B0_00, GPIO_SD_B0_05],
    mux: [0x401F_81BC, 0x401F_81D0],
    pad: [0x401F_83AC, 0x401F_83C0]);
group!(gpio_sd_b1, 12,
         [GPIO_SD_B1_00, GPIO_SD_B1_11],
    mux: [0x401F_81D4, 0x401F_8200],
    pad: [0x401F_83C4, 0x401F_83F0]);
