//! ADC pin implementations
//!
//! Implementation derived from Table 66-2: ADC External Signals
//! from the iMXRT1060 Reference Manual, Rev 2. There is a similar
//! information available in Table 10-1: Muxing Options, in the IOMUXC
//! section of the reference manual.

use super::{ad_b0::*, ad_b1::*};
use crate::{
    adc::{Pin, ADC1, ADC2},
    consts::*,
};

//
// ADC1
//
adc!(module: ADC1, pad: AD_B1_11, input: U0);
adc!(module: ADC1, pad: AD_B0_12, input: U1);
adc!(module: ADC1, pad: AD_B0_13, input: U2);
adc!(module: ADC1, pad: AD_B0_14, input: U3);
adc!(module: ADC1, pad: AD_B0_15, input: U4);
adc!(module: ADC1, pad: AD_B1_00, input: U5);
adc!(module: ADC1, pad: AD_B1_01, input: U6);
adc!(module: ADC1, pad: AD_B1_02, input: U7);
adc!(module: ADC1, pad: AD_B1_03, input: U8);
adc!(module: ADC1, pad: AD_B1_04, input: U9);
adc!(module: ADC1, pad: AD_B1_05, input: U10);
adc!(module: ADC1, pad: AD_B1_06, input: U11);
adc!(module: ADC1, pad: AD_B1_07, input: U12);
adc!(module: ADC1, pad: AD_B1_08, input: U13);
adc!(module: ADC1, pad: AD_B1_09, input: U14);
adc!(module: ADC1, pad: AD_B1_10, input: U15);

//
// ADC2
//
adc!(module: ADC2, pad: AD_B1_11, input: U0);
adc!(module: ADC2, pad: AD_B1_12, input: U1);
adc!(module: ADC2, pad: AD_B1_13, input: U2);
adc!(module: ADC2, pad: AD_B1_14, input: U3);
adc!(module: ADC2, pad: AD_B1_15, input: U4);
adc!(module: ADC2, pad: AD_B1_00, input: U5);
adc!(module: ADC2, pad: AD_B1_01, input: U6);
adc!(module: ADC2, pad: AD_B1_02, input: U7);
adc!(module: ADC2, pad: AD_B1_03, input: U8);
adc!(module: ADC2, pad: AD_B1_04, input: U9);
adc!(module: ADC2, pad: AD_B1_05, input: U10);
adc!(module: ADC2, pad: AD_B1_06, input: U11);
adc!(module: ADC2, pad: AD_B1_07, input: U12);
adc!(module: ADC2, pad: AD_B1_08, input: U13);
adc!(module: ADC2, pad: AD_B1_09, input: U14);
adc!(module: ADC2, pad: AD_B1_10, input: U15);
