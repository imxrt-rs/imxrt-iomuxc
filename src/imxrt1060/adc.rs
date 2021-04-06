//! ADC pin implementations
//!
//! Implementation derived from Table 66-2: ADC External Signals
//! from the iMXRT1060 Reference Manual, Rev 2. There is a similar
//! information available in Table 10-1: Muxing Options, in the IOMUXC
//! section of the reference manual.

use super::{ad_b0::*, ad_b1::*};
use crate::adc::{Pin, ADC1, ADC2};

//
// ADC1
//

impl Pin<ADC1> for AD_B1_11 {
    const INPUT: u32 = 0;
}

impl Pin<ADC1> for AD_B0_12 {
    const INPUT: u32 = 1;
}

impl Pin<ADC1> for AD_B0_13 {
    const INPUT: u32 = 2;
}

impl Pin<ADC1> for AD_B0_14 {
    const INPUT: u32 = 3;
}

impl Pin<ADC1> for AD_B0_15 {
    const INPUT: u32 = 4;
}

impl Pin<ADC1> for AD_B1_00 {
    const INPUT: u32 = 5;
}

impl Pin<ADC1> for AD_B1_01 {
    const INPUT: u32 = 6;
}

impl Pin<ADC1> for AD_B1_02 {
    const INPUT: u32 = 7;
}

impl Pin<ADC1> for AD_B1_03 {
    const INPUT: u32 = 8;
}

impl Pin<ADC1> for AD_B1_04 {
    const INPUT: u32 = 9;
}

impl Pin<ADC1> for AD_B1_05 {
    const INPUT: u32 = 10;
}

impl Pin<ADC1> for AD_B1_06 {
    const INPUT: u32 = 11;
}

impl Pin<ADC1> for AD_B1_07 {
    const INPUT: u32 = 12;
}

impl Pin<ADC1> for AD_B1_08 {
    const INPUT: u32 = 13;
}

impl Pin<ADC1> for AD_B1_09 {
    const INPUT: u32 = 14;
}

impl Pin<ADC1> for AD_B1_10 {
    const INPUT: u32 = 15;
}

//
// ADC2
//

impl Pin<ADC2> for AD_B1_11 {
    const INPUT: u32 = 0;
}

impl Pin<ADC2> for AD_B1_12 {
    const INPUT: u32 = 1;
}

impl Pin<ADC2> for AD_B1_13 {
    const INPUT: u32 = 2;
}

impl Pin<ADC2> for AD_B1_14 {
    const INPUT: u32 = 3;
}

impl Pin<ADC2> for AD_B1_15 {
    const INPUT: u32 = 4;
}

impl Pin<ADC2> for AD_B1_00 {
    const INPUT: u32 = 5;
}

impl Pin<ADC2> for AD_B1_01 {
    const INPUT: u32 = 6;
}

impl Pin<ADC2> for AD_B1_02 {
    const INPUT: u32 = 7;
}

impl Pin<ADC2> for AD_B1_03 {
    const INPUT: u32 = 8;
}

impl Pin<ADC2> for AD_B1_04 {
    const INPUT: u32 = 9;
}

impl Pin<ADC2> for AD_B1_05 {
    const INPUT: u32 = 10;
}

impl Pin<ADC2> for AD_B1_06 {
    const INPUT: u32 = 11;
}

impl Pin<ADC2> for AD_B1_07 {
    const INPUT: u32 = 12;
}

impl Pin<ADC2> for AD_B1_08 {
    const INPUT: u32 = 13;
}

impl Pin<ADC2> for AD_B1_09 {
    const INPUT: u32 = 14;
}

impl Pin<ADC2> for AD_B1_10 {
    const INPUT: u32 = 15;
}
